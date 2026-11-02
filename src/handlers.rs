use crate::config::CONFIG;
use crate::error::{ApiError, Result};
use crate::http_client::AppState;
use crate::scrapers::*;
use crate::types::*;
use axum::{
    extract::{Path, Query, State},
    response::Html as HtmlResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::OpenApi;

/// Query parameters for paginated endpoints
#[derive(Deserialize, utoipa::IntoParams)]
pub struct PageQuery {
    #[serde(default = "default_page")]
    pub page: u32,
}

fn default_page() -> u32 {
    1
}

/// Query parameters for search
#[derive(Deserialize, utoipa::IntoParams)]
pub struct SearchQuery {
    pub q: String,
}

/// Mirror resolve request
#[derive(Deserialize, utoipa::ToSchema)]
pub struct MirrorResolveRequest {
    pub data_content: String,
}

/// Mirror resolve response
#[derive(Serialize, utoipa::ToSchema)]
pub struct MirrorResolveResponse {
    pub embed_html: String,
}

/// GET /api/ping
#[utoipa::path(
    get,
    path = "/api/ping",
    tag = "Health",
    responses(
        (status = 200, description = "Health check", body = PingResponse)
    )
)]
pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

/// GET /api/home
#[utoipa::path(
    get,
    path = "/api/home",
    tag = "Home",
    responses(
        (status = 200, description = "Ongoing + Complete preview", body = HomeResponse),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn home(State(state): State<AppState>) -> Result<Json<HomeResponse>> {
    let html = state.fetch_html("/").await?;
    Ok(Json(parse_home(&html)))
}

/// GET /api/ongoing
#[utoipa::path(
    get,
    path = "/api/ongoing",
    tag = "Anime List",
    params(PageQuery),
    responses(
        (status = 200, description = "Ongoing anime list", body = AnimeGridResponse),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn ongoing(
    State(state): State<AppState>,
    Query(params): Query<PageQuery>,
) -> Result<Json<AnimeGridResponse>> {
    let path = if params.page == 1 {
        "/ongoing-anime/".to_string()
    } else {
        format!("/ongoing-anime/page/{}/", params.page)
    };
    let html = state.fetch_html(&path).await?;
    Ok(Json(parse_anime_grid(&html)))
}

/// GET /api/complete
#[utoipa::path(
    get,
    path = "/api/complete",
    tag = "Anime List",
    params(PageQuery),
    responses(
        (status = 200, description = "Complete anime list", body = AnimeGridResponse),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn complete(
    State(state): State<AppState>,
    Query(params): Query<PageQuery>,
) -> Result<Json<AnimeGridResponse>> {
    let path = if params.page == 1 {
        "/complete-anime/".to_string()
    } else {
        format!("/complete-anime/page/{}/", params.page)
    };
    let html = state.fetch_html(&path).await?;
    Ok(Json(parse_anime_grid(&html)))
}

/// GET /api/anime-list
#[utoipa::path(
    get,
    path = "/api/anime-list",
    tag = "Anime List",
    responses(
        (status = 200, description = "Alphabetical anime index", body = AnimeListGroups),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn anime_list(State(state): State<AppState>) -> Result<Json<AnimeListGroups>> {
    let html = state.fetch_html("/anime-list/").await?;
    Ok(Json(parse_anime_list(&html)))
}

/// GET /api/jadwal-rilis
#[utoipa::path(
    get,
    path = "/api/jadwal-rilis",
    tag = "Schedule",
    responses(
        (status = 200, description = "Release schedule", body = JadwalRilisResponse),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn jadwal_rilis(State(state): State<AppState>) -> Result<Json<JadwalRilisResponse>> {
    let html = state.fetch_html("/jadwal-rilis/").await?;
    Ok(Json(parse_jadwal_rilis(&html)))
}

/// GET /api/genres
#[utoipa::path(
    get,
    path = "/api/genres",
    tag = "Genres",
    responses(
        (status = 200, description = "Genre list", body = Vec<GenreLink>),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn genres(State(state): State<AppState>) -> Result<Json<Vec<GenreLink>>> {
    let html = state.fetch_html("/genres/").await?;
    Ok(Json(parse_genre_list(&html)))
}

/// GET /api/genres/{slug}
#[utoipa::path(
    get,
    path = "/api/genres/{slug}",
    tag = "Genres",
    params(
        ("slug" = String, Path, description = "Genre slug"),
        PageQuery
    ),
    responses(
        (status = 200, description = "Genre detail", body = GenreDetailResponse),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn genre_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<PageQuery>,
) -> Result<Json<GenreDetailResponse>> {
    let path = if params.page == 1 {
        format!("/genres/{}/", slug)
    } else {
        format!("/genres/{}/page/{}/", slug, params.page)
    };
    let html = state.fetch_html(&path).await?;
    Ok(Json(parse_genre_detail(&html)))
}

/// GET /api/search
#[utoipa::path(
    get,
    path = "/api/search",
    tag = "Search",
    params(SearchQuery),
    responses(
        (status = 200, description = "Search results", body = Vec<SearchResult>),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<SearchResult>>> {
    let path = format!("/?s={}", urlencoding::encode(&params.q));
    let html = state.fetch_html(&path).await?;
    Ok(Json(parse_search(&html)))
}

/// GET /api/anime/{slug}
#[utoipa::path(
    get,
    path = "/api/anime/{slug}",
    tag = "Anime Detail",
    params(
        ("slug" = String, Path, description = "Anime slug")
    ),
    responses(
        (status = 200, description = "Anime detail", body = AnimeDetail),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn anime_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<AnimeDetail>> {
    let html = state.fetch_html(&format!("/anime/{}/", slug)).await?;
    Ok(Json(parse_anime_detail(&html)))
}

/// GET /api/episode/{slug}
#[utoipa::path(
    get,
    path = "/api/episode/{slug}",
    tag = "Episode",
    params(
        ("slug" = String, Path, description = "Episode slug")
    ),
    responses(
        (status = 200, description = "Episode detail", body = EpisodeDetail),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn episode_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<EpisodeDetail>> {
    let html = state.fetch_html(&format!("/episode/{}/", slug)).await?;
    Ok(Json(parse_episode(&html)))
}

/// GET /api/batch/{slug}
#[utoipa::path(
    get,
    path = "/api/batch/{slug}",
    tag = "Batch",
    params(
        ("slug" = String, Path, description = "Batch slug")
    ),
    responses(
        (status = 200, description = "Batch detail", body = BatchDetail),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn batch_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<BatchDetail>> {
    let html = state.fetch_html(&format!("/batch/{}/", slug)).await?;
    Ok(Json(parse_batch(&html)))
}

/// POST /api/mirror
#[utoipa::path(
    post,
    path = "/api/mirror",
    tag = "Mirror",
    request_body = MirrorResolveRequest,
    responses(
        (status = 200, description = "Resolved embed HTML", body = MirrorResolveResponse),
        (status = 422, description = "Missing data_content", body = ErrorResponse),
        (status = 502, description = "Upstream fetch failed", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
pub async fn mirror_resolve(
    State(state): State<AppState>,
    Json(req): Json<MirrorResolveRequest>,
) -> Result<Json<MirrorResolveResponse>> {
    if req.data_content.is_empty() {
        return Err(ApiError::BadRequest("Missing 'data_content'".to_string()));
    }

    let embed_html = state.resolve_mirror_embed(&req.data_content).await?;
    Ok(Json(MirrorResolveResponse { embed_html }))
}

/// GET /docs - Swagger UI
pub async fn swagger_ui() -> HtmlResponse<String> {
    HtmlResponse(utoipa_swagger_ui::swagger_ui_html(
        "/api/openapi.json",
        Some("Otakudesu API Docs"),
    ))
}

/// GET /api/openapi.json - OpenAPI spec
pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        ping,
        home,
        ongoing,
        complete,
        anime_list,
        jadwal_rilis,
        genres,
        genre_detail,
        search,
        anime_detail,
        episode_detail,
        batch_detail,
        mirror_resolve,
        swagger_ui,
        openapi_json
    ),
    components(
        schemas(
            PingResponse,
            HomeResponse,
            AnimeGridResponse,
            AnimeListGroups,
            JadwalRilisResponse,
            GenreLink,
            GenreDetailResponse,
            SearchResult,
            AnimeDetail,
            EpisodeDetail,
            BatchDetail,
            MirrorResolveRequest,
            MirrorResolveResponse,
            ErrorResponse,
            AnimeCard,
            EpisodeLink,
            DownloadLink,
            DownloadQuality,
            Pagination,
            GenreDetailItem,
            MirrorHost,
            RecommendationItem
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Home", description = "Home page endpoints"),
        (name = "Anime List", description = "Anime listing endpoints"),
        (name = "Schedule", description = "Release schedule endpoints"),
        (name = "Genres", description = "Genre endpoints"),
        (name = "Search", description = "Search endpoints"),
        (name = "Anime Detail", description = "Anime detail endpoints"),
        (name = "Episode", description = "Episode detail endpoints"),
        (name = "Batch", description = "Batch download endpoints"),
        (name = "Mirror", description = "Mirror resolution endpoints")
    ),
    info(
        title = "Otakudesu API",
        version = "1.0.0",
        description = "Rust (axum) port of the Otakudesu scraper API"
    ),
    servers(
        (url = "http://localhost:7860", description = "Local development server"),
        (url = "https://api.example.com", description = "Production server")
    )
)]
struct ApiDoc;