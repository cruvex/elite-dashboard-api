use deadpool_postgres::tokio_postgres::Row;
use serde::Serialize;

#[derive(Serialize)]
pub struct Elite {
    pub id: i32,
    pub ign: String,
    pub country_code: String,
    pub discord_id: String,
}

impl From<Row> for Elite {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            ign: row.get("ign"),
            country_code: row.get("country_code"),
            discord_id: row.get("discord_user_id"),
        }
    }
}
