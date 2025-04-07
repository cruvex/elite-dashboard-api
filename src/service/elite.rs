use crate::app::error::AppError;
use crate::model::elite::Elite;
use crate::service::error::ServiceError::{CreatePreparedStatementError, DbConnectionError};
use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct EliteService {
    db_pool: Pool,
}

impl EliteService {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}

impl EliteService {
    pub async fn find_by_discord_id(&self, discord_id: &String) -> Result<Option<Elite>, AppError> {
        let con = self.db_pool.get().await.map_err(|_| DbConnectionError)?;

        let stmt = con
            .prepare_cached("SELECT * FROM elites WHERE discord_user_id = $1")
            .await
            .map_err(|e| CreatePreparedStatementError(e.to_string()))?;

        let elite_opt = con.query_opt(&stmt, &[discord_id]).await.map_err(|_| DbConnectionError)?.map(Elite::from);

        // let elite = row_opt.map(Elite::from);

        Ok(elite_opt)
    }
}
