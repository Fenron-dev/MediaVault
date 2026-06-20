//! Desktop shell bootstrap for MediaVault.

use std::env;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::duplicate::compute_fingerprint;
use crate::core::duplicate::compute_fingerprint_for_file;
use crate::core::import::{
    ClassificationSource, DuplicatePolicy, FileClassification, ImportConfig, ImportPlan,
    ImportPlanItem, ImportPlanner, IncomingFile, PlannedImportStep, ResolvedMetadata, UserPrompt,
};
use crate::core::vault::{RelativePath, Vault};
use crate::media::MediaType;
use crate::error::{Result, VaultError};
use serde::Serialize;
use tauri::http::{header::CONTENT_TYPE, Response, StatusCode};

const PROTOCOL_SCHEME: &str = "mediavault";

const INDEX_HTML: &str = include_str!("../dist/index.html");
const APP_JS: &str = include_str!("../dist/app.js");
const STYLES_CSS: &str = include_str!("../dist/styles.css");

/// Starts the Tauri desktop shell.
pub(crate) fn run() -> Result<()> {
    tauri::Builder::default()
        .register_uri_scheme_protocol(PROTOCOL_SCHEME, |_context, request| {
            let path = request.uri().path();

            match path {
                "/" | "/index.html" => {
                    response(StatusCode::OK, "text/html; charset=utf-8", INDEX_HTML)
                }
                "/app.js" => response(StatusCode::OK, "application/javascript; charset=utf-8", APP_JS),
                "/styles.css" => response(StatusCode::OK, "text/css; charset=utf-8", STYLES_CSS),
                "/api/vault-plan" => json_response(
                    StatusCode::OK,
                    &build_plan_response(request.uri().query()),
                ),
                _ => response(StatusCode::NOT_FOUND, "text/plain; charset=utf-8", "Not Found"),
            }
        })
        .run(tauri::generate_context!())
        .map_err(|error| VaultError::AppStartup(error.to_string()))
}

fn response(status: StatusCode, content_type: &str, body: &str) -> Response<Vec<u8>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, content_type)
        .body(body.as_bytes().to_vec())
        .expect("response construction should succeed")
}

fn json_response<T: Serialize>(status: StatusCode, value: &T) -> Response<Vec<u8>> {
    match serde_json::to_vec(value) {
        Ok(body) => Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .body(body)
            .expect("JSON response construction should succeed"),
        Err(error) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "text/plain; charset=utf-8",
            &format!("failed to serialize JSON: {error}"),
        ),
    }
}

fn build_plan_response(query: Option<&str>) -> DemoPlanResponse {
    let root_override = query.and_then(extract_query_root);

    if let Some(root_override) = root_override {
        match build_vault_plan(Some(&root_override)) {
            Ok(plan) => return plan,
            Err(error) => {
                return build_error_plan(
                    format!("Vault-Pfad konnte nicht geladen werden: {error}"),
                    Some(root_override),
                );
            }
        }
    }

    match resolve_vault_root(None) {
        Ok(Some(vault_root)) => match build_vault_plan_with_root(vault_root) {
            Ok(plan) => plan,
            Err(error) => build_error_plan(
                format!("Vault konnte nicht gescannt werden: {error}"),
                None,
            ),
        },
        Ok(None) => build_demo_plan(Some("Kein Vault gefunden, daher Demo-Daten angezeigt.".to_string())),
        Err(error) => build_error_plan(
            format!("Vault-Erkennung fehlgeschlagen: {error}"),
            None,
        ),
    }
}

fn build_vault_plan(root_override: Option<&str>) -> Result<DemoPlanResponse> {
    let vault_root = resolve_vault_root(root_override)?
        .ok_or_else(|| VaultError::InvalidVaultPath("no vault root found".to_string()))?;
    build_vault_plan_with_root(vault_root)
}

