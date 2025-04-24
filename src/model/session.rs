use crate::app::constants::{DISCORD_ACCESS_TOKEN_KEY, DISCORD_REFRESH_TOKEN_KEY, USER_ID_KEY, USER_ROLE_KEY};
use redis::{FromRedisValue, RedisError, RedisResult, Value};
use serde::Serialize;
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Clone)]
pub struct Session {
    pub user: SessionUser,
    pub discord: DiscordTokens,
}

#[derive(Debug, Serialize, Clone)]
pub struct SessionUser {
    pub id: String,
    pub role: UserRole,
}

impl SessionUser {
    pub fn is_staff(&self) -> bool {
        matches!(self.role, UserRole::Staff)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct DiscordTokens {
    pub access_token: Option<String>,
    pub refresh_token: String,
}

#[derive(EnumString, Display, Debug, Serialize, Clone)]
pub enum UserRole {
    #[strum(serialize = "staff")]
    Staff,
    #[strum(serialize = "elite")]
    Elite,
    #[strum(serialize = "bot")]
    Bot,
}

impl FromRedisValue for Session {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let map: HashMap<String, String> = FromRedisValue::from_redis_value(v)?;

        let user_id = map.get(USER_ID_KEY).ok_or_else(|| RedisError::from((redis::ErrorKind::TypeError, "Missing user_id")))?.to_string();

        let user_role = map
            .get(USER_ROLE_KEY)
            .ok_or_else(|| RedisError::from((redis::ErrorKind::TypeError, "Missing user_role")))?
            .parse::<UserRole>()
            .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "Invalid role", format!("Invalid role: {e}"))))?;

        let refresh_token = map
            .get(DISCORD_REFRESH_TOKEN_KEY)
            .ok_or_else(|| RedisError::from((redis::ErrorKind::TypeError, "Missing refresh_token")))?
            .to_string();

        let access_token = map.get(DISCORD_ACCESS_TOKEN_KEY).cloned();

        Ok(Self {
            user: SessionUser {
                id: user_id,
                role: user_role,
            },
            discord: DiscordTokens { access_token, refresh_token },
        })
    }
}
