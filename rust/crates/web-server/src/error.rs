use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Application-level error type used by route handlers.
#[derive(Debug)]
pub enum AppError {
    SessionNotFound(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionNotFound(id) => write!(f, "session not found: {id}"),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::SessionNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = serde_json::json!({ "error": message });
        (status, axum::Json(body)).into_response()
    }
}
