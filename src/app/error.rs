use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub type Result<T> = core::result::Result<T, AppError>;

// Client facing errors
pub enum AppError {
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource Not Found"),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        let body = json!({ "error": message });
        (status, Json(body)).into_response()
    }
}