fn build_vault_plan_with_root(vault_root: PathBuf) -> Result<DemoPlanResponse> {
    let vault = Vault::new(vault_root.clone())?;
    let files = scan_inbox_files(&vault)?;
    let planner = ImportPlanner::new(ImportConfig {
        duplicate_policy: DuplicatePolicy::AskUser,
        ..ImportConfig::default()
    });

    let mut items = Vec::with_capacity(files.len());
    let mut seen_fingerprints = HashSet::new();

    for file in &files {
        let mut item = planner.plan_file(file)?;
        if let Some(fingerprint) = item.fingerprint.as_ref() {
            let is_duplicate = !seen_fingerprints.insert(fingerprint.hash.clone());
            if is_duplicate {
                item.duplicate_of = Some(fingerprint.hash.clone());
                item.manual_review = true;
                item.target_path = None;
                item.steps.retain(|step| {
                    !matches!(
                        step,
                        PlannedImportStep::MoveFile { .. } | PlannedImportStep::WriteSidecar { .. }
                    )
                });
                item.steps.push(PlannedImportStep::AskUser {
                    prompt: UserPrompt {
                        field_name: "duplicate".to_string(),
                        message: "Duplikat gefunden".to_string(),
                        options: vec![
                            "Behalten".to_string(),
                            "Überspringen".to_string(),
                            "Zusammenführen".to_string(),
                        ],
                    },
                });
            }
        }

        items.push(item);
    }

    let plan = ImportPlan {
        dry_run: true,
        summary: Default::default(),
        items: items.clone(),
    };
    let summary = summarize_demo_plan(&plan);

    Ok(DemoPlanResponse {
        title: "Vault-Dry-Run".to_string(),
        source: "vault".to_string(),
        vault_root: Some(vault_root.display().to_string()),
        note: if files.is_empty() {
            Some("Inbox ist leer.".to_string())
        } else {
            None
        },
        summary,
        items: items.into_iter().map(DemoPlanItem::from).collect(),
    })
}

fn build_demo_plan(note: Option<String>) -> DemoPlanResponse {
    let planner = ImportPlanner::new(ImportConfig {
        duplicate_policy: DuplicatePolicy::AskUser,
        ..ImportConfig::default()
    });

    let files = vec![
        IncomingFile {
            source_path: RelativePath::new("Inbox/Violet Evergarden.mkv")
                .expect("demo relative path should be valid"),
            size_bytes: 1_843_200_000,
            fingerprint: None,
            classification: Some(FileClassification {
                media_type: MediaType::Anime,
                confidence: 0.97,
                source: ClassificationSource::Api,
            }),
            metadata: Some(ResolvedMetadata {
                title: Some("Violet Evergarden".to_string()),
                year: Some(2018),
            }),
        },
        IncomingFile {
            source_path: RelativePath::new("Inbox/unknown_scan.zip")
                .expect("demo relative path should be valid"),
            size_bytes: 42_000_000,
            fingerprint: None,
            classification: None,
            metadata: None,
        },
        IncomingFile {
            source_path: RelativePath::new("Inbox/Demo Track 01.flac")
                .expect("demo relative path should be valid"),
            size_bytes: 38_200_000,
            fingerprint: Some(compute_fingerprint(b"shared-demo-fingerprint")),
            classification: Some(FileClassification {
                media_type: MediaType::MusicTrack,
                confidence: 0.82,
                source: ClassificationSource::Filename,
            }),
            metadata: Some(ResolvedMetadata {
                title: Some("Demo Track 01".to_string()),
                year: None,
            }),
        },
        IncomingFile {
            source_path: RelativePath::new("Inbox/Demo Track 01 copy.flac")
                .expect("demo relative path should be valid"),
            size_bytes: 38_200_000,
            fingerprint: Some(compute_fingerprint(b"shared-demo-fingerprint")),
            classification: Some(FileClassification {
                media_type: MediaType::MusicTrack,
                confidence: 0.82,
                source: ClassificationSource::Filename,
            }),
            metadata: Some(ResolvedMetadata {
                title: Some("Demo Track 01".to_string()),
                year: None,
            }),
        },
    ];

    let mut items = Vec::with_capacity(files.len());
    let mut seen_fingerprints = HashSet::new();

    for file in &files {
        let mut item = planner
            .plan_file(file)
            .expect("demo planning should succeed");

        if let Some(fingerprint) = item.fingerprint.as_ref() {
            let is_duplicate = !seen_fingerprints.insert(fingerprint.hash.clone());
            if is_duplicate {
                item.duplicate_of = Some(fingerprint.hash.clone());
                item.manual_review = true;
                item.target_path = None;
                item.steps.retain(|step| {
                    !matches!(
                        step,
                        PlannedImportStep::MoveFile { .. } | PlannedImportStep::WriteSidecar { .. }
                    )
                });
                item.steps.push(PlannedImportStep::AskUser {
                    prompt: UserPrompt {
                        field_name: "duplicate".to_string(),
                        message: "Duplikat gefunden".to_string(),
                        options: vec![
                            "Behalten".to_string(),
                            "Überspringen".to_string(),
                            "Zusammenführen".to_string(),
                        ],
                    },
                });
            }
        }

        items.push(item);
    }

    let plan = ImportPlan {
        dry_run: true,
        summary: Default::default(),
        items: items.clone(),
    };
    let summary = summarize_demo_plan(&plan);

    DemoPlanResponse {
        title: "Demo-Dry-Run".to_string(),
        source: "demo".to_string(),
        vault_root: None,
        note,
        summary,
        items: items.into_iter().map(DemoPlanItem::from).collect(),
    }
}

