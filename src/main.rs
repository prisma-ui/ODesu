mod config;
mod error;
mod handlers;
mod http_client;
mod parsing;
mod scrapers;
mod types;

use crate::config::CONFIG;
use crate::error::Result;
use crate::handlers::*;
use crate::http_client::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info,tower_http=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config (also validates)
    let _ = &*CONFIG;

    // Create shared state
    let state = AppState::new()?;

    // Build router
    let app = Router::new()
        // Health
        .route("/api/ping", get(ping))
        // Home
        .route("/api/home", get(home))
        // Anime lists
        .route("/api/ongoing", get(ongoing))
        .route("/api/complete", get(complete))
        .route("/api/anime-list", get(anime_list))
        // Schedule
        .route("/api/jadwal-rilis", get(jadwal_rilis))
        // Genres
        .route("/api/genres", get(genres))
        .route("/api/genres/{slug}", get(genre_detail))
        // Search
        .route("/api/search", get(search))
        // Anime detail
        .route("/api/anime/{slug}", get(anime_detail))
        // Episode
        .route("/api/episode/{slug}", get(episode_detail))
        // Batch
        .route("/api/batch/{slug}", get(batch_detail))
        // Mirror
        .route("/api/mirror", post(mirror_resolve))
        // Docs
        .route("/docs", get(swagger_ui))
        .route("/api/openapi.json", get(openapi_json))
        // Shared state
        .with_state(state)
        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(CONFIG.request_timeout()))
                .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
                .into_inner(),
        );

    // Start server
    let addr = CONFIG.socket_addr();
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}