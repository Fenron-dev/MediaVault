//! Application configuration and portable settings.

use std::path::PathBuf;

use crate::ai::LmStudioConfig;
use crate::core::import::ImportConfig;

/// High-level application configuration for the portable desktop app.
#[derive(Debug, Clone, PartialEq)]
pub struct AppConfig {
    /// Optional vault root selected by the user.
    pub vault_root: Option<PathBuf>,
    /// UI language code.
    pub ui_language: String,
    /// Import behavior configuration.
    pub import: ImportConfig,
    /// AI configuration.
    pub ai: LmStudioConfig,
    /// Whether file watching is enabled.
    pub watch_for_changes: bool,
}

impl AppConfig {
    /// Creates a config with local-first defaults.
    pub fn local_defaults() -> Self {
        Self {
            vault_root: None,
            ui_language: "de".to_string(),
            import: ImportConfig::default(),
            ai: LmStudioConfig::local_defaults(),
            watch_for_changes: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_use_german_ui() {
        let config = AppConfig::local_defaults();
        assert_eq!(config.ui_language, "de");
        assert!(!config.watch_for_changes);
    }
}
