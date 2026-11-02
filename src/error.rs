use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::types::ErrorResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Upstream fetch failed: {0}")]
    UpstreamFetch(String),

    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, detail) = match self {
            ApiError::UpstreamFetch(msg) => (StatusCode::BAD_GATEWAY, msg),
            ApiError::ParseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::BadRequest(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse { detail });
        (status, body).into_response()
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::UpstreamFetch(err.to_string())
    }
}

impl From<scraper::error::SelectorError> for ApiError {
    fn from(err: scraper::error::SelectorError) -> Self {
        ApiError::ParseError(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::ParseError(err.to_string())
    }
}

impl From<base64::DecodeError> for ApiError {
    fn from(err: base64::DecodeError) -> Self {
        ApiError::ParseError(err.to_string())
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;