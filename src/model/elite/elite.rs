use chrono::NaiveDate;
use serde::Serialize;
use std::error::Error;
use strum_macros::EnumString;
use tokio_postgres::Row;
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type, to_sql_checked};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Elite {
    pub id: i32,
    pub minecraft_uuid: Uuid,
    pub ign: Option<String>, // FIXME - theoretically there should be no null values here but there are for now :)
    pub being_tracked: bool,
    pub discord_user_id: String,
    pub status: EliteStatus,
    pub country_code: String,
    pub birthday: Option<NaiveDate>,
}

impl From<&Row> for Elite {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            minecraft_uuid: row.get::<_, Uuid>("minecraft_uuid"),
            ign: row.get("ign"),
            being_tracked: row.get("being_tracked"),
            discord_user_id: row.get("discord_user_id"),
            status: row.get("status"),
            country_code: row.get("country_code"),
            birthday: row.get::<_, Option<NaiveDate>>("birthday"),
        }
    }
}

#[derive(Debug, Serialize, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum EliteStatus {
    #[strum(serialize = "staff")]
    Staff,
    #[strum(serialize = "veteran")]
    Veteran,
    #[strum(serialize = "elite")]
    Elite,
    #[strum(serialize = "trial")]
    Trial,
    #[strum(serialize = "none")]
    None,
}

impl ToSql for EliteStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Ensure we're writing to a TEXT-compatible type
        if *ty != Type::TEXT {
            return Err(format!("invalid type: expected TEXT but got {}", ty).into());
        }

        let s = match self {
            EliteStatus::Staff => "staff",
            EliteStatus::Veteran => "veteran",
            EliteStatus::Elite => "elite",
            EliteStatus::Trial => "trial",
            EliteStatus::None => "none",
        };

        out.extend_from_slice(s.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        *ty == Type::TEXT
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for EliteStatus {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match std::str::from_utf8(raw)? {
            "staff" => Ok(EliteStatus::Staff),
            "veteran" => Ok(EliteStatus::Veteran),
            "elite" => Ok(EliteStatus::Elite),
            "trial" => Ok(EliteStatus::Trial),
            "none" => Ok(EliteStatus::None),
            other => Err(format!("unknown status: {}", other).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::TEXT || *ty == Type::VARCHAR
    }
}

// TODO - look into using `TryFrom` for better error handling
// impl TryFrom<&Row> for Elite {
//     type Error = AppError;
//
//     fn try_from(row: &Row) -> Result<Self, Self::Error> {
//         Self {
//             id: row.get("id"),
//             minecraft_uuid: row.get::<_, Uuid>("minecraft_uuid"),
//             ign: row.get("ign"),
//             discord_user_id: row.get("discord_user_id"),
//             status: row.get("status"),
//             country_code: row.get("country_code"),
//             birthday: row.get::<_, NaiveDate>("birthday"),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum EliteRowMapperError {
//     #[allow(dead_code)] // FIXME
//     InvalidField(String),
// }

// impl From<EliteRowMapperError> for AppError {
//     fn from(value: EliteRowMapperError) -> Self {
//         trace!("{:<12} - {value:?}", "FROM_APP_ERR");
//
//         match value {
//             EliteRowMapperError::InvalidField(_) => AppError::InternalServerError,
//         }
//     }
// }
//
// impl TryFrom<&Row> for Elite {
//     type Error = EliteRowMapperError;
//
//     fn try_from(row: &Row) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: row.try_get("id").map_err(|e| EliteRowMapperError::InvalidField(e.to_string()))?,
//             minecraft_uuid: row.try_get("minecraft_uuid").map_err(|e| EliteRowMapperError::InvalidField(e.to_string()))?,
//             ign: row.try_get("ign").map_err(|e| EliteRowMapperError::InvalidField(e.to_string()))?,
//             being_tracked: row.try_get("being_tracked").map_err(|e| EliteRowMapperError::InvalidField(e.to_string()))?,
//             discord_user_id: row.try_get("discord_user_id").map_err(|e| EliteRowMapperError::InvalidField(e.to_string()))?,
//             status: row.try_get("status").map_err(|e| EliteRowMapperError::InvalidField(e.to_string()))?,
//             country_code: row.try_get("country_code").map_err(|e| EliteRowMapperError::InvalidField(e.to_string()))?,
//             birthday: row.try_get("birthday").map_err(|e| {
//
//                 trace!("{:<12} - {:?}", "TRY_FROM", e);
//                 EliteRowMapperError::InvalidField("test".to_string())
//             })?,
//         })
//     }
// }

// impl TryFrom
