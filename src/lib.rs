#![doc = "MediaVault core foundation."]

mod desktop;

pub mod ai;
pub mod api;
pub mod app;
pub mod core;
pub mod error;
pub mod media;

pub use ai::{AnalysisProfile, LmStudioConfig};
pub use api::anilist::AniListClient;
pub use app::AppConfig;
pub use core::covers::{CoverCandidate, CoverFallbackChain, CoverSource};
pub use core::duplicate::{compute_fingerprint, compute_fingerprint_for_file, FileFingerprint};
pub use core::import::{
    ClassificationSource, DuplicatePolicy, FileClassification, ImportConfig, ImportPlan,
    ImportPlanItem, ImportPlanner, ImportSummary, IncomingFile, PlannedImportStep,
    ResolvedMetadata, UserPrompt,
};
pub use core::properties::{render_sidecar_yaml, sidecar_path_for};
pub use core::vault::{RelativePath, Vault};
pub use error::{Result, VaultError};
pub use media::{
    MediaEntry, MediaProperties, MediaStatus, MediaType, PropertySource, ALL_MEDIA_TYPES,
};

/// Starts the MediaVault desktop shell.
///
/// The core modules still expose the vault, import, and metadata primitives;
/// this entry point now opens the first testable Tauri window.
pub fn run() -> Result<()> {
    desktop::run()
}
