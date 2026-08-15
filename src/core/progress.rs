//! # core::progress
//!
//! Persistent playback and reading position store.
//!
//! ## Storage layout
//! Each item's progress is stored as a single JSON file under
//! `<vault>/.mediavault/progress/<key>.json`, where `<key>` is a hex-encoded
//! FNV-1a-64 hash of the vault-relative path string.  Using a path hash keeps
//! file-system names short and filesystem-safe while still being stable as long
//! as the file hasn't moved inside the vault.
//!
//! ## Dependencies
//! - `core::vault::Vault` – directory resolution

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::{Result, VaultError};

// ---------------------------------------------------------------------------
// Progress data types
// ---------------------------------------------------------------------------

/// Playback or reading position for one media item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MediaProgress {
    /// Video file (MP4, MOV, WebM, …).
    Video {
        position_seconds: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration_seconds: Option<f64>,
    },
    /// Audio file (MP3, M4A, FLAC, …) or standalone audio track.
    Audio {
        position_seconds: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration_seconds: Option<f64>,
    },
    /// Audiobook — may span multiple files, position is in the current part.
    Audiobook {
        part_index: u32,
        position_seconds: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        part_hash: Option<String>,
    },
    /// EPUB file — position encoded as an EPUB CFI string.
    Epub {
        /// EPUB Canonical Fragment Identifier, layout-independent.
        cfi: String,
        /// 0.0 – 1.0 approximate progress for display.
        percentage: f64,
    },
    /// PDF file — page number (1-indexed) and fractional scroll within that page.
    Pdf {
        page: u32,
        /// 0.0 = top of page, 1.0 = bottom.
        scroll_fraction: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        total_pages: Option<u32>,
    },
    /// Manga or comic — tracked by page index (0-indexed).
    Manga {
        page: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        total_pages: Option<u32>,
    },
}

