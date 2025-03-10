use crate::app::error::AppError;
use std::fmt::{Debug, Display, Formatter};
use tracing::debug;

#[derive(Clone, Debug)]
pub enum Error {
    DiscordApiRequestError(String),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        debug!("{:<12} - {value:?}", "FROM_APP_ERR");

        match value {
            Error::DiscordApiRequestError(_) => AppError::InternalServerError,
        }
    }
}
impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
