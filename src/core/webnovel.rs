//! # core::webnovel
//!
//! Persistent webnovel subscription store.
//!
//! ## Storage layout
//! Each subscription is stored as a single JSON file under
//! `<vault>/.mediavault/webnovels/<key>.json`, where `<key>` is a hex-encoded
//! FNV-1a-64 hash of the normalized subscription URL.  Hashing keeps the file
//! name short and filesystem-safe regardless of URL characters.
//!
//! ## Chapter identity rules
//! A chapter is identified by its **URL**, never by its title (titles are
//! routinely edited upstream).  Chapter `index` reflects the table-of-contents
//! order at the time the chapter was first seen.  If a known URL later
//! disappears from the ToC, the local record is kept — downloaded content is
//! never discarded because of upstream changes.  Newly appearing chapters are
//! appended with fresh indices.
//!
//! ## Dependencies
//! - `core::vault::Vault` – directory resolution (callers pass `system_dir()`)

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::{Result, VaultError};

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// A subscribed webnovel and its chapter bookkeeping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Stable identifier — FNV-1a-64 hex of the normalized URL.
    pub id: String,
    /// Normalized overview/ToC page URL as subscribed.
    pub url: String,
    /// Source adapter id: `royalroad`, `wordpress`, `generic`, `novelupdates`.
    pub source: String,
    /// Novel title as reported by the source.
    pub title: String,
    /// Author, if the source exposes one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Cover image URL, if the source exposes one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    /// Synopsis/description, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Marked as finished — suppresses batch EPUBs and periodic checks.
    #[serde(default)]
    pub completed: bool,
    /// Paused subscriptions are skipped by checks but keep their data.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// All chapters ever seen in the ToC, in first-seen order.
    #[serde(default)]
    pub known_chapters: Vec<KnownChapter>,
    /// UNIX timestamp of the last completed update check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_check_unix: Option<u64>,
    /// Human-readable error from the last failed check, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    /// UNIX timestamp of subscription creation.
    pub created_at_unix: u64,
}

/// One chapter as tracked by a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownChapter {
    /// 1-based position in ToC order at the time the chapter was first seen.
    pub index: u32,
    /// Chapter title as shown in the ToC.
    pub title: String,
    /// Chapter URL — the identity key for deduplication.
    pub url: String,
    /// Set once the chapter content is cached locally; `None` means the
    /// chapter was seen in the ToC but not downloaded yet (resume support).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloaded_at_unix: Option<u64>,
}

impl Subscription {
    /// Creates a new subscription for a normalized URL.
    pub fn new(
        url: impl Into<String>,
        source: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        let url = url.into();
        Self {
            id: subscription_id(&url),
            url,
            source: source.into(),
            title: title.into(),
            author: None,
            cover_url: None,
            description: None,
            completed: false,
            enabled: true,
            known_chapters: Vec::new(),
            last_check_unix: None,
            last_error: None,
            created_at_unix: unix_now(),
        }
    }

    /// Number of chapters whose content is cached locally.
    pub fn downloaded_count(&self) -> usize {
        self.known_chapters
            .iter()
            .filter(|chapter| chapter.downloaded_at_unix.is_some())
            .count()
    }
}

/// Normalizes a subscription URL so equivalent inputs map to one identity.
///
/// Strips the fragment and trailing slashes and trims whitespace.  Scheme and
/// host casing are left to the caller's input; in practice pasted URLs are
/// already lowercase there.
pub fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();
    let without_fragment = match trimmed.split_once('#') {
        Some((base, _fragment)) => base,
        None => trimmed,
    };
    without_fragment.trim_end_matches('/').to_string()
}

/// Derives the stable subscription id for a normalized URL.
pub fn subscription_id(url: &str) -> String {
    format!("{:016x}", fnv1a64(normalize_url(url).as_bytes()))
}

// ---------------------------------------------------------------------------
// Store operations
// ---------------------------------------------------------------------------

/// Returns the directory where subscription JSON files are stored.
pub fn webnovels_dir(system_dir: &Path) -> PathBuf {
    system_dir.join("webnovels")
}

/// Returns the file path for a specific subscription id.
pub fn subscription_file_path(system_dir: &Path, subscription_id: &str) -> PathBuf {
    webnovels_dir(system_dir).join(format!("{subscription_id}.json"))
}

