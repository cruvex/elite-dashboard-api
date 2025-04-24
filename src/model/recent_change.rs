use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct RecentChange {
    pub uuid: Uuid,
    pub old_ign: String,
    pub new_ign: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
}

impl From<Row> for RecentChange {
    fn from(row: Row) -> Self {
        Self {
            uuid: row.get::<_, Uuid>("uuid"),
            old_ign: row.get("old_ign"),
            new_ign: row.get("new_ign"),
            timestamp: row.get("timestamp"),
            event_type: row.get("event_type"),
        }
    }
}
