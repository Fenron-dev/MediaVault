//! Desktop shell bootstrap for MediaVault.

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::api::anilist::{AniListAnimeMetadata, AniListClient};
use crate::core::duplicate::compute_fingerprint;
use crate::core::duplicate::compute_fingerprint_for_file;
use crate::core::import::{
    ClassificationSource, DuplicatePolicy, FileClassification, ImportConfig, ImportPlan,
    ImportPlanItem, ImportPlanner, IncomingFile, PlannedImportStep, ResolvedMetadata, UserPrompt,
};
use crate::core::properties::{render_sidecar_yaml, sidecar_path_for};
use crate::core::vault::{RelativePath, Vault};
use crate::error::{Result, VaultError};
use crate::media::{MediaEntry, MediaStatus, MediaType, PropertySource};
use serde::{Deserialize, Serialize};
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
                "/app.js" => response(
                    StatusCode::OK,
                    "application/javascript; charset=utf-8",
                    APP_JS,
                ),
                "/styles.css" => response(StatusCode::OK, "text/css; charset=utf-8", STYLES_CSS),
                "/api/vault-plan" => json_response(
                    StatusCode::OK,
                    &build_plan_response(request.uri().query()),
                ),
                "/api/anilist-search" => json_response(
                    StatusCode::OK,
                    &build_anilist_search_response(request.uri().query()),
                ),
                "/api/select-folder" => json_response(
                    StatusCode::OK,
                    &build_select_folder_response(),
                ),
                "/api/create-vault" => json_response(
                    StatusCode::OK,
                    &build_create_vault_response(request.uri().query()),
                ),
                "/api/vault-root" => json_response(
                    StatusCode::OK,
                    &build_vault_root_response(request.uri().query()),
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
    let root_override = query.and_then(|query| extract_query_value(query, "root"));

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
        Ok(None) => build_demo_plan(Some(
            "Kein Vault gefunden, daher Demo-Daten angezeigt.".to_string(),
        )),
        Err(error) => build_error_plan(
            format!("Vault-Erkennung fehlgeschlagen: {error}"),
            None,
        ),
    }
}

fn build_anilist_search_response(query: Option<&str>) -> AniListSearchResponse {
    let Some(query) = query else {
        return AniListSearchResponse::error("missing query".to_string());
    };
    let Some(title) = extract_query_value(query, "title") else {
        return AniListSearchResponse::error("missing title".to_string());
    };

    let adult = extract_query_value(query, "adult")
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let limit = extract_query_value(query, "limit")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(8);
    let client = AniListClient::default();

    match client.search_anime_candidates(&title, adult, limit) {
        Ok(results) => AniListSearchResponse {
            metadata: results.first().cloned(),
            results,
            error: None,
        },
        Err(error) => AniListSearchResponse::error(error.to_string()),
    }
}

fn build_create_vault_response(query: Option<&str>) -> CreateVaultResponse {
    let Some(query) = query else {
        return CreateVaultResponse::error("missing query".to_string());
    };

    let Some(parent) = extract_query_value(query, "parent") else {
        return CreateVaultResponse::error("missing parent".to_string());
    };
    let Some(name) = extract_query_value(query, "name") else {
        return CreateVaultResponse::error("missing name".to_string());
    };

    match create_vault_at(&parent, &name) {
        Ok(path) => CreateVaultResponse {
            path: Some(path.display().to_string()),
            created: true,
            error: None,
        },
        Err(error) => CreateVaultResponse::error(error.to_string()),
    }
}

fn build_vault_root_response(query: Option<&str>) -> VaultRootResponse {
    let mut state = load_app_state().unwrap_or_default();

    if let Some(query) = query {
        if let Some(root) = extract_query_value(query, "root") {
            let normalized = root.trim();
            if normalized.is_empty() {
                state.vault_root = None;
            } else {
                match resolve_existing_root(PathBuf::from(normalized)) {
                    Ok(resolved) => {
                        state.vault_root = Some(resolved.display().to_string());
                    }
                    Err(error) => {
                        return VaultRootResponse::error(error.to_string());
                    }
                }
            }

            if let Err(error) = save_app_state(&state) {
                return VaultRootResponse::error(error.to_string());
            }
        }
    }

    VaultRootResponse {
        root: state.vault_root,
        error: None,
    }
}

