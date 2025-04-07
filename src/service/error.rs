use crate::app::error::AppError;
use tracing::trace;

#[derive(Debug)]
pub enum ServiceError {
    DbConnectionError,
    CreatePreparedStatementError(String),
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        trace!("{:<12} - {value:?}", "FROM_APP_ERR");

        match value {
            ServiceError::DbConnectionError => AppError::InternalServerError,
            ServiceError::CreatePreparedStatementError(_) => AppError::InternalServerError,
        }
    }
}
