use crate::app::error::AppError;
use tracing::trace;

#[derive(Debug, Clone)]
pub enum Error {
    // Auth/Discord related errors
    NoCodeInDiscordCallbackPath,
    NoStateInDiscordCallbackPath,
    DiscordApiError(String),

    // Session errors
    SessionCookieNotFound,
    NoSessionFound,
    InvalidSession(String),

    // Redis errors
    RedisOperationError(String),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        trace!("{:<12} - {value:?}", "FROM_APP_ERR");

        match value {
            Error::NoCodeInDiscordCallbackPath => AppError::BadRequest,
            Error::NoStateInDiscordCallbackPath => AppError::BadRequest,
            Error::DiscordApiError(_) => AppError::InternalServerError,
            Error::RedisOperationError(_) => AppError::InternalServerError,
            Error::SessionCookieNotFound => AppError::Unauthorized,
            Error::NoSessionFound => AppError::Unauthorized,
            Error::InvalidSession(_) => AppError::Unauthorized,
        }
    }
}
