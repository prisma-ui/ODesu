use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GenreLink {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AnimeCard {
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeLink {
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadLink {
    pub host: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadQuality {
    pub format: String,
    pub quality: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    pub links: Vec<DownloadLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub current_page: u32,
    pub last_page: u32,
    pub has_next: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AnimeGridResponse {
    pub anime_list: Vec<AnimeCard>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HomeResponse {
    pub ongoing: Vec<AnimeCard>,
    pub complete: Vec<AnimeCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AnimeListGroups(pub HashMap<String, Vec<EpisodeLink>>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JadwalRilisResponse(pub HashMap<String, Vec<EpisodeLink>>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AnimeDetail {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub japanese_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_episode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<String>,
    pub genres: Vec<GenreLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synopsis: Option<String>,
    pub episode_list: Vec<EpisodeLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<EpisodeLink>,
    pub recommendations: Vec<RecommendationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationItem {
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MirrorHost {
    pub host: String,
    pub data_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeDetail {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    pub mirrors: HashMap<String, Vec<MirrorHost>>,
    pub downloads: Vec<DownloadQuality>,
    pub episode_list: Vec<EpisodeLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_episode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anime_slug: Option<String>,
    pub info: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BatchDetail {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anime_slug: Option<String>,
    pub info: HashMap<String, serde_json::Value>,
    pub downloads: Vec<DownloadQuality>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    pub genres: Vec<GenreLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GenreDetailItem {
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<String>,
    pub genres: Vec<GenreLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synopsis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GenreDetailResponse {
    pub genre_title: String,
    pub anime_list: Vec<GenreDetailItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MirrorResolveRequest {
    pub data_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MirrorResolveResponse {
    pub embed_html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub status: String,
    pub timestamp: String,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            current_page: 1,
            last_page: 1,
            has_next: false,
        }
    }
}