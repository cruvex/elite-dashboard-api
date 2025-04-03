use crate::app::error::AppError;
use tracing::debug;

#[derive(Debug, Clone)]
pub enum Error {
    // Auth/Discord related errors
    NoCodeInDiscordCallbackPath,
    NoStateInDiscordCallbackPath,
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
    AuthCookieNotFound,
    RefreshCookieNotFound,

    // General request/response errors
    ReqStampNotInReqExt,
    CtxNotinReqExt,
    InvalidRequest(String),
    UnauthorizedAccess,
    InternalServerError(String),
    SessionCookieNotFound,
    NoSessionFound,
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        debug!("{:<12} - {value:?}", "FROM_APP_ERR");

        match value {
            Error::NoCodeInDiscordCallbackPath => AppError::BadRequest,
            Error::NoStateInDiscordCallbackPath => AppError::BadRequest,
            Error::DiscordTokenError(_) => AppError::InternalServerError,
            Error::DiscordApiError(_) => AppError::InternalServerError,
            Error::JwtTokenGenerationError => AppError::InternalServerError,
            Error::JwtTokenValidationError => AppError::Unauthorized,
            Error::JwtTokenExpired => AppError::Unauthorized,
            Error::RedisConnectionError => AppError::InternalServerError,
            Error::RedisOperationError(_) => AppError::InternalServerError,
            Error::CookieParseError => AppError::InternalServerError,
            Error::AuthCookieNotFound => AppError::Unauthorized,
            Error::RefreshCookieNotFound => AppError::Unauthorized,
            Error::ReqStampNotInReqExt => AppError::InternalServerError,
            Error::CtxNotinReqExt => AppError::InternalServerError,
            Error::InvalidRequest(_) => AppError::BadRequest,
            Error::UnauthorizedAccess => AppError::Unauthorized,
            Error::InternalServerError(_) => AppError::InternalServerError,
            Error::SessionCookieNotFound => AppError::Unauthorized,
            Error::NoSessionFound => AppError::Unauthorized,
        }
    }
}
