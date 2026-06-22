//! Desktop shell bootstrap for MediaVault.

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::api::anilist::{AniListAnimeMetadata, AniListClient};
use crate::api::audiobookshelf::AbsClient;
use crate::core::duplicate::compute_fingerprint;
use crate::core::duplicate::compute_fingerprint_for_file;
use crate::core::import::{
    ClassificationSource, DuplicatePolicy, FileClassification, ImportConfig, ImportPlan,
    ImportPlanItem, ImportPlanner, IncomingFile, PlannedImportStep, ResolvedMetadata, UserPrompt,
};
use crate::core::playlist::{
    delete_cursor, delete_playlist, list_playlists, load_cursor, load_playlist, save_cursor,
    save_playlist, Playlist, PlaylistCursor, PlaylistFilter, PlaylistKind, SortRule,
};
use crate::core::progress::{
    delete_progress, list_in_progress, load_progress, save_progress, MediaProgress, ProgressRecord,
};
use crate::core::properties::{render_sidecar_yaml, sidecar_path_for};
use crate::core::vault::{RelativePath, Vault};
use crate::error::{Result, VaultError};
use crate::media::{MediaEntry, MediaStatus, MediaType, PropertySource};
use serde::{Deserialize, Serialize};
use tauri::http::{header::CONTENT_TYPE, Response, StatusCode};

const PROTOCOL_SCHEME: &str = "mediavault";
const LEGACY_SYSTEM_DIR: &str = ".mediashelf";
const LEGACY_SIDECAR_SUFFIX: &str = ".mediashelf.yaml";
const ANILIST_CACHE_FILE: &str = "anilist_cache.json";

const INBOX_SUBFOLDERS: &[&str] = &[
    "Unsortiert",
    "Anime/TV",
    "Anime/Filme",
    "Serien",
    "Filme",
    "Musik",
    "Bücher",
    "Hörbücher",
    "Manga",
    "Comics",
    "TTRPG",
    "Games",
];

type AniListCacheMap = HashMap<String, AniListAnimeMetadata>;

fn anilist_cache_path() -> Option<PathBuf> {
    let home = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE"))?;
    Some(
        PathBuf::from(home)
            .join(".mediavault")
            .join(ANILIST_CACHE_FILE),
    )
}

fn load_anilist_cache() -> AniListCacheMap {
    let path = match anilist_cache_path() {
        Some(p) => p,
        None => return HashMap::new(),
    };
    let raw = match fs::read_to_string(&path) {
        Ok(r) => r,
        Err(_) => return HashMap::new(),
    };
    serde_json::from_str(&raw).unwrap_or_default()
}

fn save_anilist_cache(cache: &AniListCacheMap) {
    let Some(path) = anilist_cache_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(body) = serde_json::to_string(cache) {
        let _ = fs::write(path, body);
    }
}

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
                "/api/vault-plan" => {
                    json_response(StatusCode::OK, &build_plan_response(request.uri().query()))
                }
                "/api/anilist-search" => json_response(
                    StatusCode::OK,
                    &build_anilist_search_response(request.uri().query()),
                ),
                "/api/select-folder" => {
                    json_response(StatusCode::OK, &build_select_folder_response())
                }
                "/api/create-vault" => json_response(
                    StatusCode::OK,
                    &build_create_vault_response(request.uri().query()),
                ),
                "/api/vault-root" => json_response(
                    StatusCode::OK,
                    &build_vault_root_response(request.uri().query()),
                ),
                "/api/media-file" => {
                    let range = request
                        .headers()
                        .get("range")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
                    build_media_file_response(request.uri().query(), range.as_deref())
                }
                "/api/apply-import" => {
                    json_response(StatusCode::OK, &build_apply_import_response(request.body()))
                }
                "/api/save-sidecars" => json_response(
                    StatusCode::OK,
                    &build_save_sidecars_response(request.body()),
                ),
                "/api/cleanup-vault" => json_response(
                    StatusCode::OK,
                    &build_cleanup_vault_response(request.uri().query()),
                ),
                "/api/progress/save" => json_response(
                    StatusCode::OK,
                    &build_save_progress_response(request.body()),
                ),
                "/api/progress/load" => json_response(
                    StatusCode::OK,
                    &build_load_progress_response(request.uri().query()),
                ),
                "/api/progress/delete" => json_response(
                    StatusCode::OK,
                    &build_delete_progress_response(request.body()),
                ),
                "/api/progress/list" => json_response(
                    StatusCode::OK,
                    &build_list_progress_response(request.uri().query()),
                ),
                "/api/open-external" => json_response(
                    StatusCode::OK,
                    &build_open_external_response(request.uri().query()),
                ),
                "/api/playlist/list" => json_response(
                    StatusCode::OK,
                    &build_list_playlists_response(request.uri().query()),
                ),
                "/api/playlist/get" => json_response(
                    StatusCode::OK,
                    &build_get_playlist_response(request.uri().query()),
                ),
                "/api/playlist/save" => json_response(
                    StatusCode::OK,
                    &build_save_playlist_response(request.body()),
                ),
                "/api/playlist/delete" => json_response(
                    StatusCode::OK,
                    &build_delete_playlist_response(request.body()),
                ),
                "/api/playlist/cursor/save" => {
                    json_response(StatusCode::OK, &build_save_cursor_response(request.body()))
                }
                "/api/playlist/cursor/load" => json_response(
                    StatusCode::OK,
                    &build_load_cursor_response(request.uri().query()),
                ),
                "/api/abs/test" => json_response(
                    StatusCode::OK,
                    &build_abs_test_response(request.uri().query()),
                ),
                "/api/abs/libraries" => json_response(
                    StatusCode::OK,
                    &build_abs_libraries_response(request.uri().query()),
                ),
                "/api/abs/library-items" => json_response(
                    StatusCode::OK,
                    &build_abs_library_items_response(request.uri().query()),
                ),
                "/api/abs/sync-progress" => json_response(
                    StatusCode::OK,
                    &build_abs_sync_progress_response(request.body()),
                ),
                "/api/recent-items" => json_response(
                    StatusCode::OK,
                    &build_recent_items_response(request.uri().query()),
                ),
                "/api/in-progress" => json_response(
                    StatusCode::OK,
                    &build_in_progress_response(request.uri().query()),
                ),
                _ => response(
                    StatusCode::NOT_FOUND,
                    "text/plain; charset=utf-8",
                    "Not Found",
                ),
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
    let refresh = query
        .and_then(|query| extract_query_value(query, "refresh"))
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if let Some(root_override) = root_override {
        match build_vault_plan(Some(&root_override), refresh) {
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
        Ok(Some(vault_root)) => match build_vault_plan_with_root(vault_root, refresh) {
            Ok(plan) => plan,
            Err(error) => {
                build_error_plan(format!("Vault konnte nicht gescannt werden: {error}"), None)
            }
        },
        Ok(None) => build_demo_plan(Some(
            "Kein Vault gefunden, daher Demo-Daten angezeigt.".to_string(),
        )),
        Err(error) => build_error_plan(format!("Vault-Erkennung fehlgeschlagen: {error}"), None),
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

fn build_media_file_response(query: Option<&str>, range: Option<&str>) -> Response<Vec<u8>> {
    let Some(query) = query else {
        return response(
            StatusCode::BAD_REQUEST,
            "text/plain; charset=utf-8",
            "missing query",
        );
    };

    let Some(path) = extract_query_value(query, "path") else {
        return response(
            StatusCode::BAD_REQUEST,
            "text/plain; charset=utf-8",
            "missing path",
        );
    };

    let root_override = extract_query_value(query, "root");
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(root)) => root,
        Ok(None) => {
            return response(
                StatusCode::BAD_REQUEST,
                "text/plain; charset=utf-8",
                "no vault open",
            );
        }
        Err(error) => {
            return response(
                StatusCode::BAD_REQUEST,
                "text/plain; charset=utf-8",
                &error.to_string(),
            );
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(vault) => vault,
        Err(error) => {
            return response(
                StatusCode::BAD_REQUEST,
                "text/plain; charset=utf-8",
                &error.to_string(),
            );
        }
    };

    let relative = match RelativePath::new(path) {
        Ok(path) => path,
        Err(error) => {
            return response(
                StatusCode::BAD_REQUEST,
                "text/plain; charset=utf-8",
                &error.to_string(),
            );
        }
    };

    let absolute = match vault.resolve(relative.as_path()) {
        Ok(path) => path,
        Err(error) => {
            return response(
                StatusCode::BAD_REQUEST,
                "text/plain; charset=utf-8",
                &error.to_string(),
            );
        }
    };

    let file_size = match fs::metadata(&absolute) {
        Ok(m) => m.len(),
        Err(error) => {
            return response(
                StatusCode::NOT_FOUND,
                "text/plain; charset=utf-8",
                &error.to_string(),
            )
        }
    };

    let content_type = media_content_type(&absolute);

    // Serve a partial range when the browser requests one (required for video seeking).
    if let Some(range_str) = range.and_then(|r| r.strip_prefix("bytes=")) {
        let (start, end) = parse_byte_range(range_str, file_size);

        if start >= file_size || end >= file_size || start > end {
            return Response::builder()
                .status(StatusCode::RANGE_NOT_SATISFIABLE)
                .header("Content-Range", format!("bytes */{file_size}"))
                .body(Vec::new())
                .expect("range error response should build");
        }

        let length = end - start + 1;

        let body = match read_file_range(&absolute, start, length) {
            Ok(b) => b,
            Err(error) => {
                return response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "text/plain; charset=utf-8",
                    &error.to_string(),
                )
            }
        };

        return Response::builder()
            .status(StatusCode::PARTIAL_CONTENT)
            .header(CONTENT_TYPE, content_type)
            .header("Content-Range", format!("bytes {start}-{end}/{file_size}"))
            .header("Content-Length", length.to_string())
            .header("Accept-Ranges", "bytes")
            .body(body)
            .expect("partial content response should build");
    }

    // No Range header — serve the full file, but advertise range support so the
    // browser knows it can seek without a full reload.
    match fs::read(&absolute) {
        Ok(body) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, content_type)
            .header("Content-Length", file_size.to_string())
            .header("Accept-Ranges", "bytes")
            .body(body)
            .expect("media response should build"),
        Err(error) => response(
            StatusCode::NOT_FOUND,
            "text/plain; charset=utf-8",
            &error.to_string(),
        ),
    }
}

