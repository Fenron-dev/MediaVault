//! Desktop shell bootstrap for MediaVault.

use std::collections::HashSet;

use crate::core::duplicate::compute_fingerprint;
use crate::core::import::{
    ClassificationSource, DuplicatePolicy, FileClassification, ImportConfig, ImportPlan,
    ImportPlanItem, ImportPlanner, IncomingFile, PlannedImportStep, ResolvedMetadata, UserPrompt,
};
use crate::core::vault::RelativePath;
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
                "/api/demo-plan" => json_response(StatusCode::OK, &build_demo_plan()),
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

fn build_demo_plan() -> DemoPlanResponse {
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
        summary,
        items: items.into_iter().map(DemoPlanItem::from).collect(),
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
