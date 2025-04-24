use crate::app::error::AppError;
use crate::db::error::DbError;
use crate::model::recent_change::RecentChange;
use crate::service::error::ServiceError::CreatePreparedStatementError;
use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct IgnTrackerService {
    db_pool: Pool,
}

impl IgnTrackerService {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}

impl IgnTrackerService {
    pub async fn get_latest_changes(&self, limit: i64, offset: i64) -> Result<Vec<RecentChange>, AppError> {
        let con = self.db_pool.get().await.map_err(|_| DbError::ConnectionError)?;

        let stmt = con
            .prepare_cached(
                "
                SELECT
                    uuid,
                    old_ign,
                    new_ign,
                    timestamp,
                    event_type
                FROM (
                    SELECT
                        uuid,
                        LAG(ign) OVER (PARTITION BY uuid ORDER BY timestamp) AS old_ign,
                        ign AS new_ign,
                        timestamp,
                        'IGN_CHANGE' AS event_type
                    FROM
                        name_history
                ) AS ign_changes
                WHERE
                    old_ign IS NOT NULL
                    AND old_ign <> new_ign
                UNION ALL
                SELECT
                    tuh.uuid,
                    NULL AS old_ign,
                    nh_at_event.ign AS new_ign,
                    tuh.timestamp,
                    tuh.operation AS event_type
                FROM
                    tracked_uuids_history tuh
                    LEFT JOIN LATERAL (
                        SELECT
                            nh.ign
                        FROM
                            name_history nh
                        WHERE
                            nh.uuid = tuh.uuid
                            AND nh.timestamp <= tuh.timestamp
                        ORDER BY
                            nh.timestamp DESC
                        LIMIT 1
                    ) nh_at_event ON TRUE
                ORDER BY
                    timestamp DESC
                LIMIT
                    $1
                OFFSET
                    $2;
            ",
            )
            .await
            .map_err(|e| CreatePreparedStatementError(e.to_string()))?;

        let rows = con.query(&stmt, &[&limit, &offset]).await.map_err(|e| DbError::QueryError(e.to_string()))?;

        let latest_changes: Vec<RecentChange> = rows.into_iter().map(RecentChange::from).collect();

        Ok(latest_changes)
    }
}