/// Loads a subscription by id, if one exists.
pub fn load_subscription(system_dir: &Path, subscription_id: &str) -> Result<Option<Subscription>> {
    let path = subscription_file_path(system_dir, subscription_id);
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&path).map_err(VaultError::from)?;
    let subscription: Subscription = serde_json::from_str(&raw)
        .map_err(|e| VaultError::Serialization(format!("subscription JSON parse error: {e}")))?;
    Ok(Some(subscription))
}

/// Persists a subscription, creating the store directory if needed.
pub fn save_subscription(system_dir: &Path, subscription: &Subscription) -> Result<()> {
    let dir = webnovels_dir(system_dir);
    fs::create_dir_all(&dir).map_err(VaultError::from)?;
    let path = subscription_file_path(system_dir, &subscription.id);
    let json = serde_json::to_string_pretty(subscription).map_err(|e| {
        VaultError::Serialization(format!("subscription JSON serialize error: {e}"))
    })?;
    fs::write(&path, json).map_err(VaultError::from)?;
    Ok(())
}

/// Deletes a subscription record.  The novel's downloaded files are NOT
/// touched — callers decide separately whether to remove the vault folder.
pub fn delete_subscription(system_dir: &Path, subscription_id: &str) -> Result<()> {
    let path = subscription_file_path(system_dir, subscription_id);
    if path.exists() {
        fs::remove_file(&path).map_err(VaultError::from)?;
    }
    Ok(())
}

/// Returns all subscriptions, sorted by creation time (oldest first).
///
/// Files that cannot be parsed are silently skipped so one corrupt record
/// does not hide the rest of the library.
pub fn list_subscriptions(system_dir: &Path) -> Result<Vec<Subscription>> {
    let dir = webnovels_dir(system_dir);
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut subscriptions: Vec<Subscription> = fs::read_dir(&dir)
        .map_err(VaultError::from)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                return None;
            }
            let raw = fs::read_to_string(&path).ok()?;
            serde_json::from_str(&raw).ok()
        })
        .collect();

    subscriptions.sort_by_key(|subscription| subscription.created_at_unix);
    Ok(subscriptions)
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn default_true() -> bool {
    true
}

/// Returns the current UNIX timestamp in seconds.
pub fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

/// FNV-1a 64-bit hash.
///
/// Duplicated from `core::progress` (where it is private) to avoid widening
/// that module's API for a two-line function.
fn fnv1a64(bytes: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_system_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("webnovel-test-{label}-{}", std::process::id()))
    }

    #[test]
    fn normalize_url_strips_fragment_and_trailing_slash() {
        assert_eq!(
            normalize_url("https://example.com/novel/ "),
            "https://example.com/novel"
        );
        assert_eq!(
            normalize_url("https://example.com/novel#chapter-3"),
            "https://example.com/novel"
        );
        assert_eq!(
            subscription_id("https://example.com/novel/"),
            subscription_id("https://example.com/novel#toc")
        );
    }

    #[test]
    fn save_load_delete_roundtrip() {
        let dir = temp_system_dir("crud");
        let mut subscription =
            Subscription::new("https://example.com/fiction/123", "royalroad", "Test Novel");
        subscription.known_chapters.push(KnownChapter {
            index: 1,
            title: "Chapter 1".to_string(),
            url: "https://example.com/fiction/123/chapter/1".to_string(),
            downloaded_at_unix: Some(1_700_000_000),
        });

        save_subscription(&dir, &subscription).expect("save should succeed");
        let loaded = load_subscription(&dir, &subscription.id)
            .expect("load should succeed")
            .expect("subscription should exist");
        assert_eq!(loaded.title, "Test Novel");
        assert_eq!(loaded.known_chapters.len(), 1);
        assert_eq!(loaded.downloaded_count(), 1);
        assert!(loaded.enabled);

        let all = list_subscriptions(&dir).expect("list should succeed");
        assert_eq!(all.len(), 1);

        delete_subscription(&dir, &subscription.id).expect("delete should succeed");
        assert!(load_subscription(&dir, &subscription.id)
            .expect("load should succeed")
            .is_none());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn list_skips_corrupt_files() {
        let dir = temp_system_dir("corrupt");
        let store = webnovels_dir(&dir);
        std::fs::create_dir_all(&store).expect("dir should create");
        std::fs::write(store.join("broken.json"), "{ not json").expect("write should succeed");

        let subscription = Subscription::new("https://example.com/fiction/9", "royalroad", "Valid");
        save_subscription(&dir, &subscription).expect("save should succeed");

        let all = list_subscriptions(&dir).expect("list should succeed");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].title, "Valid");

        std::fs::remove_dir_all(&dir).ok();
    }
}