fn build_select_folder_response() -> SelectFolderResponse {
    let selected = rfd::FileDialog::new().pick_folder();
    SelectFolderResponse {
        path: selected.map(|path| path.display().to_string()),
        error: None,
    }
}

fn create_vault_at(parent: &str, name: &str) -> Result<PathBuf> {
    let parent = PathBuf::from(parent.trim());
    if parent.as_os_str().is_empty() {
        return Err(VaultError::InvalidVaultPath(
            "parent path is empty".to_string(),
        ));
    }
    if !parent.exists() {
        return Err(VaultError::InvalidVaultPath(format!(
            "parent path does not exist: {}",
            parent.display()
        )));
    }
    if !parent.is_dir() {
        return Err(VaultError::InvalidVaultPath(format!(
            "parent path is not a directory: {}",
            parent.display()
        )));
    }

    let vault_name = sanitize_path_segment(name.trim());
    if vault_name.is_empty() {
        return Err(VaultError::InvalidVaultPath(
            "vault name is empty".to_string(),
        ));
    }

    let vault_root = parent.join(vault_name);
    fs::create_dir_all(&vault_root).map_err(VaultError::from)?;

    let vault = Vault::new(vault_root.clone())?;
    fs::create_dir_all(vault.inbox_dir()).map_err(VaultError::from)?;
    fs::create_dir_all(vault.review_queue_dir()).map_err(VaultError::from)?;
    fs::create_dir_all(vault.covers_dir()).map_err(VaultError::from)?;
    fs::create_dir_all(vault.system_dir()).map_err(VaultError::from)?;

    fs::canonicalize(&vault_root).map_err(VaultError::from)
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
    let anilist_client = AniListClient::default();

    let mut items = Vec::with_capacity(files.len());
    let mut anilist_results = Vec::with_capacity(files.len());
    let mut seen_fingerprints = HashSet::new();

    for file in &files {
        let mut item = planner.plan_file(file)?;
        anilist_results.push(resolve_anilist_metadata(&anilist_client, file, &item));
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
        items: files
            .iter()
            .zip(items.iter())
            .zip(anilist_results.iter())
            .map(|((file, item), anilist_metadata)| {
                DemoPlanItem::from_scanned(file, item, anilist_metadata.as_ref())
            })
            .collect(),
    })
}