/// Parses a `bytes=start-end` range spec and clamps both ends to `[0, file_size - 1]`.
fn parse_byte_range(range_str: &str, file_size: u64) -> (u64, u64) {
    let last = file_size.saturating_sub(1);

    // Suffix form: "-500" means the last 500 bytes.
    if let Some(suffix) = range_str.strip_prefix('-') {
        if let Ok(n) = suffix.parse::<u64>() {
            return (file_size.saturating_sub(n), last);
        }
        return (0, last);
    }

    let mut parts = range_str.splitn(2, '-');
    let start = parts
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let end = parts
        .next()
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(last)
        .min(last);

    (start, end)
}

/// Reads exactly `length` bytes from `path` starting at byte offset `start`.
fn read_file_range(path: &Path, start: u64, length: u64) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    file.seek(SeekFrom::Start(start))?;
    let mut buf = vec![0u8; length as usize];
    file.read_exact(&mut buf)?;
    Ok(buf)
}

fn build_apply_import_response(body: &[u8]) -> ApplyImportResponse {
    let request: ApplyImportRequest = match serde_json::from_slice(body) {
        Ok(request) => request,
        Err(error) => {
            return ApplyImportResponse::error(format!(
                "Import-Anfrage konnte nicht gelesen werden: {error}"
            ));
        }
    };

    let vault_root = match resolve_vault_root(request.vault_root.as_deref()) {
        Ok(Some(root)) => root,
        Ok(None) => {
            return ApplyImportResponse::error("Kein Vault geöffnet.".to_string());
        }
        Err(error) => {
            return ApplyImportResponse::error(format!(
                "Vault konnte nicht aufgelöst werden: {error}"
            ));
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(vault) => vault,
        Err(error) => {
            return ApplyImportResponse::error(format!(
                "Vault konnte nicht initialisiert werden: {error}"
            ));
        }
    };

    let mut applied = Vec::new();
    let mut skipped = Vec::new();

    for item in request.items {
        match apply_import_item(&vault, &item) {
            Ok(()) => {
                applied.push(item.source_path.clone());
            }
            Err(error) => skipped.push(ApplyImportSkipped {
                source_path: item.source_path.clone(),
                reason: error.to_string(),
            }),
        }
    }

    ApplyImportResponse {
        applied,
        skipped,
        error: None,
    }
}

fn build_save_sidecars_response(body: &[u8]) -> SaveSidecarsResponse {
    let request: SaveSidecarsRequest = match serde_json::from_slice(body) {
        Ok(request) => request,
        Err(error) => return SaveSidecarsResponse::error(format!("ungültiger Request: {error}")),
    };

    let vault_root = match resolve_vault_root(request.vault_root.as_deref()) {
        Ok(Some(root)) => root,
        Ok(None) => return SaveSidecarsResponse::error("kein Vault ausgewählt".to_string()),
        Err(error) => return SaveSidecarsResponse::error(error.to_string()),
    };

    let vault = match Vault::new(&vault_root) {
        Ok(vault) => vault,
        Err(error) => return SaveSidecarsResponse::error(error.to_string()),
    };

    let mut saved = Vec::new();
    let mut skipped = Vec::new();

    for item in request.items {
        match save_sidecar_item(&vault, &item) {
            Ok(()) => saved.push(item.media_path),
            Err(error) => skipped.push(ApplyImportSkipped {
                source_path: item.media_path,
                reason: error.to_string(),
            }),
        }
    }

    SaveSidecarsResponse {
        saved,
        skipped,
        error: None,
    }
}

fn build_cleanup_vault_response(query: Option<&str>) -> CleanupVaultResponse {
    let root_override = query.and_then(|query| extract_query_value(query, "root"));
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(root)) => root,
        Ok(None) => {
            return CleanupVaultResponse::error("Kein Vault geöffnet.".to_string());
        }
        Err(error) => return CleanupVaultResponse::error(error.to_string()),
    };
    let vault = match Vault::new(&vault_root) {
        Ok(vault) => vault,
        Err(error) => return CleanupVaultResponse::error(error.to_string()),
    };
    match run_vault_cleanup(&vault) {
        Ok(response) => response,
        Err(error) => CleanupVaultResponse::error(error.to_string()),
    }
}

/// Scans the vault for maintenance issues: orphaned sidecars, empty folders, and
/// thumbnail/asset entries that no longer have a corresponding media file.
fn run_vault_cleanup(vault: &Vault) -> Result<CleanupVaultResponse> {
    let mut issues: Vec<CleanupIssue> = Vec::new();

    // --- Orphaned .mediavault.yaml sidecars ---
    find_orphaned_sidecars(vault, vault.root(), &mut issues)?;

    // --- Empty directories (excluding system and inbox) ---
    find_empty_directories(vault, vault.root(), &mut issues)?;

    // --- Orphaned thumbnail cache files ---
    if vault.thumbnails_dir().exists() {
        for entry in fs::read_dir(vault.thumbnails_dir()).map_err(VaultError::from)? {
            let entry = entry.map_err(VaultError::from)?;
            let path = entry.path();
            if path.extension().map(|e| e == "jpg").unwrap_or(false) {
                // Thumbnails are named by content hash; we can't re-link them here without
                // the DB.  Flag them as potentially stale for manual review.
                issues.push(CleanupIssue {
                    kind: "stale_thumbnail".to_string(),
                    path: path.display().to_string(),
                    description: "Thumbnail ohne zugehörigen Eintrag (DB nicht verfügbar)"
                        .to_string(),
                });
            }
        }
    }

    let summary = CleanupSummary {
        orphaned_sidecars: issues
            .iter()
            .filter(|i| i.kind == "orphaned_sidecar")
            .count(),
        empty_directories: issues
            .iter()
            .filter(|i| i.kind == "empty_directory")
            .count(),
        stale_thumbnails: issues
            .iter()
            .filter(|i| i.kind == "stale_thumbnail")
            .count(),
    };

    Ok(CleanupVaultResponse {
        issues,
        summary,
        error: None,
    })
}

