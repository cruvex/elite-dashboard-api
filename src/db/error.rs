use crate::app::error::AppError;
use tracing::trace;

#[derive(Debug)]
pub enum DbError {
    ConnectionError,
    #[allow(dead_code)] // FIXME
    QueryError(String),
    #[allow(dead_code)] // FIXME
    MigrationError(String),
}

#[allow(dead_code)] // FIXME
pub enum RedisError {
    ConnectionError,
    #[allow(dead_code)] // FIXME
    OperationError(String),
}

impl From<DbError> for AppError {
    fn from(value: DbError) -> Self {
        trace!("{:<12} - {value:?}", "FROM_APP_ERR");

        match value {
            DbError::ConnectionError => AppError::InternalServerError,
            DbError::QueryError(_) => AppError::InternalServerError,
            DbError::MigrationError(_) => AppError::InternalServerError,
        }
    }
}