fn build_error_plan(note: String, vault_root: Option<String>) -> DemoPlanResponse {
    DemoPlanResponse {
        title: "Vault-Dry-Run".to_string(),
        source: "error".to_string(),
        vault_root,
        note: Some(note),
        summary: DemoSummary::default(),
        items: Vec::new(),
    }
}

fn summarize_demo_plan(plan: &ImportPlan) -> DemoSummary {
    let mut summary = DemoSummary::default();
    summary.total_files = plan.items.len();
    summary.items_needing_review = plan
        .items
        .iter()
        .filter(|item| requires_review(item))
        .count();
    summary.duplicates = plan
        .items
        .iter()
        .filter(|item| item.duplicate_of.is_some())
        .count();
    summary.planned_moves = plan
        .items
        .iter()
        .filter(|item| {
            item.steps
                .iter()
                .any(|step| matches!(step, PlannedImportStep::MoveFile { .. }))
        })
        .count();
    summary.planned_sidecars = plan
        .items
        .iter()
        .filter(|item| {
            item.steps
                .iter()
                .any(|step| matches!(step, PlannedImportStep::WriteSidecar { .. }))
        })
        .count();
    summary.planned_api_fetches = plan
        .items
        .iter()
        .map(|item| {
            item.steps
                .iter()
                .filter(|step| matches!(step, PlannedImportStep::FetchMetadata { .. }))
                .count()
        })
        .sum();
    summary.smart_collections = 3;
    summary
}

#[derive(Debug, Clone, Serialize)]
struct DemoPlanResponse {
    title: String,
    source: String,
    vault_root: Option<String>,
    note: Option<String>,
    summary: DemoSummary,
    items: Vec<DemoPlanItem>,
}

#[derive(Debug, Clone, Default, Serialize)]
struct DemoSummary {
    total_files: usize,
    items_needing_review: usize,
    duplicates: usize,
    planned_moves: usize,
    planned_sidecars: usize,
    planned_api_fetches: usize,
    smart_collections: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DemoPlanItem {
    source_path: String,
    target_path: Option<String>,
    manual_review: bool,
    needs_review: bool,
    duplicate_of: Option<String>,
    steps: Vec<String>,
}

impl From<ImportPlanItem> for DemoPlanItem {
    fn from(item: ImportPlanItem) -> Self {
        let needs_review = requires_review(&item);
        Self {
            source_path: item.source_path.to_string(),
            target_path: item.target_path.map(|path| path.to_string()),
            manual_review: item.manual_review,
            needs_review,
            duplicate_of: item.duplicate_of,
            steps: item.steps.into_iter().map(format_plan_step).collect(),
        }
    }
}

fn requires_review(item: &ImportPlanItem) -> bool {
    item.manual_review
        || item.duplicate_of.is_some()
        || item
            .steps
            .iter()
            .any(|step| matches!(step, PlannedImportStep::QueueReview { .. } | PlannedImportStep::AskUser { .. }))
}

fn format_plan_step(step: PlannedImportStep) -> String {
    match step {
        PlannedImportStep::DetectType => "Typ erkennen".to_string(),
        PlannedImportStep::FetchMetadata { provider } => format!("Metadaten von {provider} holen"),
        PlannedImportStep::AskUser { prompt } => {
            format!("Benutzer fragen: {}", prompt.message)
        }
        PlannedImportStep::MoveFile { target } => format!("Datei verschieben nach {target}"),
        PlannedImportStep::WriteSidecar { target } => format!("Sidecar schreiben nach {target}"),
        PlannedImportStep::RecordAudit => "Audit-Eintrag schreiben".to_string(),
        PlannedImportStep::RegisterDuplicate { fingerprint } => {
            format!("Fingerprint registrieren {fingerprint}")
        }
        PlannedImportStep::QueueReview { reason } => format!("Zur Prüfung: {reason}"),
        PlannedImportStep::Skip { reason } => format!("Überspringen: {reason}"),
    }
}

fn extract_query_root(query: &str) -> Option<String> {
    for pair in query.split('&') {
        let (key, value) = pair.split_once('=')?;
        if key == "root" {
            return urlencoding::decode(value).ok().map(|value| value.into_owned());
        }
    }

    None
}

fn resolve_vault_root(root_override: Option<&str>) -> Result<Option<PathBuf>> {
    if let Some(root) = normalized_override(root_override) {
        return Ok(Some(resolve_existing_root(root)?));
    }

    if let Ok(root) = env::var("MEDIAVAULT_VAULT_ROOT") {
        if let Some(root) = normalized_override(Some(root.as_str())) {
            return Ok(Some(resolve_existing_root(root)?));
        }
    }

    for candidate in auto_detect_vault_roots() {
        if looks_like_vault_root(&candidate) {
            return Ok(Some(resolve_existing_root(candidate)?));
        }
    }

    Ok(None)
}

fn normalized_override(root_override: Option<&str>) -> Option<PathBuf> {
    let root = root_override?.trim();
    if root.is_empty() {
        return None;
    }

    Some(PathBuf::from(root))
}

fn resolve_existing_root(root: PathBuf) -> Result<PathBuf> {
    if !root.exists() {
        return Err(VaultError::InvalidVaultPath(format!(
            "vault root does not exist: {}",
            root.display()
        )));
    }

    fs::canonicalize(&root).map_err(VaultError::from)
}

fn auto_detect_vault_roots() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current_exe) = env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            candidates.push(parent.join("Vault"));
            candidates.push(parent.to_path_buf());
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        candidates.push(current_dir.join("Vault"));
        candidates.push(current_dir);
    }

    candidates
}