fn find_orphaned_sidecars(
    vault: &Vault,
    directory: &Path,
    issues: &mut Vec<CleanupIssue>,
) -> Result<()> {
    let Ok(entries) = fs::read_dir(directory) else {
        return Ok(());
    };
    for entry in entries {
        let entry = entry.map_err(VaultError::from)?;
        let path = entry.path();
        if path.is_dir() {
            if should_skip_scanned_directory(vault, &path) {
                continue;
            }
            find_orphaned_sidecars(vault, &path, issues)?;
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if name.ends_with(".mediavault.yaml") || name.ends_with(LEGACY_SIDECAR_SUFFIX) {
            // Derive the expected media filename by stripping the sidecar suffix.
            let media_path = if name.ends_with(".mediavault.yaml") {
                path.with_file_name(name.trim_end_matches(".mediavault.yaml"))
            } else {
                path.with_file_name(name.trim_end_matches(LEGACY_SIDECAR_SUFFIX))
            };
            if !media_path.exists() {
                issues.push(CleanupIssue {
                    kind: "orphaned_sidecar".to_string(),
                    path: path.display().to_string(),
                    description: format!("Sidecar ohne Mediendatei: {}", media_path.display()),
                });
            }
        }
    }
    Ok(())
}

fn find_empty_directories(
    vault: &Vault,
    directory: &Path,
    issues: &mut Vec<CleanupIssue>,
) -> Result<()> {
    let Ok(entries) = fs::read_dir(directory) else {
        return Ok(());
    };
    let mut has_content = false;

    for entry in entries {
        let entry = entry.map_err(VaultError::from)?;
        let path = entry.path();

        if path.is_dir() {
            if should_skip_scanned_directory(vault, &path)
                || path == vault.inbox_dir()
                || path.starts_with(vault.inbox_dir())
            {
                has_content = true; // inbox subfolders are intentionally empty initially
                continue;
            }
            find_empty_directories(vault, &path, issues)?;
            has_content = true;
        } else if path.is_file() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if !is_hidden_system_entry(name) {
                has_content = true;
            }
        }
    }

    if !has_content && directory != vault.root() {
        issues.push(CleanupIssue {
            kind: "empty_directory".to_string(),
            path: directory.display().to_string(),
            description: "Leerer Ordner".to_string(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
struct CleanupIssue {
    kind: String,
    path: String,
    description: String,
}

#[derive(Debug, Clone, Serialize)]
struct CleanupSummary {
    orphaned_sidecars: usize,
    empty_directories: usize,
    stale_thumbnails: usize,
}

#[derive(Debug, Clone, Serialize)]
struct CleanupVaultResponse {
    issues: Vec<CleanupIssue>,
    summary: CleanupSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl CleanupVaultResponse {
    fn error(message: String) -> Self {
        Self {
            issues: Vec::new(),
            summary: CleanupSummary {
                orphaned_sidecars: 0,
                empty_directories: 0,
                stale_thumbnails: 0,
            },
            error: Some(message),
        }
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

    // Pre-create sorted inbox subfolders so users can drop files in the right place immediately.
    let inbox = vault.inbox_dir();
    for subfolder in INBOX_SUBFOLDERS {
        fs::create_dir_all(inbox.join(subfolder)).map_err(VaultError::from)?;
    }

    // Create app-internal cache directories up front so cover downloads and progress
    // tracking never need to check-and-mkdir at call time.
    fs::create_dir_all(vault.thumbnails_dir()).map_err(VaultError::from)?;
    fs::create_dir_all(vault.assets_dir()).map_err(VaultError::from)?;
    fs::create_dir_all(vault.progress_dir()).map_err(VaultError::from)?;

    fs::canonicalize(&vault_root).map_err(VaultError::from)
}

fn build_vault_plan(root_override: Option<&str>, refresh: bool) -> Result<DemoPlanResponse> {
    let vault_root = resolve_vault_root(root_override)?
        .ok_or_else(|| VaultError::InvalidVaultPath("no vault root found".to_string()))?;
    build_vault_plan_with_root(vault_root, refresh)
}

fn build_vault_plan_with_root(vault_root: PathBuf, refresh: bool) -> Result<DemoPlanResponse> {
    let vault = Vault::new(vault_root.clone())?;
    let files = scan_vault_files(&vault)?;
    let planner = ImportPlanner::new(ImportConfig {
        duplicate_policy: DuplicatePolicy::AskUser,
        ..ImportConfig::default()
    });
    let anilist_client = AniListClient::default();
    let mut anilist_cache = if refresh {
        HashMap::new()
    } else {
        load_anilist_cache()
    };

    let mut items = Vec::with_capacity(files.len());
    let mut anilist_results = Vec::with_capacity(files.len());
    let mut seen_fingerprints = HashSet::new();

    for file in &files {
        let mut item = planner.plan_file(&file.incoming)?;
        anilist_results.push(resolve_anilist_metadata_cached(
            &anilist_client,
            &file.incoming,
            &item,
            &mut anilist_cache,
        ));
        if is_in_inbox(&file.incoming.source_path) {
            if let Some(fingerprint) = item.fingerprint.as_ref() {
                let is_duplicate = !seen_fingerprints.insert(fingerprint.hash.clone());
                if is_duplicate {
                    item.duplicate_of = Some(fingerprint.hash.clone());
                    item.manual_review = true;
                    item.target_path = None;
                    item.steps.retain(|step| {
                        !matches!(
                            step,
                            PlannedImportStep::MoveFile { .. }
                                | PlannedImportStep::WriteSidecar { .. }
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
        }

        items.push(item);
    }

    save_anilist_cache(&anilist_cache);

    let plan = ImportPlan {
        dry_run: true,
        summary: Default::default(),
        items: items.clone(),
    };
    let summary = summarize_demo_plan(&plan);

    let mut plan_items: Vec<DemoPlanItem> = files
        .iter()
        .zip(items.iter())
        .zip(anilist_results.iter())
        .map(|((file, item), anilist_metadata)| {
            DemoPlanItem::from_scanned(
                &file.incoming,
                item,
                anilist_metadata.as_ref(),
                file.sidecar.as_ref(),
                file.sidecar_preview.as_deref(),
            )
        })
        .collect();

    group_audiobook_folders(&mut plan_items);

    // Append parts list to the sidecar YAML preview for audiobook group representatives.
    for item in &mut plan_items {
        if let Some(parts) = item.audiobook_parts.as_deref() {
            let parts_yaml = parts
                .iter()
                .enumerate()
                .map(|(i, p)| format!("  - path: \"{}\"\n    part: {}", p, i + 1))
                .collect::<Vec<_>>()
                .join("\n");
            item.sidecar_preview = format!("{}parts:\n{}\n", item.sidecar_preview, parts_yaml);
        }
    }

    Ok(DemoPlanResponse {
        title: "Vault-Dry-Run".to_string(),
        source: "vault".to_string(),
        vault_root: Some(vault_root.display().to_string()),
        note: if files.is_empty() {
            Some("Vault ist leer.".to_string())
        } else {
            None
        },
        summary,
        items: plan_items,
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
    let mut demo_cache: AniListCacheMap = HashMap::new();

    for file in &files {
        let mut item = planner
            .plan_file(file)
            .expect("demo planning should succeed");
        anilist_results.push(resolve_anilist_metadata_cached(
            &anilist_client,
            file,
            &item,
            &mut demo_cache,
        ));

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
                DemoPlanItem::from_scanned(file, item, anilist_metadata.as_ref(), None, None)
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

#[derive(Debug, Clone)]
struct ScannedVaultFile {
    incoming: IncomingFile,
    sidecar: Option<ParsedSidecar>,
    sidecar_preview: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct ParsedSidecar {
    media_type: Option<MediaType>,
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
    status: Option<MediaStatus>,
}

#[derive(Debug, Clone, Deserialize)]
struct ApplyImportRequest {
    vault_root: Option<String>,
    items: Vec<ApplyImportItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct ApplyImportItem {
    source_path: String,
    target_path: String,
    sidecar_preview: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SaveSidecarsRequest {
    vault_root: Option<String>,
    items: Vec<SaveSidecarItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct SaveSidecarItem {
    media_path: String,
    sidecar_preview: String,
}

#[derive(Debug, Clone, Serialize)]
struct ApplyImportSkipped {
    source_path: String,
    reason: String,
}

#[derive(Debug, Clone, Serialize)]
struct ApplyImportResponse {
    applied: Vec<String>,
    skipped: Vec<ApplyImportSkipped>,
    error: Option<String>,
}

impl ApplyImportResponse {
    fn error(error: String) -> Self {
        Self {
            applied: Vec::new(),
            skipped: Vec::new(),
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct SaveSidecarsResponse {
    saved: Vec<String>,
    skipped: Vec<ApplyImportSkipped>,
    error: Option<String>,
}

impl SaveSidecarsResponse {
    fn error(error: String) -> Self {
        Self {
            saved: Vec::new(),
            skipped: Vec::new(),
            error: Some(error),
        }
    }
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
    /// When this item is the group representative of a multi-file audiobook,
    /// contains the vault-relative paths of all sibling audio parts (sorted).
    #[serde(skip_serializing_if = "Option::is_none")]
    audiobook_parts: Option<Vec<String>>,
    /// True for audio files that are part of a multi-file audiobook group but
    /// are not the group representative.  The UI can collapse these.
    #[serde(default)]
    is_audiobook_part: bool,
}

impl DemoPlanItem {
    fn from_scanned(
        file: &IncomingFile,
        item: &ImportPlanItem,
        anilist: Option<&AniListAnimeMetadata>,
        sidecar: Option<&ParsedSidecar>,
        sidecar_preview: Option<&str>,
    ) -> Self {
        let classification = item
            .classification
            .as_ref()
            .or(file.classification.as_ref());
        let media_type = classification
            .map(|classification| classification.media_type)
            .or(sidecar.and_then(|value| value.media_type))
            .unwrap_or(MediaType::Unclassified);
        let needs_review = requires_review(item);
        let title = build_display_title(file, anilist, media_type);
        let anime_context = derive_anime_context(file, item, anilist, title.as_deref());
        let effective_series_title = sidecar
            .and_then(|value| value.series_title.clone())
            .or_else(|| {
                anime_context
                    .as_ref()
                    .and_then(|context| context.series_title.clone())
            });
        let effective_season_number = sidecar.and_then(|value| value.season_number).or_else(|| {
            anime_context
                .as_ref()
                .and_then(|context| context.season_number)
        });
        let effective_episode_start = sidecar.and_then(|value| value.episode_start).or_else(|| {
            anime_context
                .as_ref()
                .and_then(|context| context.episode_start)
        });
        let effective_episode_end = sidecar.and_then(|value| value.episode_end).or_else(|| {
            anime_context
                .as_ref()
                .and_then(|context| context.episode_end)
        });
        let effective_episode_title = sidecar
            .and_then(|value| value.episode_title.clone())
            .or_else(|| {
                anime_context
                    .as_ref()
                    .and_then(|context| context.episode_title.clone())
            });
        let effective_year = file
            .metadata
            .as_ref()
            .and_then(|metadata| metadata.year)
            .or_else(|| sidecar.and_then(|value| value.year));
        let collection_path = build_collection_path(
            media_type,
            title.as_deref(),
            effective_year,
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
            year: file
                .metadata
                .as_ref()
                .and_then(|metadata| metadata.year)
                .or_else(|| sidecar.and_then(|value| value.year)),
            series_title: effective_series_title,
            season_number: effective_season_number,
            episode_start: effective_episode_start,
            episode_end: effective_episode_end,
            episode_title: effective_episode_title,
            episode_count: sidecar
                .and_then(|value| value.episode_count)
                .or_else(|| anilist.and_then(|metadata| metadata.episodes)),
            runtime_minutes: sidecar
                .and_then(|value| value.runtime_minutes)
                .or_else(|| anilist.and_then(|metadata| metadata.duration)),
            average_score: sidecar
                .and_then(|value| value.average_score)
                .or_else(|| anilist.and_then(|metadata| metadata.average_score)),
            format: sidecar
                .and_then(|value| value.format.clone())
                .or_else(|| anilist.and_then(|metadata| metadata.format.clone())),
            airing_season: sidecar
                .and_then(|value| value.airing_season.clone())
                .or_else(|| anilist.and_then(|metadata| metadata.season.clone())),
            anilist_id: sidecar
                .and_then(|value| value.anilist_id)
                .or_else(|| anilist.map(|metadata| metadata.anilist_id)),
            anilist_url: sidecar
                .and_then(|value| value.anilist_url.clone())
                .or_else(|| anilist.and_then(|metadata| metadata.anilist_url.clone())),
            collection_path,
            size_bytes: file.size_bytes,
            folder_segment: media_type.folder_segment().to_string(),
            sidecar_path: sidecar_path_for(&preview_path)
                .ok()
                .map(|path| path.to_string()),
            sidecar_preview: sidecar_preview.map(ToString::to_string).unwrap_or_else(|| {
                render_sidecar_preview(file, item, media_type, anilist, anime_context.as_ref())
            }),
            steps: item.steps.iter().cloned().map(format_plan_step).collect(),
            audiobook_parts: None,
            is_audiobook_part: false,
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

    entry.source = match item
        .classification
        .as_ref()
        .map(|classification| classification.source)
    {
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
    render_sidecar_yaml(&entry).unwrap_or_else(|error| format!("---\nerror: {error}\n---\n"))
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
    if !matches!(
        media_type,
        MediaType::Anime | MediaType::HentaiAnime | MediaType::Series
    ) {
        return None;
    }

    let file_name = file
        .source_path
        .file_stem()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| file.source_path.to_string());

    // For Anime: AniList > caller hint > folder extraction > file stem.
    // For Series: folder extraction takes priority because the caller hint
    // is derived from the episode filename (e.g. "S01E02 - Title"), not the
    // series name.
    let series_title = if matches!(media_type, MediaType::Anime | MediaType::HentaiAnime) {
        anilist
            .and_then(|metadata| metadata.display_title().map(|value| value.to_string()))
            .or_else(|| series_title_hint.map(|value| value.to_string()))
            .or_else(|| extract_series_hint_from_path(&file.source_path))
            .or_else(|| extract_anime_series_hint(&file.source_path))
            .or_else(|| Some(normalize_title_candidate(&file_name)))
    } else {
        extract_series_hint_from_path(&file.source_path)
            .or_else(|| extract_anime_series_hint(&file.source_path))
            .or_else(|| Some(normalize_title_candidate(&file_name)))
    };

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
    year: Option<u16>,
    anime_context: Option<&AnimeEpisodeContext>,
    anilist: Option<&AniListAnimeMetadata>,
) -> String {
    // Append "(year)" suffix when a year is known — helps distinguish remakes and re-releases.
    let year_suffix =
        |y: Option<u16>| -> String { y.map(|y| format!(" ({y})")).unwrap_or_default() };

    match media_type {
        MediaType::Anime | MediaType::HentaiAnime => {
            if is_anilist_movie(anilist) {
                let t = sanitize_path_segment(title.unwrap_or("Unbenannt"));
                let y =
                    year_suffix(year.or_else(|| anilist.and_then(|a| a.start_date.as_ref()?.year)));
                return format!("Anime/Filme/{t}{y}");
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
        MediaType::Series => {
            let series = anime_context
                .and_then(|context| context.series_title.as_deref())
                .or(title)
                .unwrap_or("Unbekannte Serie");
            let season_number = anime_context
                .and_then(|context| context.season_number)
                .unwrap_or(1);
            format!(
                "Serien/{}/Staffel {}",
                sanitize_path_segment(series),
                season_number
            )
        }
        MediaType::Film => {
            let t = sanitize_path_segment(title.unwrap_or("Unbenannt"));
            let y = year_suffix(year);
            format!("Filme/{t}{y}")
        }
        // Books and Ebooks: Bücher/<Title (Year)>/  — Author subfolder added once OpenLibrary
        // metadata is available.
        MediaType::Book | MediaType::Ebook => {
            let t = sanitize_path_segment(title.unwrap_or("Unbenannt"));
            let y = year_suffix(year);
            format!("Bücher/{t}{y}")
        }
        MediaType::Audiobook => {
            let t = sanitize_path_segment(title.unwrap_or("Unbenannt"));
            let y = year_suffix(year);
            format!("Hörbücher/{t}{y}")
        }
        // Music: Musik/<Title (Year)>/  — Artist subfolder added once MusicBrainz metadata is
        // available.
        MediaType::MusicAlbum | MediaType::MusicTrack => {
            let t = sanitize_path_segment(title.unwrap_or("Unbenannt"));
            let y = year_suffix(year);
            format!("Musik/{t}{y}")
        }
        _ => {
            let folder = media_type.folder_segment();
            let t = sanitize_path_segment(title.unwrap_or("Unbenannt"));
            let y = year_suffix(year);
            format!("{folder}/{t}{y}")
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
    let is_anime = matches!(media_type, MediaType::Anime | MediaType::HentaiAnime);
    let is_series_with_context = media_type == MediaType::Series && anime_context.is_some();

    if !is_anime && !is_series_with_context {
        return item.target_path.as_ref().map(|path| path.to_string());
    }

    if is_anime && is_anilist_movie(anilist) {
        let movie_title = sanitize_path_segment(title.unwrap_or("Unbenannt"));
        let year = file
            .metadata
            .as_ref()
            .and_then(|m| m.year)
            .or_else(|| anilist.and_then(|a| a.start_date.as_ref()?.year));
        let year_suffix = year.map(|y| format!(" ({y})")).unwrap_or_default();
        let folder_name = format!("{movie_title}{year_suffix}");
        let extension = file
            .source_path
            .extension()
            .map(|ext| ext.to_string_lossy().to_string());
        let mut path = PathBuf::from("Anime");
        path.push("Filme");
        path.push(&folder_name);
        let file_name = match extension {
            Some(extension) if !extension.is_empty() => format!("{folder_name}.{extension}"),
            _ => folder_name,
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

    // Anime → "Anime/Serien/…",  TV Series → "Serien/…"
    let mut path = if is_anime {
        let mut p = PathBuf::from("Anime");
        p.push("Serien");
        p
    } else {
        PathBuf::from("Serien")
    };
    path.push(sanitize_path_segment(series_title));
    path.push(format!("Staffel {season_number}"));
    let file_name = match extension {
        Some(extension) if !extension.is_empty() => {
            format!("{}.{}", sanitize_path_segment(&episode_label), extension)
        }
        _ => sanitize_path_segment(&episode_label),
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
    let season = context.season_number.unwrap_or(1);
    match (context.episode_start, context.episode_end) {
        (Some(start), Some(end)) if start != end => {
            let label = format!("S{season:02}E{start:02}-E{end:02}");
            Some(match context.episode_title.as_deref() {
                Some(title) if !title.trim().is_empty() => {
                    format!("{label} - {}", sanitize_path_segment(title))
                }
                _ => label,
            })
        }
        (Some(start), _) => {
            let label = format!("S{season:02}E{start:02}");
            Some(match context.episode_title.as_deref() {
                Some(title) if !title.trim().is_empty() => {
                    format!("{label} - {}", sanitize_path_segment(title))
                }
                _ => label,
            })
        }
        _ => context
            .episode_title
            .as_deref()
            .map(sanitize_path_segment)
            .filter(|title| !title.is_empty()),
    }
}

fn resolve_anilist_metadata_cached(
    client: &AniListClient,
    file: &IncomingFile,
    item: &ImportPlanItem,
    cache: &mut AniListCacheMap,
) -> Option<AniListAnimeMetadata> {
    let classification = item
        .classification
        .as_ref()
        .or(file.classification.as_ref())?;
    if !should_attempt_anilist(classification.media_type, &file.source_path) {
        return None;
    }

    let search_title = build_anime_search_title(file)?;
    let cache_key = search_title.to_lowercase();

    if let Some(cached) = cache.get(&cache_key) {
        return Some(cached.clone());
    }

    let result = client
        .search_anime(
            &search_title,
            AniListClient::adult_flag_for(classification.media_type),
        )
        .ok()
        .flatten();

    if let Some(ref metadata) = result {
        cache.insert(cache_key, metadata.clone());
    }

    result
}

fn should_attempt_anilist(media_type: MediaType, source_path: &RelativePath) -> bool {
    if matches!(media_type, MediaType::Anime | MediaType::HentaiAnime) {
        return true;
    }

    source_path.to_string().to_lowercase().contains("anime")
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
            || matches!(character, '+' | '-' | 'e' | 'p' | 's' | 'x' | ' ' | '.')
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
        || item.steps.iter().any(|step| {
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
            return urlencoding::decode(value)
                .ok()
                .map(|value| value.into_owned());
        }
    }

    None
}

fn media_content_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        // Images
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("bmp") => "image/bmp",
        Some("svg") => "image/svg+xml",
        Some("avif") => "image/avif",
        Some("tif" | "tiff") => "image/tiff",
        // Video (natively supported by macOS WKWebView)
        Some("mp4" | "m4v") => "video/mp4",
        Some("mov") => "video/quicktime",
        Some("webm") => "video/webm",
        Some("ogv") => "video/ogg",
        Some("avi") => "video/x-msvideo",
        Some("mkv") => "video/x-matroska",
        // Audio
        Some("mp3") => "audio/mpeg",
        Some("m4a" | "m4b") => "audio/mp4",
        Some("aac") => "audio/aac",
        Some("ogg" | "oga") => "audio/ogg",
        Some("opus") => "audio/opus",
        Some("flac") => "audio/flac",
        Some("wav") => "audio/wav",
        Some("weba") => "audio/webm",
        // Documents
        Some("pdf") => "application/pdf",
        Some("epub") => "application/epub+zip",
        _ => "application/octet-stream",
    }
}

fn apply_import_item(vault: &Vault, item: &ApplyImportItem) -> Result<()> {
    let source_relative = RelativePath::new(&item.source_path)?;
    let target_relative = RelativePath::new(&item.target_path)?;

    let source_absolute = vault.resolve(source_relative.as_path())?;
    let target_absolute = vault.resolve(target_relative.as_path())?;

    if !source_absolute.exists() {
        return Err(VaultError::InvalidVaultPath(format!(
            "Quelldatei nicht gefunden: {}",
            source_relative
        )));
    }

    let target_parent = target_absolute.parent().ok_or_else(|| {
        VaultError::InvalidVaultPath(format!(
            "Zielpfad hat keinen Elternordner: {}",
            target_relative
        ))
    })?;
    fs::create_dir_all(target_parent).map_err(VaultError::from)?;

    if source_absolute != target_absolute {
        if target_absolute.exists() {
            return Err(VaultError::InvalidVaultPath(format!(
                "Zieldatei existiert bereits: {}",
                target_relative
            )));
        }
        move_file_with_fallback(&source_absolute, &target_absolute)?;
    }

    write_sidecar_preview(vault, &target_relative, &item.sidecar_preview)?;

    prune_empty_inbox_dirs(vault, source_absolute.parent());
    Ok(())
}

fn save_sidecar_item(vault: &Vault, item: &SaveSidecarItem) -> Result<()> {
    let media_relative = RelativePath::new(&item.media_path)?;
    let media_absolute = vault.resolve(media_relative.as_path())?;

    if !media_absolute.exists() {
        return Err(VaultError::InvalidVaultPath(format!(
            "Datei nicht gefunden: {}",
            media_relative
        )));
    }

    write_sidecar_preview(vault, &media_relative, &item.sidecar_preview)
}

fn write_sidecar_preview(
    vault: &Vault,
    media_relative: &RelativePath,
    sidecar_preview: &str,
) -> Result<()> {
    let sidecar_relative = sidecar_path_for(media_relative)?;
    let sidecar_absolute = vault.resolve(sidecar_relative.as_path())?;
    let sidecar_parent = sidecar_absolute.parent().ok_or_else(|| {
        VaultError::InvalidVaultPath(format!(
            "Sidecar-Pfad hat keinen Elternordner: {}",
            sidecar_relative
        ))
    })?;
    fs::create_dir_all(sidecar_parent).map_err(VaultError::from)?;
    fs::write(&sidecar_absolute, sidecar_preview.as_bytes()).map_err(VaultError::from)?;

    let mut legacy_relative = media_relative.to_path_buf();
    legacy_relative.set_extension("mediashelf.yaml");
    let legacy_absolute = vault.resolve(legacy_relative)?;
    if legacy_absolute.exists() {
        fs::remove_file(legacy_absolute).map_err(VaultError::from)?;
    }

    Ok(())
}

fn move_file_with_fallback(source: &Path, target: &Path) -> Result<()> {
    match fs::rename(source, target) {
        Ok(()) => Ok(()),
        Err(error) if is_cross_device_error(&error) => {
            fs::copy(source, target).map_err(VaultError::from)?;
            fs::remove_file(source).map_err(VaultError::from)?;
            Ok(())
        }
        Err(error) => Err(VaultError::from(error)),
    }
}

fn is_cross_device_error(error: &std::io::Error) -> bool {
    const EXDEV_OS_ERROR: i32 = 18;
    error.raw_os_error() == Some(EXDEV_OS_ERROR)
}

fn prune_empty_inbox_dirs(vault: &Vault, start: Option<&Path>) {
    let inbox_root = vault.inbox_dir();
    let mut current = start.map(Path::to_path_buf);

    while let Some(path) = current {
        if path == inbox_root || path == vault.root() {
            break;
        }

        let can_remove = match fs::read_dir(&path) {
            Ok(mut entries) => entries.next().is_none(),
            Err(_) => false,
        };

        if !can_remove {
            break;
        }

        let parent = path.parent().map(Path::to_path_buf);
        if fs::remove_dir(&path).is_err() {
            break;
        }
        current = parent;
    }
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
    path.join("Inbox").is_dir()
        || path.join(".mediavault").is_dir()
        || path.join(LEGACY_SYSTEM_DIR).is_dir()
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

fn scan_vault_files(vault: &Vault) -> Result<Vec<ScannedVaultFile>> {
    let mut files = Vec::new();
    scan_directory(vault, vault.root(), &mut files)?;
    Ok(files)
}

fn scan_directory(
    vault: &Vault,
    directory: &Path,
    files: &mut Vec<ScannedVaultFile>,
) -> Result<()> {
    for entry in fs::read_dir(directory).map_err(VaultError::from)? {
        let entry = entry.map_err(VaultError::from)?;
        let path = entry.path();

        let metadata = entry.metadata().map_err(VaultError::from)?;

        if metadata.is_dir() {
            if should_skip_scanned_directory(vault, &path) {
                continue;
            }
            scan_directory(vault, &path, files)?;
            continue;
        }

        if !metadata.is_file() {
            continue;
        }

        if should_skip_scanned_file(&path) {
            continue;
        }

        let relative_path = vault.relative_from_absolute(&path)?;
        let sidecar_file = find_sidecar_file(vault, &relative_path)?;
        let sidecar_preview = sidecar_file
            .as_ref()
            .map(|path| fs::read_to_string(path).map_err(VaultError::from))
            .transpose()?;
        let parsed_sidecar = sidecar_preview
            .as_deref()
            .map(parse_sidecar_metadata)
            .transpose()?;
        let parsed_nfo = find_nfo_companion(&path)
            .map(|nfo_path| {
                fs::read_to_string(&nfo_path)
                    .map(|content| parse_nfo_metadata(&content))
                    .map_err(VaultError::from)
            })
            .transpose()?;
        // NFO data fills any gap not already covered by the .mediavault.yaml sidecar.
        let effective_sidecar = merge_nfo_into_sidecar(parsed_sidecar, parsed_nfo.as_ref());
        let fingerprint = if is_in_inbox(&relative_path) {
            Some(compute_fingerprint_for_file(&path)?)
        } else {
            None
        };
        let classification = effective_sidecar
            .as_ref()
            .and_then(classification_from_sidecar)
            .or_else(|| detect_classification(&relative_path));
        let resolved_title = effective_sidecar
            .as_ref()
            .and_then(|sidecar| {
                sidecar
                    .series_title
                    .clone()
                    .or_else(|| sidecar.title.clone())
            })
            .or_else(|| {
                relative_path
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().to_string())
            });
        let resolved_metadata = Some(ResolvedMetadata {
            title: resolved_title,
            year: effective_sidecar.as_ref().and_then(|sidecar| sidecar.year),
        });

        files.push(ScannedVaultFile {
            incoming: IncomingFile {
                source_path: relative_path,
                size_bytes: metadata.len(),
                fingerprint,
                classification,
                metadata: resolved_metadata,
            },
            sidecar: effective_sidecar,
            sidecar_preview,
        });
    }

    Ok(())
}

fn should_skip_scanned_directory(vault: &Vault, path: &Path) -> bool {
    if path == vault.system_dir() || path == vault.root().join(LEGACY_SYSTEM_DIR) {
        return true;
    }

    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    name.eq_ignore_ascii_case("_review_queue") || is_hidden_system_entry(name)
}

fn should_skip_scanned_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    is_hidden_system_entry(name)
        || name.ends_with(".mediavault.yaml")
        || name.ends_with(LEGACY_SIDECAR_SUFFIX)
        // NFO files are metadata companions, not independent media entries.
        || name.to_lowercase().ends_with(".nfo")
}

/// Audio file extensions that qualify a file to be counted as an audiobook part.
const AUDIOBOOK_AUDIO_EXTS: &[&str] = &[
    "mp3", "m4a", "m4b", "aac", "ogg", "opus", "flac", "wav", "weba",
];

/// Post-processes a flat list of plan items to detect multi-file audiobook groups.
///
/// Rule: if every audio file in a directory is classified as `Audiobook`, they
/// are treated as parts of a single audiobook.  The lexicographically first
/// item becomes the group representative and receives `audiobook_parts`; the
/// others are flagged as `is_audiobook_part = true`.
///
/// Items that don't belong to a multi-file group are left unchanged.
fn group_audiobook_folders(items: &mut Vec<DemoPlanItem>) {
    // Group item indices by parent directory, counting only Audiobook items.
    let mut dir_groups: HashMap<String, Vec<usize>> = HashMap::new();

    for (idx, item) in items.iter().enumerate() {
        if item.media_type != MediaType::Audiobook.to_string() {
            continue;
        }
        let ext = item
            .source_path
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        if !AUDIOBOOK_AUDIO_EXTS.contains(&ext.as_str()) {
            continue;
        }
        // Parent directory = everything before the last '/'
        let parent = item
            .source_path
            .rfind('/')
            .map(|idx| item.source_path[..idx].to_string())
            .unwrap_or_default();
        dir_groups.entry(parent).or_default().push(idx);
    }

    for (_dir, mut group_indices) in dir_groups {
        if group_indices.len() < 2 {
            continue; // Single file — not a multi-part audiobook
        }

        // Sort indices by source_path so parts are in alphabetical (track) order.
        group_indices.sort_by(|&a, &b| items[a].source_path.cmp(&items[b].source_path));

        let part_paths: Vec<String> = group_indices
            .iter()
            .map(|&idx| items[idx].source_path.clone())
            .collect();

        let representative = group_indices[0];
        items[representative].audiobook_parts = Some(part_paths);

        for &part_idx in &group_indices[1..] {
            items[part_idx].is_audiobook_part = true;
        }
    }
}

/// Looks for a Kodi/XBMC-style `.nfo` companion file alongside a media file.
///
/// Checks in priority order:
/// 1. Same filename with `.nfo` extension (e.g. `Movie.mkv` → `Movie.nfo`)
/// 2. `movie.nfo` in the same directory (common Kodi convention)
fn find_nfo_companion(media_path: &Path) -> Option<PathBuf> {
    let mut nfo_path = media_path.to_path_buf();
    nfo_path.set_extension("nfo");
    if nfo_path.exists() {
        return Some(nfo_path);
    }
    let movie_nfo = media_path.parent()?.join("movie.nfo");
    if movie_nfo.exists() {
        Some(movie_nfo)
    } else {
        None
    }
}

/// Metadata extracted from a Kodi/XBMC NFO companion file.
#[derive(Debug, Default)]
struct ParsedNfoMetadata {
    title: Option<String>,
    /// For episode NFOs: the show title (`<showtitle>`).
    series_title: Option<String>,
    year: Option<u16>,
    season_number: Option<u16>,
    episode_start: Option<u16>,
}

/// Parses a subset of the Kodi NFO XML format without a full XML parser.
///
/// Only extracts fields that map to existing `ParsedSidecar` slots; everything
/// else is ignored.  The NFO format is well-defined enough that tag-based
/// substring search is reliable here.
fn parse_nfo_metadata(content: &str) -> ParsedNfoMetadata {
    ParsedNfoMetadata {
        title: extract_xml_tag(content, "title"),
        series_title: extract_xml_tag(content, "showtitle"),
        year: extract_xml_tag(content, "year").and_then(|s| s.parse::<u16>().ok()),
        season_number: extract_xml_tag(content, "season").and_then(|s| s.parse::<u16>().ok()),
        episode_start: extract_xml_tag(content, "episode").and_then(|s| s.parse::<u16>().ok()),
    }
}

/// Extracts the text content of the first matching XML tag.
fn extract_xml_tag(content: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = content.find(&open)? + open.len();
    let end = content[start..].find(&close).map(|i| start + i)?;
    let value = content[start..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

/// Merges NFO companion data into a sidecar, with NFO only filling absent fields.
///
/// The `.mediavault.yaml` sidecar always wins; NFO data only fills gaps.
fn merge_nfo_into_sidecar(
    sidecar: Option<ParsedSidecar>,
    nfo: Option<&ParsedNfoMetadata>,
) -> Option<ParsedSidecar> {
    let Some(nfo) = nfo else {
        return sidecar;
    };
    let mut result = sidecar.unwrap_or_default();
    if result.title.is_none() {
        result.title = nfo.title.clone();
    }
    if result.series_title.is_none() {
        result.series_title = nfo.series_title.clone();
    }
    if result.year.is_none() {
        result.year = nfo.year;
    }
    if result.season_number.is_none() {
        result.season_number = nfo.season_number;
    }
    if result.episode_start.is_none() {
        result.episode_start = nfo.episode_start;
    }
    Some(result)
}

fn find_sidecar_file(vault: &Vault, media_path: &RelativePath) -> Result<Option<PathBuf>> {
    let current = vault.resolve(sidecar_path_for(media_path)?.as_path())?;
    if current.exists() {
        return Ok(Some(current));
    }

    let mut legacy_relative = media_path.to_path_buf();
    legacy_relative.set_extension("mediashelf.yaml");
    let legacy = vault.resolve(legacy_relative)?;
    if legacy.exists() {
        return Ok(Some(legacy));
    }

    Ok(None)
}

fn classification_from_sidecar(sidecar: &ParsedSidecar) -> Option<FileClassification> {
    sidecar.media_type.map(|media_type| FileClassification {
        media_type,
        confidence: 0.99,
        source: ClassificationSource::User,
    })
}

fn parse_sidecar_metadata(raw: &str) -> Result<ParsedSidecar> {
    let mut sidecar = ParsedSidecar::default();
    let mut lines = raw.lines();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "---" {
            continue;
        }

        let Some((key, value)) = trimmed.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        match key {
            "media_type" => sidecar.media_type = parse_media_type(unquote_yaml(value)),
            "title" => sidecar.title = Some(unquote_yaml(value)),
            "year" => sidecar.year = value.parse::<u16>().ok(),
            "series_title" => sidecar.series_title = Some(unquote_yaml(value)),
            "season_number" => sidecar.season_number = value.parse::<u16>().ok(),
            "episode_start" => sidecar.episode_start = value.parse::<u16>().ok(),
            "episode_end" => sidecar.episode_end = value.parse::<u16>().ok(),
            "episode_title" => sidecar.episode_title = Some(unquote_yaml(value)),
            "episode_count" => sidecar.episode_count = value.parse::<u16>().ok(),
            "runtime_minutes" => sidecar.runtime_minutes = value.parse::<u16>().ok(),
            "average_score" => sidecar.average_score = value.parse::<f32>().ok(),
            "format" => sidecar.format = Some(unquote_yaml(value)),
            "airing_season" => sidecar.airing_season = Some(unquote_yaml(value)),
            "anilist_id" => sidecar.anilist_id = value.parse::<u32>().ok(),
            "anilist_url" => sidecar.anilist_url = Some(unquote_yaml(value)),
            "status" => sidecar.status = parse_media_status(unquote_yaml(value).as_str()),
            _ => {}
        }
    }

    Ok(sidecar)
}

fn unquote_yaml(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let is_double = trimmed.starts_with('"') && trimmed.ends_with('"');
        let is_single = trimmed.starts_with('\'') && trimmed.ends_with('\'');
        if is_double || is_single {
            return trimmed[1..trimmed.len() - 1]
                .replace("\\n", "\n")
                .replace("\\\"", "\"")
                .replace("\\'", "'")
                .replace("\\\\", "\\");
        }
    }
    trimmed.to_string()
}

fn parse_media_type(value: String) -> Option<MediaType> {
    match value.trim().to_lowercase().as_str() {
        "film" => Some(MediaType::Film),
        "series" => Some(MediaType::Series),
        "anime" => Some(MediaType::Anime),
        "hentai-anime" => Some(MediaType::HentaiAnime),
        "book" => Some(MediaType::Book),
        "ebook" => Some(MediaType::Ebook),
        "comic" => Some(MediaType::Comic),
        "manga" => Some(MediaType::Manga),
        "music-album" => Some(MediaType::MusicAlbum),
        "music-track" => Some(MediaType::MusicTrack),
        "podcast" => Some(MediaType::Podcast),
        "audiobook" => Some(MediaType::Audiobook),
        "video-game" => Some(MediaType::VideoGame),
        "document" => Some(MediaType::Document),
        "photo" => Some(MediaType::Photo),
        "video-misc" => Some(MediaType::VideoMisc),
        "archive" => Some(MediaType::Archive),
        "image" => Some(MediaType::Image),
        "software" => Some(MediaType::Software),
        "3d-model" => Some(MediaType::Model3D),
        "unclassified" => Some(MediaType::Unclassified),
        _ => None,
    }
}

fn parse_media_status(value: &str) -> Option<MediaStatus> {
    match value.trim().to_lowercase().as_str() {
        "inbox" => Some(MediaStatus::Inbox),
        "needs-review" => Some(MediaStatus::NeedsReview),
        "in-library" => Some(MediaStatus::InLibrary),
        "wishlist" => Some(MediaStatus::Wishlist),
        "completed" => Some(MediaStatus::Completed),
        "on-hold" => Some(MediaStatus::OnHold),
        "archived" => Some(MediaStatus::Archived),
        "ignored" => Some(MediaStatus::Ignored),
        _ => None,
    }
}

fn classify_from_inbox_folder(path: &str) -> Option<FileClassification> {
    let make = |media_type: MediaType| FileClassification {
        media_type,
        confidence: 0.96,
        source: ClassificationSource::Folder,
    };
    // These paths reflect the pre-created INBOX_SUBFOLDERS — the user intentionally
    // placed the file there, so treat the folder as a strong signal.
    if path.starts_with("inbox/anime/tv/") || path.starts_with("inbox/anime/serien/") {
        return Some(make(MediaType::Anime));
    }
    if path.starts_with("inbox/anime/") {
        return Some(make(MediaType::Anime));
    }
    if path.starts_with("inbox/serien/") {
        return Some(make(MediaType::Series));
    }
    if path.starts_with("inbox/filme/") {
        return Some(make(MediaType::Film));
    }
    if path.starts_with("inbox/musik/") {
        return Some(make(MediaType::MusicAlbum));
    }
    if path.starts_with("inbox/bücher/") {
        return Some(make(MediaType::Book));
    }
    if path.starts_with("inbox/hörbücher/") {
        return Some(make(MediaType::Audiobook));
    }
    if path.starts_with("inbox/manga/") {
        return Some(make(MediaType::Manga));
    }
    if path.starts_with("inbox/comics/") {
        return Some(make(MediaType::Comic));
    }
    if path.starts_with("inbox/ttrpg/") {
        return Some(make(MediaType::RPG));
    }
    if path.starts_with("inbox/games/") {
        return Some(make(MediaType::VideoGame));
    }
    if path.starts_with("inbox/unsortiert/") {
        return Some(FileClassification {
            media_type: MediaType::Unclassified,
            confidence: 0.50,
            source: ClassificationSource::Folder,
        });
    }
    None
}

fn detect_classification(relative_path: &RelativePath) -> Option<FileClassification> {
    let path = relative_path.to_string().to_lowercase();

    // Inbox subfolder takes highest priority — the user explicitly sorted it there.
    if let Some(cls) = classify_from_inbox_folder(&path) {
        return Some(cls);
    }

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
    "dcim", "camera", "photo", "photos", "picture", "pictures", "img_", "dsc",
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

fn is_in_inbox(relative_path: &RelativePath) -> bool {
    relative_path
        .as_path()
        .components()
        .next()
        .map(|component| component.as_os_str() == "Inbox")
        .unwrap_or(false)
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

// ---------------------------------------------------------------------------
// Progress API
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct SaveProgressRequest {
    vault_root: Option<String>,
    vault_path: String,
    progress: MediaProgress,
    #[serde(default)]
    completed: bool,
}

#[derive(Serialize)]
struct SaveProgressResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl SaveProgressResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

#[derive(Serialize)]
struct LoadProgressResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    record: Option<ProgressRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Deserialize)]
struct DeleteProgressRequest {
    vault_root: Option<String>,
    vault_path: String,
}

#[derive(Serialize)]
struct DeleteProgressResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl DeleteProgressResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

#[derive(Serialize)]
struct ListProgressResponse {
    records: Vec<ProgressRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn build_save_progress_response(body: &[u8]) -> SaveProgressResponse {
    let req: SaveProgressRequest = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => return SaveProgressResponse::error(format!("Invalid request: {e}")),
    };

    let vault_root = match resolve_vault_root(req.vault_root.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => return SaveProgressResponse::error("Kein Vault geöffnet."),
        Err(e) => return SaveProgressResponse::error(e.to_string()),
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => return SaveProgressResponse::error(e.to_string()),
    };

    match save_progress(
        &vault.progress_dir(),
        &req.vault_path,
        req.progress,
        req.completed,
    ) {
        Ok(()) => SaveProgressResponse::ok(),
        Err(e) => SaveProgressResponse::error(e.to_string()),
    }
}

fn build_load_progress_response(query: Option<&str>) -> LoadProgressResponse {
    let query = match query {
        Some(q) => q,
        None => {
            return LoadProgressResponse {
                record: None,
                error: Some("missing query".into()),
            }
        }
    };

    let vault_path = match extract_query_value(query, "path") {
        Some(p) => p,
        None => {
            return LoadProgressResponse {
                record: None,
                error: Some("missing path".into()),
            }
        }
    };

    let root_override = extract_query_value(query, "root");
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => {
            return LoadProgressResponse {
                record: None,
                error: Some("Kein Vault geöffnet.".into()),
            }
        }
        Err(e) => {
            return LoadProgressResponse {
                record: None,
                error: Some(e.to_string()),
            }
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => {
            return LoadProgressResponse {
                record: None,
                error: Some(e.to_string()),
            }
        }
    };

    match load_progress(&vault.progress_dir(), &vault_path) {
        Ok(record) => LoadProgressResponse {
            record,
            error: None,
        },
        Err(e) => LoadProgressResponse {
            record: None,
            error: Some(e.to_string()),
        },
    }
}

fn build_delete_progress_response(body: &[u8]) -> DeleteProgressResponse {
    let req: DeleteProgressRequest = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => return DeleteProgressResponse::error(format!("Invalid request: {e}")),
    };

    let vault_root = match resolve_vault_root(req.vault_root.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => return DeleteProgressResponse::error("Kein Vault geöffnet."),
        Err(e) => return DeleteProgressResponse::error(e.to_string()),
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => return DeleteProgressResponse::error(e.to_string()),
    };

    match delete_progress(&vault.progress_dir(), &req.vault_path) {
        Ok(()) => DeleteProgressResponse::ok(),
        Err(e) => DeleteProgressResponse::error(e.to_string()),
    }
}

fn build_list_progress_response(query: Option<&str>) -> ListProgressResponse {
    let root_override = query.and_then(|q| extract_query_value(q, "root"));
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => {
            return ListProgressResponse {
                records: vec![],
                error: Some("Kein Vault geöffnet.".into()),
            }
        }
        Err(e) => {
            return ListProgressResponse {
                records: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => {
            return ListProgressResponse {
                records: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    match list_in_progress(&vault.progress_dir()) {
        Ok(records) => ListProgressResponse {
            records,
            error: None,
        },
        Err(e) => ListProgressResponse {
            records: vec![],
            error: Some(e.to_string()),
        },
    }
}

// ---------------------------------------------------------------------------
// Open-with-system API
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct OpenExternalResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl OpenExternalResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

fn build_open_external_response(query: Option<&str>) -> OpenExternalResponse {
    let query = match query {
        Some(q) => q,
        None => return OpenExternalResponse::error("missing query"),
    };

    let path = match extract_query_value(query, "path") {
        Some(p) => p,
        None => return OpenExternalResponse::error("missing path"),
    };

    let root_override = extract_query_value(query, "root");
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => return OpenExternalResponse::error("Kein Vault geöffnet."),
        Err(e) => return OpenExternalResponse::error(e.to_string()),
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => return OpenExternalResponse::error(e.to_string()),
    };

    let relative = match RelativePath::new(&path) {
        Ok(r) => r,
        Err(e) => return OpenExternalResponse::error(e.to_string()),
    };

    let absolute = match vault.resolve(relative.as_path()) {
        Ok(p) => p,
        Err(e) => return OpenExternalResponse::error(e.to_string()),
    };

    // `open` on macOS launches the file with the default app; equivalent to
    // double-clicking in Finder. This is fire-and-forget — we only care that
    // the process started, not how it exits.
    match std::process::Command::new("open").arg(&absolute).spawn() {
        Ok(_) => OpenExternalResponse::ok(),
        Err(e) => OpenExternalResponse::error(format!("open failed: {e}")),
    }
}

// ---------------------------------------------------------------------------
// Dashboard APIs
// ---------------------------------------------------------------------------

/// A lightweight item descriptor for dashboard cards.
#[derive(Serialize)]
struct DashboardItem {
    vault_path: String,
    title: String,
    media_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position_seconds: Option<f64>,
    /// File modification time as UNIX seconds.
    modified_at: u64,
}

#[derive(Serialize)]
struct RecentItemsResponse {
    items: Vec<DashboardItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct InProgressResponse {
    items: Vec<DashboardItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

const DASHBOARD_LIMIT: usize = 24;

fn build_recent_items_response(query: Option<&str>) -> RecentItemsResponse {
    let root_override = query.and_then(|q| extract_query_value(q, "root"));
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => {
            return RecentItemsResponse {
                items: vec![],
                error: Some("Kein Vault geöffnet.".into()),
            }
        }
        Err(e) => {
            return RecentItemsResponse {
                items: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    let vault = match Vault::new(&vault_root) {
        Ok(v) => v,
        Err(e) => {
            return RecentItemsResponse {
                items: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    let scanned = match scan_vault_files(&vault) {
        Ok(files) => files,
        Err(e) => {
            return RecentItemsResponse {
                items: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    // Sort by file system modification time, newest first.
    let mut with_mtime: Vec<(u64, ScannedVaultFile)> = scanned
        .into_iter()
        .filter_map(|file| {
            let abs = vault.resolve(file.incoming.source_path.as_path()).ok()?;
            let mtime = fs::metadata(&abs)
                .ok()?
                .modified()
                .ok()?
                .duration_since(std::time::UNIX_EPOCH)
                .ok()?
                .as_secs();
            Some((mtime, file))
        })
        .collect();

    with_mtime.sort_by(|a, b| b.0.cmp(&a.0));

    let items = with_mtime
        .into_iter()
        .take(DASHBOARD_LIMIT)
        .map(|(mtime, file)| {
            let cls = file.incoming.classification.as_ref();
            let media_type = cls
                .map(|c| c.media_type.to_string())
                .unwrap_or_else(|| "unclassified".to_string());
            let title = file
                .sidecar
                .as_ref()
                .and_then(|s| s.title.clone().or_else(|| s.series_title.clone()))
                .or_else(|| {
                    file.incoming
                        .source_path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                })
                .unwrap_or_else(|| file.incoming.source_path.to_string());
            let year = file
                .incoming
                .metadata
                .as_ref()
                .and_then(|m| m.year)
                .or_else(|| file.sidecar.as_ref().and_then(|s| s.year));
            DashboardItem {
                vault_path: file.incoming.source_path.to_string(),
                title,
                media_type,
                year,
                cover_url: None,
                progress_fraction: None,
                position_seconds: None,
                modified_at: mtime,
            }
        })
        .collect();

    RecentItemsResponse { items, error: None }
}

fn build_in_progress_response(query: Option<&str>) -> InProgressResponse {
    let root_override = query.and_then(|q| extract_query_value(q, "root"));
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => {
            return InProgressResponse {
                items: vec![],
                error: Some("Kein Vault geöffnet.".into()),
            }
        }
        Err(e) => {
            return InProgressResponse {
                items: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => {
            return InProgressResponse {
                items: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    let records = match list_in_progress(&vault.progress_dir()) {
        Ok(r) => r,
        Err(e) => {
            return InProgressResponse {
                items: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    let items = records
        .into_iter()
        .take(DASHBOARD_LIMIT)
        .map(|record| {
            let title = record
                .vault_path
                .rsplit('/')
                .next()
                .and_then(|name| name.rsplit('.').nth(1).map(|_| name.to_string()))
                .unwrap_or_else(|| record.vault_path.clone());
            let position_seconds = match &record.progress {
                MediaProgress::Video {
                    position_seconds, ..
                }
                | MediaProgress::Audio {
                    position_seconds, ..
                } => Some(*position_seconds),
                MediaProgress::Audiobook {
                    position_seconds, ..
                } => Some(*position_seconds),
                _ => None,
            };
            let modified_at = record.last_accessed;
            DashboardItem {
                vault_path: record.vault_path.clone(),
                title,
                media_type: "unknown".to_string(),
                year: None,
                cover_url: None,
                progress_fraction: record.fraction(),
                position_seconds,
                modified_at,
            }
        })
        .collect();

    InProgressResponse { items, error: None }
}

// ---------------------------------------------------------------------------
// Playlist APIs
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct ListPlaylistsResponse {
    playlists: Vec<Playlist>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct GetPlaylistResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    playlist: Option<Playlist>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Deserialize)]
struct SavePlaylistRequest {
    vault_root: Option<String>,
    playlist: Playlist,
}

#[derive(Serialize)]
struct SavePlaylistResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl SavePlaylistResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

#[derive(Deserialize)]
struct DeletePlaylistRequest {
    vault_root: Option<String>,
    id: String,
}

#[derive(Serialize)]
struct DeletePlaylistResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl DeletePlaylistResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

#[derive(Deserialize)]
struct SaveCursorRequest {
    vault_root: Option<String>,
    cursor: PlaylistCursor,
}

#[derive(Serialize)]
struct SaveCursorResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl SaveCursorResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

#[derive(Serialize)]
struct LoadCursorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<PlaylistCursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn build_list_playlists_response(query: Option<&str>) -> ListPlaylistsResponse {
    let root_override = query.and_then(|q| extract_query_value(q, "root"));
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => {
            return ListPlaylistsResponse {
                playlists: vec![],
                error: Some("Kein Vault geöffnet.".into()),
            }
        }
        Err(e) => {
            return ListPlaylistsResponse {
                playlists: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => {
            return ListPlaylistsResponse {
                playlists: vec![],
                error: Some(e.to_string()),
            }
        }
    };

    match list_playlists(&vault.system_dir()) {
        Ok(playlists) => ListPlaylistsResponse {
            playlists,
            error: None,
        },
        Err(e) => ListPlaylistsResponse {
            playlists: vec![],
            error: Some(e.to_string()),
        },
    }
}

fn build_get_playlist_response(query: Option<&str>) -> GetPlaylistResponse {
    let query = match query {
        Some(q) => q,
        None => {
            return GetPlaylistResponse {
                playlist: None,
                error: Some("missing query".into()),
            }
        }
    };

    let id = match extract_query_value(query, "id") {
        Some(id) => id,
        None => {
            return GetPlaylistResponse {
                playlist: None,
                error: Some("missing id".into()),
            }
        }
    };

    let root_override = extract_query_value(query, "root");
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => {
            return GetPlaylistResponse {
                playlist: None,
                error: Some("Kein Vault geöffnet.".into()),
            }
        }
        Err(e) => {
            return GetPlaylistResponse {
                playlist: None,
                error: Some(e.to_string()),
            }
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => {
            return GetPlaylistResponse {
                playlist: None,
                error: Some(e.to_string()),
            }
        }
    };

    match load_playlist(&vault.system_dir(), &id) {
        Ok(playlist) => GetPlaylistResponse {
            playlist,
            error: None,
        },
        Err(e) => GetPlaylistResponse {
            playlist: None,
            error: Some(e.to_string()),
        },
    }
}

fn build_save_playlist_response(body: &[u8]) -> SavePlaylistResponse {
    let req: SavePlaylistRequest = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => return SavePlaylistResponse::error(format!("Invalid request: {e}")),
    };

    let vault_root = match resolve_vault_root(req.vault_root.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => return SavePlaylistResponse::error("Kein Vault geöffnet."),
        Err(e) => return SavePlaylistResponse::error(e.to_string()),
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => return SavePlaylistResponse::error(e.to_string()),
    };

    let mut playlist = req.playlist;
    match save_playlist(&vault.system_dir(), &mut playlist) {
        Ok(()) => SavePlaylistResponse::ok(),
        Err(e) => SavePlaylistResponse::error(e.to_string()),
    }
}

fn build_delete_playlist_response(body: &[u8]) -> DeletePlaylistResponse {
    let req: DeletePlaylistRequest = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => return DeletePlaylistResponse::error(format!("Invalid request: {e}")),
    };

    let vault_root = match resolve_vault_root(req.vault_root.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => return DeletePlaylistResponse::error("Kein Vault geöffnet."),
        Err(e) => return DeletePlaylistResponse::error(e.to_string()),
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => return DeletePlaylistResponse::error(e.to_string()),
    };

    match delete_playlist(&vault.system_dir(), &req.id) {
        Ok(()) => DeletePlaylistResponse::ok(),
        Err(e) => DeletePlaylistResponse::error(e.to_string()),
    }
}

fn build_save_cursor_response(body: &[u8]) -> SaveCursorResponse {
    let req: SaveCursorRequest = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => return SaveCursorResponse::error(format!("Invalid request: {e}")),
    };

    let vault_root = match resolve_vault_root(req.vault_root.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => return SaveCursorResponse::error("Kein Vault geöffnet."),
        Err(e) => return SaveCursorResponse::error(e.to_string()),
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => return SaveCursorResponse::error(e.to_string()),
    };

    let mut cursor = req.cursor;
    match save_cursor(&vault.progress_dir(), &mut cursor) {
        Ok(()) => SaveCursorResponse::ok(),
        Err(e) => SaveCursorResponse::error(e.to_string()),
    }
}

fn build_load_cursor_response(query: Option<&str>) -> LoadCursorResponse {
    let query = match query {
        Some(q) => q,
        None => {
            return LoadCursorResponse {
                cursor: None,
                error: Some("missing query".into()),
            }
        }
    };

    let id = match extract_query_value(query, "id") {
        Some(id) => id,
        None => {
            return LoadCursorResponse {
                cursor: None,
                error: Some("missing id".into()),
            }
        }
    };

    let root_override = extract_query_value(query, "root");
    let vault_root = match resolve_vault_root(root_override.as_deref()) {
        Ok(Some(r)) => r,
        Ok(None) => {
            return LoadCursorResponse {
                cursor: None,
                error: Some("Kein Vault geöffnet.".into()),
            }
        }
        Err(e) => {
            return LoadCursorResponse {
                cursor: None,
                error: Some(e.to_string()),
            }
        }
    };

    let vault = match Vault::new(vault_root) {
        Ok(v) => v,
        Err(e) => {
            return LoadCursorResponse {
                cursor: None,
                error: Some(e.to_string()),
            }
        }
    };

    match load_cursor(&vault.progress_dir(), &id) {
        Ok(cursor) => LoadCursorResponse {
            cursor,
            error: None,
        },
        Err(e) => LoadCursorResponse {
            cursor: None,
            error: Some(e.to_string()),
        },
    }
}

// ---------------------------------------------------------------------------
// Audiobookshelf (ABS) sync APIs
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct AbsTestResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl AbsTestResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

#[derive(Serialize)]
struct AbsLibrariesResponse {
    libraries: Vec<crate::api::audiobookshelf::AbsLibrary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct AbsLibraryItemsResponse {
    items: Vec<crate::api::audiobookshelf::AbsLibraryItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Deserialize)]
struct AbsSyncProgressRequest {
    abs_url: String,
    api_key: String,
    item_id: String,
    current_time: f64,
    #[serde(default)]
    duration: f64,
    #[serde(default)]
    is_finished: bool,
}

#[derive(Serialize)]
struct AbsSyncProgressResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl AbsSyncProgressResponse {
    fn ok() -> Self {
        Self {
            ok: true,
            error: None,
        }
    }
    fn error(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(msg.into()),
        }
    }
}

fn build_abs_test_response(query: Option<&str>) -> AbsTestResponse {
    let query = match query {
        Some(q) => q,
        None => return AbsTestResponse::error("missing query"),
    };
    let url = match extract_query_value(query, "url") {
        Some(u) => u,
        None => return AbsTestResponse::error("missing url"),
    };
    let key = extract_query_value(query, "key").unwrap_or_default();
    match AbsClient::new(url, key) {
        Ok(client) => match client.test_connection() {
            Ok(()) => AbsTestResponse::ok(),
            Err(e) => AbsTestResponse::error(e.to_string()),
        },
        Err(e) => AbsTestResponse::error(e.to_string()),
    }
}

fn build_abs_libraries_response(query: Option<&str>) -> AbsLibrariesResponse {
    let query = match query {
        Some(q) => q,
        None => {
            return AbsLibrariesResponse {
                libraries: vec![],
                error: Some("missing query".into()),
            }
        }
    };
    let url = match extract_query_value(query, "url") {
        Some(u) => u,
        None => {
            return AbsLibrariesResponse {
                libraries: vec![],
                error: Some("missing url".into()),
            }
        }
    };
    let key = extract_query_value(query, "key").unwrap_or_default();
    match AbsClient::new(url, key) {
        Ok(client) => match client.list_libraries() {
            Ok(libraries) => AbsLibrariesResponse {
                libraries,
                error: None,
            },
            Err(e) => AbsLibrariesResponse {
                libraries: vec![],
                error: Some(e.to_string()),
            },
        },
        Err(e) => AbsLibrariesResponse {
            libraries: vec![],
            error: Some(e.to_string()),
        },
    }
}

fn build_abs_library_items_response(query: Option<&str>) -> AbsLibraryItemsResponse {
    let query = match query {
        Some(q) => q,
        None => {
            return AbsLibraryItemsResponse {
                items: vec![],
                error: Some("missing query".into()),
            }
        }
    };
    let url = match extract_query_value(query, "url") {
        Some(u) => u,
        None => {
            return AbsLibraryItemsResponse {
                items: vec![],
                error: Some("missing url".into()),
            }
        }
    };
    let key = extract_query_value(query, "key").unwrap_or_default();
    let library_id = match extract_query_value(query, "library") {
        Some(id) => id,
        None => {
            return AbsLibraryItemsResponse {
                items: vec![],
                error: Some("missing library".into()),
            }
        }
    };
    match AbsClient::new(url, key) {
        Ok(client) => match client.list_library_items(&library_id) {
            Ok(items) => AbsLibraryItemsResponse { items, error: None },
            Err(e) => AbsLibraryItemsResponse {
                items: vec![],
                error: Some(e.to_string()),
            },
        },
        Err(e) => AbsLibraryItemsResponse {
            items: vec![],
            error: Some(e.to_string()),
        },
    }
}

fn build_abs_sync_progress_response(body: &[u8]) -> AbsSyncProgressResponse {
    let req: AbsSyncProgressRequest = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => return AbsSyncProgressResponse::error(format!("Invalid request: {e}")),
    };
    match AbsClient::new(&req.abs_url, &req.api_key) {
        Ok(client) => {
            let result = client.set_progress(
                &req.item_id,
                req.current_time,
                req.duration,
                req.is_finished,
            );
            match result {
                Ok(()) => AbsSyncProgressResponse::ok(),
                Err(e) => AbsSyncProgressResponse::error(e.to_string()),
            }
        }
        Err(e) => AbsSyncProgressResponse::error(e.to_string()),
    }
}