fn build_demo_plan(note: Option<String>) -> DemoPlanResponse {
    let planner = ImportPlanner::new(ImportConfig {
        duplicate_policy: DuplicatePolicy::AskUser,
        ..ImportConfig::default()
    });
    let anilist_client = AniListClient::default();

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
    let mut anilist_results = Vec::with_capacity(files.len());
    let mut seen_fingerprints = HashSet::new();

    for file in &files {
        let mut item = planner
            .plan_file(file)
            .expect("demo planning should succeed");
        anilist_results.push(resolve_anilist_metadata(&anilist_client, file, &item));

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
        items: files
            .iter()
            .zip(items.iter())
            .zip(anilist_results.iter())
            .map(|((file, item), anilist_metadata)| {
                DemoPlanItem::from_scanned(file, item, anilist_metadata.as_ref())
            })
            .collect(),
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
struct AniListSearchResponse {
    metadata: Option<AniListAnimeMetadata>,
    results: Vec<AniListAnimeMetadata>,
    error: Option<String>,
}

impl AniListSearchResponse {
    fn error(error: String) -> Self {
        Self {
            metadata: None,
            results: Vec::new(),
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct CreateVaultResponse {
    path: Option<String>,
    created: bool,
    error: Option<String>,
}

impl CreateVaultResponse {
    fn error(error: String) -> Self {
        Self {
            path: None,
            created: false,
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct SelectFolderResponse {
    path: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DemoPlanItem {
    source_path: String,
    target_path: Option<String>,
    manual_review: bool,
    needs_review: bool,
    duplicate_of: Option<String>,
    media_type: String,
    classification_source: Option<String>,
    confidence: Option<f32>,
    title: Option<String>,
    year: Option<u16>,
    series_title: Option<String>,
    season_number: Option<u16>,
    episode_start: Option<u16>,
    episode_end: Option<u16>,
    episode_title: Option<String>,
    episode_count: Option<u16>,
    runtime_minutes: Option<u16>,
    average_score: Option<f32>,
    format: Option<String>,
    airing_season: Option<String>,
    anilist_id: Option<u32>,
    anilist_url: Option<String>,
    collection_path: String,
    size_bytes: u64,
    folder_segment: String,
    sidecar_path: Option<String>,
    sidecar_preview: String,
    steps: Vec<String>,
}

impl DemoPlanItem {
    fn from_scanned(
        file: &IncomingFile,
        item: &ImportPlanItem,
        anilist: Option<&AniListAnimeMetadata>,
    ) -> Self {
        let classification = item.classification.as_ref().or(file.classification.as_ref());
        let media_type = classification
            .map(|classification| classification.media_type)
            .unwrap_or(MediaType::Unclassified);
        let needs_review = requires_review(item);
        let title = build_display_title(file, anilist, media_type);
        let anime_context = derive_anime_context(file, item, anilist, title.as_deref());
        let collection_path = build_collection_path(
            media_type,
            title.as_deref(),
            anime_context.as_ref(),
            anilist,
        );
        let target_path = build_target_path_preview(
            file,
            item,
            media_type,
            title.as_deref(),
            anime_context.as_ref(),
            anilist,
        );
        let preview_path = target_path
            .as_ref()
            .and_then(|path| RelativePath::new(path).ok())
            .or_else(|| item.target_path.clone())
            .unwrap_or_else(|| file.source_path.clone());

        Self {
            source_path: item.source_path.to_string(),
            target_path,
            manual_review: item.manual_review,
            needs_review,
            duplicate_of: item.duplicate_of.clone(),
            media_type: media_type.to_string(),
            classification_source: classification
                .map(|classification| classification_source_label(&classification.source)),
            confidence: classification.map(|classification| classification.confidence),
            title,
            year: file.metadata.as_ref().and_then(|metadata| metadata.year),
            series_title: anime_context
                .as_ref()
                .and_then(|context| context.series_title.clone()),
            season_number: anime_context
                .as_ref()
                .and_then(|context| context.season_number),
            episode_start: anime_context
                .as_ref()
                .and_then(|context| context.episode_start),
            episode_end: anime_context
                .as_ref()
                .and_then(|context| context.episode_end),
            episode_title: anime_context
                .as_ref()
                .and_then(|context| context.episode_title.clone()),
            episode_count: anilist.and_then(|metadata| metadata.episodes),
            runtime_minutes: anilist.and_then(|metadata| metadata.duration),
            average_score: anilist.and_then(|metadata| metadata.average_score),
            format: anilist.and_then(|metadata| metadata.format.clone()),
            airing_season: anilist.and_then(|metadata| metadata.season.clone()),
            anilist_id: anilist.map(|metadata| metadata.anilist_id),
            anilist_url: anilist.and_then(|metadata| metadata.anilist_url.clone()),
            collection_path,
            size_bytes: file.size_bytes,
            folder_segment: media_type.folder_segment().to_string(),
            sidecar_path: sidecar_path_for(&preview_path).ok().map(|path| path.to_string()),
            sidecar_preview: render_sidecar_preview(
                file,
                item,
                media_type,
                anilist,
                anime_context.as_ref(),
            ),
            steps: item.steps.iter().cloned().map(format_plan_step).collect(),
        }
    }
}

fn render_sidecar_preview(
    file: &IncomingFile,
    item: &ImportPlanItem,
    media_type: MediaType,
    anilist: Option<&AniListAnimeMetadata>,
    anime_context: Option<&AnimeEpisodeContext>,
) -> String {
    let relative_path = item
        .target_path
        .clone()
        .unwrap_or_else(|| file.source_path.clone());
    let mut entry = MediaEntry::new(
        format!("preview-{}", file.source_path),
        media_type,
        relative_path,
        file.source_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| file.source_path.to_string()),
    );

    entry.source = match item.classification.as_ref().map(|classification| classification.source) {
        Some(ClassificationSource::Api) => PropertySource::Api,
        Some(ClassificationSource::Ai) => PropertySource::Ai,
        Some(ClassificationSource::User) => PropertySource::User,
        Some(ClassificationSource::Folder)
        | Some(ClassificationSource::Filename)
        | Some(ClassificationSource::Extension)
        | Some(ClassificationSource::Unknown)
        | None => PropertySource::System,
    };
    entry.properties.status = Some(if item.manual_review || item.duplicate_of.is_some() {
        MediaStatus::NeedsReview
    } else {
        MediaStatus::Inbox
    });
    entry.properties.title = build_display_title(file, anilist, media_type);
    entry.properties.title_original = anilist.and_then(|metadata| metadata.title_native.clone());
    entry.properties.description = anilist.and_then(|metadata| metadata.description.clone());
    entry.properties.year = anilist
        .and_then(|metadata| metadata.season_year)
        .or(file.metadata.as_ref().and_then(|metadata| metadata.year));
    entry.properties.anilist_id = anilist.map(|metadata| metadata.anilist_id);
    entry.properties.anilist_url = anilist.and_then(|metadata| metadata.anilist_url.clone());
    entry.properties.series_title = anime_context
        .and_then(|context| context.series_title.clone())
        .or_else(|| {
            anilist.and_then(|metadata| metadata.display_title().map(|value| value.to_string()))
        });
    entry.properties.season_number = anime_context.and_then(|context| context.season_number);
    entry.properties.episode_start = anime_context.and_then(|context| context.episode_start);
    entry.properties.episode_end = anime_context.and_then(|context| context.episode_end);
    entry.properties.episode_title =
        anime_context.and_then(|context| context.episode_title.clone());
    entry.properties.episode_count = anilist.and_then(|metadata| metadata.episodes);
    entry.properties.runtime_minutes = anilist.and_then(|metadata| metadata.duration);
    entry.properties.average_score = anilist.and_then(|metadata| metadata.average_score);
    entry.properties.format = anilist.and_then(|metadata| metadata.format.clone());
    entry.properties.airing_season = anilist.and_then(|metadata| metadata.season.clone());
    entry.properties.rating_external = anilist.and_then(|metadata| metadata.average_score);
    entry.properties.genres = anilist
        .map(|metadata| metadata.genres.clone())
        .unwrap_or_default();
    entry.properties.categories = anilist
        .and_then(|metadata| metadata.format.clone())
        .map(|format| vec![format])
        .unwrap_or_default();
    entry.properties.notes = Some(if item.manual_review {
        "Manuelle Prüfung erforderlich".to_string()
    } else if item.duplicate_of.is_some() {
        "Als Duplikat markiert".to_string()
    } else {
        "Automatisch erzeugte Vorschau".to_string()
    });
    render_sidecar_yaml(&entry).unwrap_or_else(|error| {
        format!("---\nerror: {error}\n---\n")
    })
}

#[derive(Debug, Clone, Default)]
struct AnimeEpisodeContext {
    series_title: Option<String>,
    season_number: Option<u16>,
    episode_start: Option<u16>,
    episode_end: Option<u16>,
    episode_title: Option<String>,
}

fn build_display_title(
    file: &IncomingFile,
    anilist: Option<&AniListAnimeMetadata>,
    media_type: MediaType,
) -> Option<String> {
    if matches!(media_type, MediaType::Anime | MediaType::HentaiAnime) {
        if let Some(anilist_title) = anilist.and_then(|metadata| metadata.display_title()) {
            return Some(anilist_title.to_string());
        }
    }

    file.metadata
        .as_ref()
        .and_then(|metadata| metadata.title.clone())
        .or_else(|| {
            file.source_path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })
        .map(|value| normalize_title_candidate(&value))
        .filter(|value| !value.is_empty())
}

fn derive_anime_context(
    file: &IncomingFile,
    _item: &ImportPlanItem,
    anilist: Option<&AniListAnimeMetadata>,
    series_title_hint: Option<&str>,
) -> Option<AnimeEpisodeContext> {
    let media_type = file.classification.as_ref()?.media_type;
    if !matches!(media_type, MediaType::Anime | MediaType::HentaiAnime) {
        return None;
    }

    let file_name = file
        .source_path
        .file_stem()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| file.source_path.to_string());
    let series_title = anilist
        .and_then(|metadata| metadata.display_title().map(|value| value.to_string()))
        .or_else(|| series_title_hint.map(|value| value.to_string()))
        .or_else(|| extract_series_hint_from_path(&file.source_path))
        .or_else(|| extract_anime_series_hint(&file.source_path))
        .or_else(|| Some(normalize_title_candidate(&file_name)));
    let season_number = extract_season_number(&file.source_path).or(Some(1));
    let (episode_start, episode_end) = parse_episode_range(&file_name);
    let episode_title = parse_episode_title(&file_name, series_title.as_deref());

    Some(AnimeEpisodeContext {
        series_title,
        season_number,
        episode_start,
        episode_end,
        episode_title,
    })
}

fn build_collection_path(
    media_type: MediaType,
    title: Option<&str>,
    anime_context: Option<&AnimeEpisodeContext>,
    anilist: Option<&AniListAnimeMetadata>,
) -> String {
    match media_type {
        MediaType::Anime | MediaType::HentaiAnime => {
            if is_anilist_movie(anilist) {
                return format!(
                    "Anime/Filme/{}",
                    sanitize_path_segment(title.unwrap_or("Unbenannt"))
                );
            }

            let series = anime_context
                .and_then(|context| context.series_title.as_deref())
                .or(title)
                .unwrap_or("Unbenannt");
            let season_number = anime_context
                .and_then(|context| context.season_number)
                .unwrap_or(1);
            format!(
                "Anime/Serien/{}/Staffel {}",
                sanitize_path_segment(series),
                season_number
            )
        }
        MediaType::Series => format!(
            "Series/{}",
            sanitize_path_segment(title.unwrap_or("Unbenannt"))
        ),
        MediaType::Film => format!(
            "Movies/{}",
            sanitize_path_segment(title.unwrap_or("Unbenannt"))
        ),
        _ => {
            let folder = media_type.folder_segment();
            if let Some(title) = title {
                format!("{folder}/{}", sanitize_path_segment(title))
            } else {
                folder.to_string()
            }
        }
    }
}

fn build_target_path_preview(
    file: &IncomingFile,
    item: &ImportPlanItem,
    media_type: MediaType,
    title: Option<&str>,
    anime_context: Option<&AnimeEpisodeContext>,
    anilist: Option<&AniListAnimeMetadata>,
) -> Option<String> {
    if !matches!(media_type, MediaType::Anime | MediaType::HentaiAnime) {
        return item.target_path.as_ref().map(|path| path.to_string());
    }

    if is_anilist_movie(anilist) {
        let movie_title = sanitize_path_segment(title.unwrap_or("Unbenannt"));
        let extension = file
            .source_path
            .extension()
            .map(|ext| ext.to_string_lossy().to_string());
        let mut path = PathBuf::from("Anime");
        path.push("Filme");
        path.push(&movie_title);
        let file_name = match extension {
            Some(extension) if !extension.is_empty() => format!("{movie_title}.{extension}"),
            _ => movie_title,
        };
        path.push(file_name.as_str());
        return Some(path.display().to_string());
    }

    let series_title = anime_context
        .and_then(|context| context.series_title.as_deref())
        .or(title)
        .unwrap_or("Unbenannt");
    let season_number = anime_context
        .and_then(|context| context.season_number)
        .unwrap_or(1);
    let episode_label = anime_context
        .and_then(|context| format_episode_label(context))
        .unwrap_or_else(|| {
            file.source_path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
                .unwrap_or_else(|| "episode".to_string())
        });
    let extension = file
        .source_path
        .extension()
        .map(|ext| ext.to_string_lossy().to_string());
    let mut path = PathBuf::from("Anime");
    path.push("Serien");
    path.push(sanitize_path_segment(series_title));
    path.push(format!("Staffel {season_number}"));
    let file_name = match extension {
        Some(extension) if !extension.is_empty() => {
            format!(
                "{} - {}.{}",
                sanitize_path_segment(series_title),
                sanitize_path_segment(&episode_label),
                extension
            )
        }
        _ => format!(
            "{} - {}",
            sanitize_path_segment(series_title),
            sanitize_path_segment(&episode_label)
        ),
    };
    path.push(file_name);
    Some(path.display().to_string())
}

fn is_anilist_movie(anilist: Option<&AniListAnimeMetadata>) -> bool {
    anilist
        .and_then(|metadata| metadata.format.as_deref())
        .map(|format| format.eq_ignore_ascii_case("MOVIE"))
        .unwrap_or(false)
}

fn format_episode_label(context: &AnimeEpisodeContext) -> Option<String> {
    match (context.episode_start, context.episode_end) {
        (Some(start), Some(end)) if start != end => Some(format!("{start:02}-{end:02}")),
        (Some(start), _) => Some(format!("{start:02}")),
        _ => context.episode_title.clone(),
    }
}

fn resolve_anilist_metadata(
    client: &AniListClient,
    file: &IncomingFile,
    item: &ImportPlanItem,
) -> Option<AniListAnimeMetadata> {
    let classification = item.classification.as_ref().or(file.classification.as_ref())?;
    if !should_attempt_anilist(classification.media_type, &file.source_path) {
        return None;
    }

    let search_title = build_anime_search_title(file)?;
    client
        .search_anime(&search_title, AniListClient::adult_flag_for(classification.media_type))
        .ok()
        .flatten()
}

fn should_attempt_anilist(media_type: MediaType, source_path: &RelativePath) -> bool {
    if matches!(media_type, MediaType::Anime | MediaType::HentaiAnime) {
        return true;
    }

    source_path
        .to_string()
        .to_lowercase()
        .contains("anime")
}

fn build_anime_search_title(file: &IncomingFile) -> Option<String> {
    let raw = file
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.title.clone())
        .or_else(|| extract_series_hint_from_path(&file.source_path))
        .or_else(|| extract_anime_series_hint(&file.source_path))
        .or_else(|| {
            file.source_path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })?;

    let cleaned = normalize_title_candidate(&raw);
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

fn normalize_title_candidate(value: &str) -> String {
    let mut normalized = value.replace(['_', '.'], " ");
    normalized = strip_bracketed_sections(&normalized);
    normalized = normalized.replace("  ", " ");

    if let Some((head, tail)) = normalized.rsplit_once(" - ") {
        if looks_like_episode_fragment(tail) {
            normalized = head.to_string();
        }
    }

    normalized
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn strip_bracketed_sections(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut square_depth = 0usize;
    let mut round_depth = 0usize;

    for character in value.chars() {
        match character {
            '[' => square_depth += 1,
            ']' => square_depth = square_depth.saturating_sub(1),
            '(' => round_depth += 1,
            ')' => round_depth = round_depth.saturating_sub(1),
            _ if square_depth == 0 && round_depth == 0 => result.push(character),
            _ => {}
        }
    }

    result
}

fn looks_like_episode_fragment(value: &str) -> bool {
    let lower = value.trim().to_lowercase();
    if lower.is_empty() {
        return false;
    }

    lower.chars().all(|character| {
        character.is_ascii_digit()
            || matches!(
                character,
                '+' | '-' | 'e' | 'p' | 's' | 'x' | ' ' | '.'
            )
    }) || lower.contains("episode")
        || lower.contains("ep")
}

fn parse_episode_range(value: &str) -> (Option<u16>, Option<u16>) {
    let lower = value.to_lowercase();
    if let Some((start, end)) = lower.split_once('+') {
        return (parse_leading_number(start), parse_leading_number(end));
    }

    if let Some((start, end)) = lower.split_once('-') {
        let start_number = parse_leading_number(start);
        let end_number = parse_leading_number(end);
        if start_number.is_some() && end_number.is_some() {
            return (start_number, end_number);
        }
    }

    if let Some(number) = extract_number_after_marker(&lower, "episode") {
        return (Some(number), Some(number));
    }

    if let Some(number) = extract_number_after_marker(&lower, "ep") {
        return (Some(number), Some(number));
    }

    if let Some(number) = extract_short_marker_number(&lower, 'e') {
        return (Some(number), Some(number));
    }

    if let Some(number) = trailing_number(&lower) {
        return (Some(number), Some(number));
    }

    (None, None)
}

fn parse_episode_title(value: &str, series_title: Option<&str>) -> Option<String> {
    let cleaned = normalize_title_candidate(value);
    if cleaned.is_empty() {
        return None;
    }

    if let Some(series_title) = series_title {
        let lower_cleaned = cleaned.to_lowercase();
        let lower_series = series_title.to_lowercase();
        if lower_cleaned == lower_series {
            return None;
        }
        if lower_cleaned.starts_with(&lower_series) {
            if let Some(rest) = cleaned.get(series_title.len()..) {
                let rest = rest.trim_start_matches(['-', ':', ' ']).trim();
                if !rest.is_empty() {
                    return Some(rest.to_string());
                }
            }
        }
    }

    Some(cleaned)
}

fn extract_anime_series_hint(relative_path: &RelativePath) -> Option<String> {
    let mut components = relative_path.as_path().components();
    while let Some(component) = components.next() {
        let text = component.as_os_str().to_string_lossy();
        if text.eq_ignore_ascii_case("anime") {
            let mut series_candidate = None;
            for next in components {
                let candidate = next.as_os_str().to_string_lossy().to_string();
                if is_season_folder(&candidate) {
                    continue;
                }
                series_candidate = Some(candidate);
                break;
            }
            return series_candidate.map(|value| normalize_title_candidate(&value));
        }
    }

    None
}

fn extract_series_hint_from_path(relative_path: &RelativePath) -> Option<String> {
    let mut previous_meaningful: Option<String> = None;

    for component in relative_path.as_path().components() {
        let text = component.as_os_str().to_string_lossy();
        let value = text.trim();
        if value.is_empty() || is_hidden_system_entry(value) {
            continue;
        }

        if is_season_folder(value) {
            if let Some(previous) = previous_meaningful.as_ref() {
                let candidate = normalize_title_candidate(previous);
                if !candidate.is_empty() {
                    return Some(candidate);
                }
            }
            continue;
        }

        previous_meaningful = Some(value.to_string());
    }

    None
}

fn is_hidden_system_entry(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with("._")
        || trimmed.starts_with('.')
        || trimmed.eq_ignore_ascii_case(".ds_store")
        || trimmed.eq_ignore_ascii_case("thumbs.db")
        || trimmed.eq_ignore_ascii_case("desktop.ini")
}

fn extract_season_number(relative_path: &RelativePath) -> Option<u16> {
    let path = relative_path.to_string().to_lowercase();
    if let Some(number) = extract_number_after_marker(&path, "season ") {
        return Some(number);
    }
    if let Some(number) = extract_number_after_marker(&path, "staffel ") {
        return Some(number);
    }
    if let Some(number) = extract_number_after_marker(&path, "season") {
        return Some(number);
    }
    if let Some(number) = extract_number_after_marker(&path, "staffel") {
        return Some(number);
    }
    if let Some(number) = extract_short_marker_number(&path, 's') {
        return Some(number);
    }

    None
}

fn is_season_folder(value: &str) -> bool {
    let lower = value.trim().to_lowercase();
    lower.starts_with("season ")
        || lower.starts_with("season_")
        || lower.starts_with("season-")
        || lower.starts_with("season")
        || lower.starts_with("staffel ")
        || lower.starts_with("staffel_")
        || lower.starts_with("staffel-")
        || lower.starts_with("staffel")
        || extract_short_marker_number(&lower, 's').is_some()
}

fn extract_number_after_marker(value: &str, marker: &str) -> Option<u16> {
    let index = value.find(marker)?;
    let tail = &value[index + marker.len()..];
    parse_leading_number(tail)
}

fn extract_short_marker_number(value: &str, marker: char) -> Option<u16> {
    for (index, character) in value.char_indices() {
        if character != marker {
            continue;
        }

        let before_is_boundary = value[..index]
            .chars()
            .next_back()
            .map(|previous| !previous.is_ascii_alphanumeric())
            .unwrap_or(true);
        let after_is_digit = value[index + character.len_utf8()..]
            .chars()
            .next()
            .map(|next| next.is_ascii_digit())
            .unwrap_or(false);

        if before_is_boundary && after_is_digit {
            return parse_leading_number(&value[index + character.len_utf8()..]);
        }
    }

    None
}

fn parse_leading_number(value: &str) -> Option<u16> {
    let digits: String = value
        .chars()
        .skip_while(|character| !character.is_ascii_digit())
        .take_while(|character| character.is_ascii_digit())
        .collect();

    if digits.is_empty() {
        return None;
    }

    digits.parse().ok()
}

fn trailing_number(value: &str) -> Option<u16> {
    let digits: String = value
        .chars()
        .rev()
        .skip_while(|character| !character.is_ascii_digit())
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    if digits.is_empty() {
        return None;
    }

    digits.parse().ok()
}

fn sanitize_path_segment(value: &str) -> String {
    let mut sanitized = String::with_capacity(value.len());

    for character in value.chars() {
        let replacement = match character {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => ' ',
            control if control.is_control() => ' ',
            other => other,
        };
        sanitized.push(replacement);
    }

    sanitized
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn classification_source_label(source: &ClassificationSource) -> String {
    match source {
        ClassificationSource::User => "user".to_string(),
        ClassificationSource::Folder => "folder".to_string(),
        ClassificationSource::Filename => "filename".to_string(),
        ClassificationSource::Extension => "extension".to_string(),
        ClassificationSource::Api => "api".to_string(),
        ClassificationSource::Ai => "ai".to_string(),
        ClassificationSource::Unknown => "unknown".to_string(),
    }
}

fn requires_review(item: &ImportPlanItem) -> bool {
    item.manual_review
        || item.duplicate_of.is_some()
        || item
            .steps
            .iter()
            .any(|step| {
                matches!(
                    step,
                    PlannedImportStep::QueueReview { .. } | PlannedImportStep::AskUser { .. }
                )
            })
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

fn extract_query_value(query: &str, wanted_key: &str) -> Option<String> {
    for pair in query.split('&') {
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };
        if key == wanted_key {
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

    if let Ok(Some(root)) = load_saved_vault_root() {
        return Ok(Some(resolve_existing_root(root)?));
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

fn load_saved_vault_root() -> Result<Option<PathBuf>> {
    Ok(load_app_state()?.vault_root.map(PathBuf::from))
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

fn app_state_path() -> Result<PathBuf> {
    let home = env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .ok_or_else(|| VaultError::Io("home directory not available".to_string()))?;

    Ok(PathBuf::from(home).join(".mediavault").join("state.json"))
}

fn load_app_state() -> Result<AppState> {
    let path = app_state_path()?;
    if !path.exists() {
        return Ok(AppState::default());
    }

    let raw = fs::read_to_string(&path).map_err(VaultError::from)?;
    serde_json::from_str(&raw).map_err(|error| VaultError::Serialization(error.to_string()))
}

fn save_app_state(state: &AppState) -> Result<()> {
    let path = app_state_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(VaultError::from)?;
    }

    let body = serde_json::to_string_pretty(state)
        .map_err(|error| VaultError::Serialization(error.to_string()))?;
    fs::write(path, body).map_err(VaultError::from)
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct AppState {
    vault_root: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct VaultRootResponse {
    root: Option<String>,
    error: Option<String>,
}

impl VaultRootResponse {
    fn error(error: String) -> Self {
        Self {
            root: None,
            error: Some(error),
        }
    }
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

        if should_skip_scanned_entry(&path) {
            continue;
        }

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

fn should_skip_scanned_entry(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    is_hidden_system_entry(name)
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
            } else if has_season_episode_marker(&path) {
                Some(FileClassification {
                    media_type: MediaType::Series,
                    confidence: 0.91,
                    source: ClassificationSource::Filename,
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
        Some("png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp" | "tif" | "tiff") => {
            if contains_any(&path, &PHOTO_HINTS) {
                Some(FileClassification {
                    media_type: MediaType::Photo,
                    confidence: 0.92,
                    source: if contains_any(&path, &CAMERA_HINTS) {
                        ClassificationSource::Folder
                    } else {
                        ClassificationSource::Filename
                    },
                })
            } else if contains_any(&path, &IMAGE_HINTS) {
                Some(FileClassification {
                    media_type: MediaType::Image,
                    confidence: 0.91,
                    source: ClassificationSource::Filename,
                })
            } else {
                Some(FileClassification {
                    media_type: MediaType::Image,
                    confidence: 0.81,
                    source: ClassificationSource::Extension,
                })
            }
        }
        _ => None,
    }
}

const PHOTO_HINTS: [&str; 8] = [
    "dcim",
    "camera",
    "photo",
    "photos",
    "picture",
    "pictures",
    "img_",
    "dsc",
];

const CAMERA_HINTS: [&str; 4] = ["dcim", "camera", "dsc", "img_"];

const IMAGE_HINTS: [&str; 7] = [
    "screenshot",
    "screen shot",
    "wallpaper",
    "scan",
    "diagram",
    "logo",
    "icon",
];

fn contains_any(value: &str, hints: &[&str]) -> bool {
    hints.iter().any(|hint| value.contains(hint))
}

fn has_season_episode_marker(value: &str) -> bool {
    let lower = value.to_lowercase();
    let bytes = lower.as_bytes();

    bytes.windows(6).any(|window| {
        window[0] == b's'
            && window[1].is_ascii_digit()
            && window[2].is_ascii_digit()
            && window[3] == b'e'
            && window[4].is_ascii_digit()
            && window[5].is_ascii_digit()
    })
}
