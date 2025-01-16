use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use log::debug;
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJson(String)
}

// Implement conversion from serde_json::Error to your custom Error
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error.to_string())
    }
}

// Implement IntoResponse for your custom Error
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("Error occurred: {self:?}");

        let status = match &self {
            Error::SerdeJson(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({ "error": format!("{self:?}") }));
        (status, body).into_response()
    }
}