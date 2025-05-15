use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub type Result<T> = core::result::Result<T, AppError>;

// Client facing errors
#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(Option<String>),
    InternalServerError,
    BadRequest(Option<String>),
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(Some(msg)) => (StatusCode::NOT_FOUND, Some(msg.clone())),
            AppError::NotFound(None) => (StatusCode::NOT_FOUND, Some("Resource Not Found".to_string())),
            AppError::BadRequest(None) => (StatusCode::BAD_REQUEST, None),
            AppError::BadRequest(Some(msg)) => (StatusCode::BAD_REQUEST, Some(msg.clone())),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, None),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, None),
        };

        let mut res = match message {
            Some(msg) => (status, Json(json!({ "error": msg }))).into_response(),
            None => status.into_response(),
        };

        // Put error in response for later use in response_mapper
        res.extensions_mut().insert(self);

        res
    }
}
