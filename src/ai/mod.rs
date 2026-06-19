//! Local AI integration scaffolding for LM Studio.

/// LM Studio configuration used by the foundation layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LmStudioConfig {
    /// Chat completions endpoint.
    pub chat_endpoint: String,
    /// Embeddings endpoint.
    pub embeddings_endpoint: String,
    /// Analysis model name.
    pub analysis_model: String,
    /// Embedding model name.
    pub embedding_model: String,
    /// Request timeout in seconds.
    pub timeout_seconds: u64,
}

impl LmStudioConfig {
    /// Creates a config with sensible local defaults.
    pub fn local_defaults() -> Self {
        Self {
            chat_endpoint: "http://localhost:1234/v1/chat/completions".to_string(),
            embeddings_endpoint: "http://localhost:1234/v1/embeddings".to_string(),
            analysis_model: "llama3".to_string(),
            embedding_model: "nomic-embed-text".to_string(),
            timeout_seconds: 60,
        }
    }
}

/// A prompt profile for a specific media type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisProfile {
    /// Short profile name.
    pub name: String,
    /// System prompt text.
    pub system_prompt: String,
    /// User prompt template.
    pub user_template: String,
}

impl AnalysisProfile {
    /// Creates a new analysis profile.
    pub fn new(
        name: impl Into<String>,
        system_prompt: impl Into<String>,
        user_template: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            system_prompt: system_prompt.into(),
            user_template: user_template.into(),
        }
    }
}
