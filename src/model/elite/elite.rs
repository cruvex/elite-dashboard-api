use chrono::NaiveDate;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Elite {
    pub id: i32,
    pub minecraft_uuid: Uuid,
    pub ign: String,
    pub being_tracked: bool,
    pub discord_user_id: String,
    pub status: String,
    pub country_code: String,
    pub birthday: NaiveDate,
}

impl From<Row> for Elite {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            minecraft_uuid: row.get::<_, Uuid>("minecraft_uuid"),
            ign: row.get("ign"),
            being_tracked: row.get("being_tracked"),
            discord_user_id: row.get("discord_user_id"),
            status: row.get("status"),
            country_code: row.get("country_code"),
            birthday: row.get::<_, NaiveDate>("birthday"),
        }
    }
}
