use crate::app::error::AppError;
use crate::model::elite::{Elite, EliteStatus};
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
            .prepare_cached("SELECT * FROM elites_with_ign WHERE discord_user_id = $1 AND status = ANY($2)")
            .await
            .map_err(|e| CreatePreparedStatementError(e.to_string()))?;

        let elite_opt = con
            .query_opt(&stmt, &[discord_id, &vec!["staff", "veteran", "elite", "trial"]]) // TODO - Maybe make statuses a parameter / or just remove it entirely
            .await
            .map_err(|_| DbConnectionError)?
            .map(|row| Elite::from(&row));

        Ok(elite_opt)
    }

    pub async fn elites_all(&self, statuses: &Vec<EliteStatus>) -> Result<Vec<Elite>, AppError> {
        let con = self.db_pool.get().await.map_err(|_| DbConnectionError)?;

        let stmt = con
            .prepare_cached("SELECT * FROM elites_with_ign WHERE status = ANY($1)")
            .await
            .map_err(|e| CreatePreparedStatementError(e.to_string()))?;

        let elites = con.query(&stmt, &[statuses]).await.map_err(|_| DbConnectionError)?.iter().map(Elite::from).collect();

        Ok(elites)
    }
}
