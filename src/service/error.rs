use crate::app::error::AppError;
use tracing::trace;

#[derive(Debug)]
pub enum ServiceError {
    DbConnectionError,
    #[allow(dead_code)] // FIXME
    CreatePreparedStatementError(String),
    NoFieldsToUpdate,
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        trace!("{:<12} - {value:?}", "FROM_APP_ERR");

        match value {
            ServiceError::DbConnectionError => AppError::InternalServerError,
            ServiceError::CreatePreparedStatementError(_) => AppError::InternalServerError,
            ServiceError::NoFieldsToUpdate => AppError::BadRequest(Some("Provide at least 1 field to update".to_string())),
        }
    }
}