impl MediaProgress {
    /// Returns a value in [0.0, 1.0] suitable for a progress bar, if derivable.
    pub fn fraction(&self) -> Option<f64> {
        match self {
            Self::Video {
                position_seconds,
                duration_seconds: Some(dur),
            }
            | Self::Audio {
                position_seconds,
                duration_seconds: Some(dur),
            } => {
                if *dur > 0.0 {
                    Some((position_seconds / dur).clamp(0.0, 1.0))
                } else {
                    None
                }
            }
            Self::Epub { percentage, .. } => Some(percentage.clamp(0.0, 1.0)),
            Self::Pdf {
                page,
                total_pages: Some(total),
                ..
            } => {
                if *total > 0 {
                    Some((*page as f64 / *total as f64).clamp(0.0, 1.0))
                } else {
                    None
                }
            }
            Self::Manga {
                page,
                total_pages: Some(total),
            } => {
                if *total > 0 {
                    Some((*page as f64 / *total as f64).clamp(0.0, 1.0))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns `true` if the item appears to be finished (≥ 90 %).
    pub fn is_completed(&self) -> bool {
        self.fraction().map(|f| f >= 0.90).unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// On-disk record
// ---------------------------------------------------------------------------

/// The full progress record written to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRecord {
    /// Vault-relative path of the media file this record belongs to.
    pub vault_path: String,
    /// Actual playback / reading position.
    pub progress: MediaProgress,
    /// The same position in the interchange shape, see [`MediaPositionDto`].
    ///
    /// Written on every save so a record can be handed to another player
    /// without a converter; ignored on load when `progress` is present, which
    /// keeps this app's own richer variants (audiobook part index) intact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<MediaPositionDto>,
    /// UNIX timestamp of the last access (seconds since epoch).
    pub last_accessed: u64,
    /// Whether the user has explicitly marked this as completed.
    #[serde(default)]
    pub completed: bool,
}

/// Reading/playback position in the shape the Fundus media manager uses.
///
/// ## Why a second representation
/// [`MediaProgress`] is modelled for this app; Fundus stores one flat position
/// row (`position_kind`, `numeric_value`, `position_key`, `total`, …).  Keeping
/// both in the record means neither side needs to know the other's variants to
/// read a position — and nothing has to be migrated here.
///
/// ## Counting
/// `numeric_value` is **1-based** for page-like kinds, matching Fundus.  This
/// app counts pages from zero internally, so every conversion crosses that
/// boundary — the single most likely place for an off-by-one.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MediaPositionDto {
    /// `time`, `page`, `epubCfi` or `imageIndex`.
    pub kind: String,
    /// Seconds for `time`, 1-based index for page-like kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_value: Option<f64>,
    /// Opaque position key — the EPUB CFI, or the file's relative path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Total duration or page count, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// Identifier of the file the position belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Human-readable position, e.g. `Kapitel 3/12 · Seite 7`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Position kind names, as spelled by Fundus.
pub mod position_kind {
    /// Audio/video playback position in seconds.
    pub const TIME: &str = "time";
    /// Page number in a paginated document (PDF).
    pub const PAGE: &str = "page";
    /// EPUB Canonical Fragment Identifier.
    pub const EPUB_CFI: &str = "epubCfi";
    /// Image index in a comic or image sequence.
    pub const IMAGE_INDEX: &str = "imageIndex";
}

impl MediaProgress {
    /// Converts this position into the interchange shape.
    ///
    /// # Parameters
    /// - `vault_path` – Relative path stored as `key`/`file_id`
    /// - `label` – Optional human-readable position for display in other apps
    pub fn to_position(&self, vault_path: &str, label: Option<String>) -> MediaPositionDto {
        let mut dto = MediaPositionDto {
            kind: position_kind::TIME.to_string(),
            numeric_value: None,
            key: Some(vault_path.to_string()),
            total: None,
            file_id: Some(vault_path.to_string()),
            label,
        };

        match self {
            Self::Video {
                position_seconds,
                duration_seconds,
            }
            | Self::Audio {
                position_seconds,
                duration_seconds,
            } => {
                dto.numeric_value = Some(*position_seconds);
                dto.total = duration_seconds.map(|value| value);
            }
            Self::Audiobook {
                position_seconds, ..
            } => {
                dto.numeric_value = Some(*position_seconds);
            }
            Self::Epub { cfi, percentage } => {
                dto.kind = position_kind::EPUB_CFI.to_string();
                // The CFI is the position; the percentage only drives bars.
                dto.key = Some(cfi.clone());
                dto.numeric_value = Some(*percentage);
                dto.total = Some(1.0);
            }
            Self::Pdf {
                page, total_pages, ..
            } => {
                dto.kind = position_kind::PAGE.to_string();
                dto.numeric_value = Some(f64::from(*page));
                dto.total = total_pages.map(f64::from);
            }
            Self::Manga { page, total_pages } => {
                dto.kind = position_kind::IMAGE_INDEX.to_string();
                // Zero-based here, one-based there.
                dto.numeric_value = Some(f64::from(*page) + 1.0);
                dto.total = total_pages.map(f64::from);
            }
        }
        dto
    }

    /// Rebuilds a position from the interchange shape.
    ///
    /// Returns `None` for kinds this app cannot represent.
    pub fn from_position(dto: &MediaPositionDto) -> Option<Self> {
        let value = dto.numeric_value.unwrap_or(0.0);
        match dto.kind.as_str() {
            position_kind::TIME => Some(Self::Video {
                position_seconds: value,
                duration_seconds: dto.total,
            }),
            position_kind::PAGE => Some(Self::Pdf {
                page: value.max(0.0) as u32,
                scroll_fraction: 0.0,
                total_pages: dto.total.map(|total| total.max(0.0) as u32),
            }),
            position_kind::EPUB_CFI => Some(Self::Epub {
                cfi: dto.key.clone().unwrap_or_default(),
                percentage: value.clamp(0.0, 1.0),
            }),
            position_kind::IMAGE_INDEX => Some(Self::Manga {
                // One-based there, zero-based here; a stored 0 means page one.
                page: (value - 1.0).max(0.0) as u32,
                total_pages: dto.total.map(|total| total.max(0.0) as u32),
            }),
            _ => None,
        }
    }
}

impl ProgressRecord {
    /// Creates a new record with the current timestamp.
    pub fn new(vault_path: impl Into<String>, progress: MediaProgress) -> Self {
        Self::with_label(vault_path, progress, None)
    }

    /// Creates a record carrying a human-readable position label.
    ///
    /// The label is only meaningful to other readers of the interchange
    /// position (`Kapitel 3/12 · Seite 7`); this app derives its own display
    /// text from the typed variant.
    pub fn with_label(
        vault_path: impl Into<String>,
        progress: MediaProgress,
        label: Option<String>,
    ) -> Self {
        let vault_path = vault_path.into();
        let position = progress.to_position(&vault_path, label);
        Self {
            vault_path,
            progress,
            position: Some(position),
            last_accessed: unix_now(),
            completed: false,
        }
    }

    /// Returns the approximate progress fraction for UI display.
    pub fn fraction(&self) -> Option<f64> {
        self.progress.fraction()
    }
}

// ---------------------------------------------------------------------------
// Store operations
// ---------------------------------------------------------------------------

/// Derives the progress file path for a given vault-relative path.
///
/// Uses an FNV-1a-64 hash of the path string so the result is filesystem-safe
/// regardless of special characters in the path.
pub fn progress_file_path(progress_dir: &std::path::Path, vault_path: &str) -> PathBuf {
    let key = fnv1a64(vault_path.as_bytes());
    progress_dir.join(format!("{key:016x}.json"))
}

/// Loads the progress record for a vault-relative path, if one exists.
pub fn load_progress(
    progress_dir: &std::path::Path,
    vault_path: &str,
) -> Result<Option<ProgressRecord>> {
    let file_path = progress_file_path(progress_dir, vault_path);

    if !file_path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(&file_path).map_err(VaultError::from)?;
    match serde_json::from_str::<ProgressRecord>(&raw) {
        Ok(record) => Ok(Some(record)),
        // A record written by another player carries only the interchange
        // position and no typed `progress`, so the strict parse fails. Rather
        // than discarding a perfectly good reading position, rebuild from it.
        Err(strict_error) => match rebuild_from_position(&raw, vault_path) {
            Some(record) => Ok(Some(record)),
            None => Err(VaultError::InvalidVaultPath(format!(
                "progress JSON parse error: {strict_error}"
            ))),
        },
    }
}

/// Rebuilds a record from a foreign file that only carries the interchange
/// position.
fn rebuild_from_position(raw: &str, vault_path: &str) -> Option<ProgressRecord> {
    #[derive(Deserialize)]
    struct ForeignRecord {
        position: MediaPositionDto,
        #[serde(default)]
        last_accessed: u64,
        #[serde(default)]
        completed: bool,
    }

    let foreign: ForeignRecord = serde_json::from_str(raw).ok()?;
    let progress = MediaProgress::from_position(&foreign.position)?;
    Some(ProgressRecord {
        vault_path: vault_path.to_string(),
        progress,
        position: Some(foreign.position),
        last_accessed: if foreign.last_accessed == 0 {
            unix_now()
        } else {
            foreign.last_accessed
        },
        completed: foreign.completed,
    })
}

/// Persists a progress record, creating the progress directory if needed.
pub fn save_progress(
    progress_dir: &std::path::Path,
    vault_path: &str,
    progress: MediaProgress,
    completed: bool,
) -> Result<()> {
    save_progress_labeled(progress_dir, vault_path, progress, completed, None)
}

/// Like [`save_progress`], with a human-readable position label for other
/// readers of the interchange position.
pub fn save_progress_labeled(
    progress_dir: &std::path::Path,
    vault_path: &str,
    progress: MediaProgress,
    completed: bool,
    label: Option<String>,
) -> Result<()> {
    fs::create_dir_all(progress_dir).map_err(VaultError::from)?;

    let mut record = ProgressRecord::with_label(vault_path, progress, label);
    record.completed = completed;

    let file_path = progress_file_path(progress_dir, vault_path);
    let json = serde_json::to_string_pretty(&record)
        .map_err(|e| VaultError::InvalidVaultPath(format!("progress JSON serialize error: {e}")))?;

    fs::write(&file_path, json).map_err(VaultError::from)?;
    Ok(())
}

/// Deletes the progress record for a vault-relative path (marks as not started).
pub fn delete_progress(progress_dir: &std::path::Path, vault_path: &str) -> Result<()> {
    let file_path = progress_file_path(progress_dir, vault_path);
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(VaultError::from)?;
    }
    Ok(())
}

/// Returns all progress records, sorted by most recently accessed first.
///
/// Items that cannot be parsed are silently skipped.
pub fn list_in_progress(progress_dir: &std::path::Path) -> Result<Vec<ProgressRecord>> {
    if !progress_dir.exists() {
        return Ok(Vec::new());
    }

    let mut records: Vec<ProgressRecord> = fs::read_dir(progress_dir)
        .map_err(VaultError::from)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? != "json" {
                return None;
            }
            let raw = fs::read_to_string(&path).ok()?;
            let record: ProgressRecord = serde_json::from_str(&raw).ok()?;
            if record.completed {
                return None;
            }
            Some(record)
        })
        .collect();

    records.sort_by_key(|record| std::cmp::Reverse(record.last_accessed));
    Ok(records)
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// FNV-1a 64-bit hash — matches the implementation in `core::duplicate`.
fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut state = OFFSET;
    for &b in bytes {
        state ^= u64::from(b);
        state = state.wrapping_mul(PRIME);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_progress_fraction() {
        let p = MediaProgress::Video {
            position_seconds: 900.0,
            duration_seconds: Some(3600.0),
        };
        assert!((p.fraction().unwrap() - 0.25).abs() < 1e-9);
        assert!(!p.is_completed());
    }

    #[test]
    fn video_progress_completed() {
        let p = MediaProgress::Video {
            position_seconds: 3300.0,
            duration_seconds: Some(3600.0),
        };
        assert!(p.is_completed());
    }

    #[test]
    fn epub_progress_fraction() {
        let p = MediaProgress::Epub {
            cfi: "epubcfi(/6/4!/4)".to_string(),
            percentage: 0.55,
        };
        assert!((p.fraction().unwrap() - 0.55).abs() < 1e-9);
    }

    #[test]
    fn progress_file_path_is_deterministic() {
        use std::path::Path;
        let dir = Path::new("/vault/.mediavault/progress");
        let a = progress_file_path(dir, "Anime/Test.mkv");
        let b = progress_file_path(dir, "Anime/Test.mkv");
        assert_eq!(a, b);
    }

    #[test]
    fn different_paths_get_different_keys() {
        use std::path::Path;
        let dir = Path::new("/vault/.mediavault/progress");
        let a = progress_file_path(dir, "Anime/A.mkv");
        let b = progress_file_path(dir, "Filme/A.mkv");
        assert_ne!(a, b);
    }

    // -----------------------------------------------------------------------
    // Interchange position (Fundus-compatible)
    // -----------------------------------------------------------------------

    #[test]
    fn manga_positions_cross_the_one_based_boundary() {
        // Fundus counts images from one, this app from zero. Getting this
        // wrong shifts every restored reading position by a page.
        let progress = MediaProgress::Manga {
            page: 0,
            total_pages: Some(50),
        };
        let dto = progress.to_position("Manga/Serie/Kapitel 0001.cbz", None);

        assert_eq!(dto.kind, position_kind::IMAGE_INDEX);
        assert_eq!(dto.numeric_value, Some(1.0));
        assert_eq!(dto.total, Some(50.0));
        assert_eq!(dto.key.as_deref(), Some("Manga/Serie/Kapitel 0001.cbz"));

        assert_eq!(MediaProgress::from_position(&dto), Some(progress));
    }

    #[test]
    fn foreign_first_page_never_underflows() {
        // A writer that counts from zero would send 0; subtracting one must
        // not wrap around on an unsigned page number.
        let dto = MediaPositionDto {
            kind: position_kind::IMAGE_INDEX.to_string(),
            numeric_value: Some(0.0),
            key: None,
            total: Some(12.0),
            file_id: None,
            label: None,
        };
        assert_eq!(
            MediaProgress::from_position(&dto),
            Some(MediaProgress::Manga {
                page: 0,
                total_pages: Some(12)
            })
        );
    }

    #[test]
    fn every_kind_round_trips_through_the_interchange_shape() {
        let cases = [
            (
                MediaProgress::Video {
                    position_seconds: 42.5,
                    duration_seconds: Some(3600.0),
                },
                position_kind::TIME,
            ),
            (
                MediaProgress::Pdf {
                    page: 7,
                    scroll_fraction: 0.0,
                    total_pages: Some(120),
                },
                position_kind::PAGE,
            ),
            (
                MediaProgress::Epub {
                    cfi: "epubcfi(/6/4!/4/2)".to_string(),
                    percentage: 0.25,
                },
                position_kind::EPUB_CFI,
            ),
            (
                MediaProgress::Manga {
                    page: 13,
                    total_pages: Some(20),
                },
                position_kind::IMAGE_INDEX,
            ),
        ];

        for (progress, expected_kind) in cases {
            let dto = progress.to_position("Pfad/Datei", None);
            assert_eq!(dto.kind, expected_kind);
            assert_eq!(
                MediaProgress::from_position(&dto),
                Some(progress),
                "round trip failed for {expected_kind}"
            );
        }
    }

    #[test]
    fn saved_records_carry_the_interchange_position() {
        let dir = std::env::temp_dir().join(format!("progress-dto-{}", std::process::id()));
        save_progress_labeled(
            &dir,
            "Manga/Serie/Kapitel 0003.cbz",
            MediaProgress::Manga {
                page: 6,
                total_pages: Some(30),
            },
            false,
            Some("Kapitel 3/12 · Seite 7".to_string()),
        )
        .expect("save should succeed");

        let record = load_progress(&dir, "Manga/Serie/Kapitel 0003.cbz")
            .expect("load should succeed")
            .expect("record should exist");
        let position = record.position.expect("interchange position is written");
        assert_eq!(position.numeric_value, Some(7.0));
        assert_eq!(position.label.as_deref(), Some("Kapitel 3/12 · Seite 7"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_foreign_record_without_a_typed_progress_is_still_readable() {
        let dir = std::env::temp_dir().join(format!("progress-foreign-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let path = progress_file_path(&dir, "Manga/Serie/Kapitel 0002.cbz");
        // Shaped like a record another player would write: position only.
        std::fs::write(
            &path,
            r#"{"position":{"kind":"imageIndex","numeric_value":5,"total":40},
                "last_accessed":1700000000,"completed":false}"#,
        )
        .expect("write");

        let record = load_progress(&dir, "Manga/Serie/Kapitel 0002.cbz")
            .expect("load should succeed")
            .expect("record should exist");
        assert_eq!(
            record.progress,
            MediaProgress::Manga {
                page: 4,
                total_pages: Some(40)
            }
        );

        std::fs::remove_dir_all(&dir).ok();
    }
}
