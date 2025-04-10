use crate::app::error::AppError;
use crate::db::error::DbError;
use deadpool_postgres::Pool;
use refinery::embed_migrations;
use std::ops::DerefMut;
use tracing::info;

embed_migrations!("./migrations");

pub async fn run_migrations(db_pool: Pool) -> Result<(), AppError> {
    let mut conn = db_pool.get().await.map_err(|_| DbError::ConnectionError)?;
    let client = conn.deref_mut().deref_mut();

    info!("Running migrations...");

    let report = migrations::runner()
        .set_grouped(true)
        .set_abort_missing(true)
        .run_async(client)
        .await
        .map_err(|e| DbError::MigrationError(e.to_string()))?;

    if report.applied_migrations().is_empty() {
        info!("");
        info!("No new migrations to apply");
        info!("");
    } else {
        report.applied_migrations().iter().for_each(|migration| {
            info!("Successfully applied migration: {}", migration.name());
        });
        info!("");
        info!("Successfully applied {} migration(s)", report.applied_migrations().len());
        info!("");
    }

    Ok(())
}
