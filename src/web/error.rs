use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    // Auth/Discord related errors
    NoDiscordCodeInPath,
    DiscordTokenError(String),
    DiscordApiError(String),

    // JWT related errors
    JwtTokenGenerationError,
    JwtTokenValidationError,
    JwtTokenExpired,

    // Redis errors
    RedisConnectionError,
    RedisOperationError(String),

    // Cookie related errors
    CookieParseError,
    CookieNotFound,

    // General request/response errors
    ReqStampNotInReqExt,
    InvalidRequest(String),
    UnauthorizedAccess,
    InternalServerError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - {self:?}", "INTO_RES");

        // Placeholder response
        let mut response = (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
