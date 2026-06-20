//! AniList query helpers and request scaffolding.

use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::error::{Result, VaultError};
use crate::media::{MediaProperties, MediaType};

const DEFAULT_ENDPOINT: &str = "https://graphql.anilist.co";
const ANILIST_MEDIA_QUERY: &str = r#"
query ($search: String!, $isAdult: Boolean) {
  Media(search: $search, type: ANIME, isAdult: $isAdult) {
    id
    siteUrl
    title {
      romaji
      english
      native
    }
    description(asHtml: false)
    season
    seasonYear
    episodes
    duration
    status
    format
    genres
    averageScore
    coverImage {
      large
      extraLarge
    }
    bannerImage
    isAdult
  }
}
"#;

/// Minimal AniList client configuration used by the foundation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AniListClient {
    /// GraphQL endpoint.
    pub endpoint: String,
    /// Optional access token.
    pub access_token: Option<String>,
}

impl Default for AniListClient {
    fn default() -> Self {
        Self::new(DEFAULT_ENDPOINT)
    }
}

impl AniListClient {
    /// Creates a new AniList client configuration.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            access_token: None,
        }
    }

    /// Sets the access token used for authenticated requests.
    pub fn with_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.access_token = Some(access_token.into());
        self
    }

    /// Returns the media type specific adult flag used by AniList queries.
    pub fn adult_flag_for(media_type: MediaType) -> bool {
        media_type.is_adult()
    }

    /// Builds a JSON request body for an AniList anime search.
    pub fn build_search_query(search: &str, adult: bool) -> String {
        serde_json::json!({
            "query": ANILIST_MEDIA_QUERY,
            "variables": {
                "search": search,
                "isAdult": adult,
            }
        })
        .to_string()
    }

    /// Searches AniList for an anime title and returns the best match.
    pub fn search_anime(
        &self,
        search: &str,
        adult: bool,
    ) -> Result<Option<AniListAnimeMetadata>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(12))
            .build()
            .map_err(|error| VaultError::ExternalApi(error.to_string()))?;

        let mut request = client
            .post(&self.endpoint)
            .header(CONTENT_TYPE, "application/json");

        if let Some(access_token) = self.access_token.as_ref() {
            request = request.header(AUTHORIZATION, format!("Bearer {access_token}"));
        }

        let response = request
            .body(Self::build_search_query(search, adult))
            .send()
            .map_err(|error| VaultError::ExternalApi(error.to_string()))?;

        if !response.status().is_success() {
            return Err(VaultError::ExternalApi(format!(
                "http {} from AniList",
                response.status()
            )));
        }

        let payload: AniListGraphQlResponse = response
            .json()
            .map_err(|error| VaultError::ExternalApi(error.to_string()))?;

        if let Some(errors) = payload.errors {
            let message = errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(VaultError::ExternalApi(message));
        }

        Ok(payload.data.and_then(|data| data.media.map(AniListAnimeMetadata::from)))
    }
}

/// A normalized AniList anime result.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AniListAnimeMetadata {
    /// AniList media ID.
    pub anilist_id: u32,
    /// AniList detail URL.
    pub anilist_url: Option<String>,
    /// Romaji title.
    pub title_romaji: Option<String>,
    /// English title.
    pub title_english: Option<String>,
    /// Native title.
    pub title_native: Option<String>,
    /// Description text.
    pub description: Option<String>,
    /// Aired season label.
    pub season: Option<String>,
    /// Aired season year.
    pub season_year: Option<u16>,
    /// Episode count.
    pub episodes: Option<u16>,
    /// Episode duration in minutes.
    pub duration: Option<u16>,
    /// AniList status string.
    pub status: Option<String>,
    /// AniList format string.
    pub format: Option<String>,
    /// Genres.
    pub genres: Vec<String>,
    /// AniList community score.
    pub average_score: Option<f32>,
    /// Large cover image.
    pub cover_image_large: Option<String>,
    /// Extra large cover image.
    pub cover_image_extra_large: Option<String>,
    /// Banner image.
    pub banner_image: Option<String>,
    /// Whether the title is adult-oriented.
    pub is_adult: bool,
}

impl AniListAnimeMetadata {
    /// Returns the best title for display and search results.
    pub fn display_title(&self) -> Option<&str> {
        self.title_english
            .as_deref()
            .or(self.title_romaji.as_deref())
            .or(self.title_native.as_deref())
    }

    /// Applies the fetched AniList data to portable media properties.
    pub fn apply_to_properties(&self, properties: &mut MediaProperties) {
        properties.anilist_id = Some(self.anilist_id);
        properties.anilist_url = self.anilist_url.clone();
        properties.title = self.display_title().map(|value| value.to_string());
        properties.title_original = self.title_native.clone();
        properties.description = self.description.clone();
        properties.year = self.season_year;
        properties.genres = self.genres.clone();
        properties.categories = self
            .format
            .as_ref()
            .map(|value| vec![value.clone()])
            .unwrap_or_default();
        properties.rating_external = self.average_score;
        properties.series_title = self.display_title().map(|value| value.to_string());
        properties.episode_count = self.episodes;
        properties.runtime_minutes = self.duration;
        properties.average_score = self.average_score;
        properties.format = self.format.clone();
        properties.airing_season = self.season.clone();
    }
}

#[derive(Debug, Deserialize)]
struct AniListGraphQlResponse {
    data: Option<AniListGraphQlData>,
    errors: Option<Vec<AniListGraphQlError>>,
}

#[derive(Debug, Deserialize)]
struct AniListGraphQlData {
    #[serde(rename = "Media")]
    media: Option<AniListGraphQlMedia>,
}

#[derive(Debug, Deserialize)]
struct AniListGraphQlError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct AniListGraphQlMedia {
    id: u32,
    #[serde(rename = "siteUrl")]
    site_url: Option<String>,
    title: AniListTitles,
    description: Option<String>,
    season: Option<String>,
    #[serde(rename = "seasonYear")]
    season_year: Option<u16>,
    episodes: Option<u16>,
    duration: Option<u16>,
    status: Option<String>,
    format: Option<String>,
    genres: Vec<String>,
    #[serde(rename = "averageScore")]
    average_score: Option<f32>,
    #[serde(rename = "coverImage")]
    cover_image: AniListCoverImage,
    #[serde(rename = "bannerImage")]
    banner_image: Option<String>,
    #[serde(rename = "isAdult")]
    is_adult: bool,
}

#[derive(Debug, Deserialize)]
struct AniListTitles {
    romaji: Option<String>,
    english: Option<String>,
    native: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AniListCoverImage {
    large: Option<String>,
    #[serde(rename = "extraLarge")]
    extra_large: Option<String>,
}

impl From<AniListGraphQlMedia> for AniListAnimeMetadata {
    fn from(media: AniListGraphQlMedia) -> Self {
        Self {
            anilist_id: media.id,
            anilist_url: media.site_url,
            title_romaji: media.title.romaji,
            title_english: media.title.english,
            title_native: media.title.native,
            description: media.description,
            season: media.season,
            season_year: media.season_year,
            episodes: media.episodes,
            duration: media.duration,
            status: media.status,
            format: media.format,
            genres: media.genres,
            average_score: media.average_score,
            cover_image_large: media.cover_image.large,
            cover_image_extra_large: media.cover_image.extra_large,
            banner_image: media.banner_image,
            is_adult: media.is_adult,
        }
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
        assert!(query.contains("Media"));
        assert!(query.contains("Naruto"));
    }
}
