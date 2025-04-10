use crate::app::error::AppError;
use tracing::trace;

#[derive(Debug)]
pub enum DbError {
    ConnectionError,
    #[allow(dead_code)]
    QueryError(String),
    #[allow(dead_code)]
    MigrationError(String),
}

#[allow(dead_code)]
pub enum RedisError {
    ConnectionError,
    #[allow(dead_code)]
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
