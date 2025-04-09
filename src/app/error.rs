use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub type Result<T> = core::result::Result<T, AppError>;

// Client facing errors
#[derive(Debug, Clone)]
pub enum AppError {
    _NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::_NotFound => (StatusCode::NOT_FOUND, Some("Resource Not Found")),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, Some("Bad Request")),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, None),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, Some("Internal Server Error")),
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
