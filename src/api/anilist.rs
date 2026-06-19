//! AniList query helpers and request scaffolding.

use crate::media::MediaType;

/// Minimal AniList client configuration used by the foundation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AniListClient {
    /// GraphQL endpoint.
    pub endpoint: String,
    /// Optional access token.
    pub access_token: Option<String>,
}

impl AniListClient {
    /// Creates a new AniList client configuration.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            access_token: None,
        }
    }

    /// Returns the media type specific adult flag used by AniList queries.
    pub fn adult_flag_for(media_type: MediaType) -> bool {
        media_type.is_adult()
    }

    /// Builds a JSON request body for an AniList title search.
    pub fn build_search_query(search: &str, adult: bool) -> String {
        let escaped_search = search.replace('\\', "\\\\").replace('"', "\\\"");
        let adult_value = if adult { "true" } else { "false" };

        format!(
            r#"{{"query":"query ($search: String!, $isAdult: Boolean) {{ anime(search: $search, isAdult: $isAdult) {{ id title {{ romaji english native }} description(asHtml: false) season seasonYear episodes duration status format genres averageScore coverImage {{ large extraLarge }} bannerImage isAdult }} }}","variables":{{"search":"{escaped_search}","isAdult":{adult_value}}}}}"#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adult_flag_matches_media_type() {
        assert!(AniListClient::adult_flag_for(MediaType::HentaiAnime));
        assert!(!AniListClient::adult_flag_for(MediaType::Anime));
    }

    #[test]
    fn builds_query_text() {
        let query = AniListClient::build_search_query("Naruto", false);
        assert!(query.contains("anime"));
        assert!(query.contains("Naruto"));
    }
}
