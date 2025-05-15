use crate::app::error::AppError;
use crate::db::error::DbError;
use crate::model::elite::{Elite, EliteForUpdate, EliteStatus};
use crate::service::error::ServiceError::{CreatePreparedStatementError, DbConnectionError, NoFieldsToUpdate};
use crate::web::error::Error::EliteNotFound;
use deadpool_postgres::{GenericClient, Pool};

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

    pub async fn update_elite(&self, elite_id: i32, updated_elite: &EliteForUpdate) -> Result<Option<Elite>, AppError> {
        let con = self.db_pool.get().await.map_err(|_| DbConnectionError)?;

        let mut sets = Vec::new();
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_index = 1;

        if let Some(ref uuid) = updated_elite.minecraft_uuid {
            sets.push(format!("minecraft_uuid = ${}", param_index));
            params.push(uuid);
            param_index += 1;
        }
        if let Some(ref discord_user_id) = updated_elite.discord_user_id {
            sets.push(format!("discord_user_id = ${}", param_index));
            params.push(discord_user_id);
            param_index += 1;
        }
        if let Some(ref status) = updated_elite.status {
            sets.push(format!("status = ${}", param_index));
            params.push(status);
            param_index += 1;
        }
        if let Some(ref country_code) = updated_elite.country_code {
            sets.push(format!("country_code = ${}", param_index));
            params.push(country_code);
            param_index += 1;
        }
        if let Some(ref birthday) = updated_elite.birthday {
            sets.push(format!("birthday = ${}", param_index));
            params.push(birthday);
            param_index += 1;
        }

        if sets.is_empty() {
            return Err(NoFieldsToUpdate.into());
        }

        let query = format!("UPDATE elites SET {} WHERE id = ${}", sets.join(", "), param_index);
        params.push(&elite_id);

        let stmt = con.prepare_cached(&query).await.map_err(|e| CreatePreparedStatementError(e.to_string()))?;
        let updated_rows = con.execute(&stmt, &params).await.map_err(|e| DbError::QueryError(e.to_string()))?;
        if updated_rows == 0 {
            return Err(EliteNotFound(format!("Elite with id {elite_id} does not exist.")).into());
        }

        let query = "SELECT * FROM  elites_with_ign WHERE id = $1";
        let stmt = con.prepare_cached(&query).await.map_err(|e| CreatePreparedStatementError(e.to_string()))?;

        let updated_elite = con.query_opt(&stmt, &[&elite_id]).await.map_err(|e| DbError::QueryError(e.to_string()))?.map(|row| Elite::from(&row));

        Ok(updated_elite)
    }
}