fn looks_like_vault_root(path: &Path) -> bool {
    path.join("Inbox").is_dir() || path.join(".mediashelf").is_dir()
}

fn scan_inbox_files(vault: &Vault) -> Result<Vec<IncomingFile>> {
    let inbox_dir = vault.inbox_dir();
    if !inbox_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    scan_directory(vault, &inbox_dir, &mut files)?;
    Ok(files)
}

fn scan_directory(vault: &Vault, directory: &Path, files: &mut Vec<IncomingFile>) -> Result<()> {
    for entry in fs::read_dir(directory).map_err(VaultError::from)? {
        let entry = entry.map_err(VaultError::from)?;
        let path = entry.path();
        let metadata = entry.metadata().map_err(VaultError::from)?;

        if metadata.is_dir() {
            scan_directory(vault, &path, files)?;
            continue;
        }

        if !metadata.is_file() {
            continue;
        }

        let relative_path = vault.relative_from_absolute(&path)?;
        let fingerprint = Some(compute_fingerprint_for_file(&path)?);
        let classification = detect_classification(&relative_path);
        let resolved_metadata = Some(ResolvedMetadata {
            title: relative_path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string()),
            year: None,
        });

        files.push(IncomingFile {
            source_path: relative_path,
            size_bytes: metadata.len(),
            fingerprint,
            classification,
            metadata: resolved_metadata,
        });
    }

    Ok(())
}

fn detect_classification(relative_path: &RelativePath) -> Option<FileClassification> {
    let path = relative_path.to_string().to_lowercase();
    let extension = relative_path
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase());

    match extension.as_deref() {
        Some("mkv" | "mp4" | "avi" | "mov" | "webm") => {
            if path.contains("anime") {
                Some(FileClassification {
                    media_type: MediaType::Anime,
                    confidence: 0.94,
                    source: ClassificationSource::Folder,
                })
            } else if path.contains("series") {
                Some(FileClassification {
                    media_type: MediaType::Series,
                    confidence: 0.88,
                    source: ClassificationSource::Folder,
                })
            } else {
                Some(FileClassification {
                    media_type: MediaType::Film,
                    confidence: 0.68,
                    source: ClassificationSource::Extension,
                })
            }
        }
        Some("flac" | "mp3" | "m4a" | "aac" | "ogg" | "wav") => Some(FileClassification {
            media_type: MediaType::MusicTrack,
            confidence: 0.87,
            source: ClassificationSource::Extension,
        }),
        Some("pdf" | "epub" | "mobi" | "azw3") => Some(FileClassification {
            media_type: MediaType::Ebook,
            confidence: 0.84,
            source: ClassificationSource::Extension,
        }),
        Some("cbz" | "cbr") => Some(FileClassification {
            media_type: MediaType::Manga,
            confidence: 0.82,
            source: ClassificationSource::Extension,
        }),
        Some("zip" | "rar" | "7z" | "tar" | "gz") => Some(FileClassification {
            media_type: MediaType::Archive,
            confidence: 0.78,
            source: ClassificationSource::Extension,
        }),
        Some("png" | "jpg" | "jpeg" | "webp" | "gif") => Some(FileClassification {
            media_type: MediaType::Image,
            confidence: 0.81,
            source: ClassificationSource::Extension,
        }),
        _ => None,
    }
}
