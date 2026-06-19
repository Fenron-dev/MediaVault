#![doc = "MediaVault core foundation."]

pub mod ai;
pub mod app;
pub mod api;
pub mod core;
pub mod error;
pub mod media;

pub use ai::{AnalysisProfile, LmStudioConfig};
pub use app::AppConfig;
pub use api::anilist::AniListClient;
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

/// Starts the current MediaVault scaffold.
///
/// The UI shell and persistence layer will be wired in later. For now, the
/// library exposes the core data structures and planning primitives.
pub fn run() -> Result<()> {
    Ok(())
}
