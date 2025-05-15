use crate::app::error::AppError;
use tracing::trace;

#[derive(Debug, Clone)]
pub enum Error {
    // Auth/Discord related errors
    NoCodeInDiscordCallbackPath,
    NoStateInDiscordCallbackPath,
    #[allow(dead_code)] // FIXME
    DiscordApiError(String),

    // Session errors
    SessionCookieNotFound,
    SessionNotFound,
    #[allow(dead_code)] // FIXME
    InvalidSession(String),

    // Redis errors
    #[allow(dead_code)] // FIXME
    RedisOperationError(String),

    NotInElite,
    NotInEliteGuild,
    EliteNotFound(String),
    StaffOnly,
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        trace!("{:<12} - {value:?}", "FROM_APP_ERR");

        match value {
            Error::NoCodeInDiscordCallbackPath => AppError::BadRequest(None),
            Error::NoStateInDiscordCallbackPath => AppError::BadRequest(None),
            Error::DiscordApiError(_) => AppError::InternalServerError,
            Error::RedisOperationError(_) => AppError::InternalServerError,
            Error::SessionCookieNotFound => AppError::Unauthorized,
            Error::SessionNotFound => AppError::Unauthorized,
            Error::InvalidSession(_) => AppError::Unauthorized,
            Error::NotInElite => AppError::Unauthorized,
            Error::NotInEliteGuild => AppError::Unauthorized,
            Error::EliteNotFound(msg) => AppError::NotFound(Some(msg)),
            Error::StaffOnly => AppError::Unauthorized,
        }
    }
}
