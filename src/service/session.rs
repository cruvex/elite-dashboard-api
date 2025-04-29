use crate::app::config::SessionConfig;
use crate::app::constants::{
    CSRF_TOKEN_KEY, DISCORD_ACCESS_TOKEN_KEY, DISCORD_REFRESH_TOKEN_KEY, FIVE_MINUTES, ONE_MONTH, SESSION_COOKIE_NAME, SESSION_KEY_PREFIX,
    USER_ID_KEY, USER_ROLE_KEY,
};
use crate::app::error::AppError;
use crate::model::session::{Session, UserRole};
use crate::web::error::Error;
use hex::encode;
use oauth2::basic::BasicTokenResponse;
use oauth2::{CsrfToken, TokenResponse};
use rand::RngCore;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, ErrorKind, ExpireOption, pipe};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use tower_cookies::Cookie;
use tower_cookies::cookie::SameSite;
use tracing::debug;

#[derive(Clone)]
pub struct SessionService {
    redis: Arc<ConnectionManager>,
    pub secure_cookie: bool,
}

impl SessionService {
    pub fn new(session_config: &SessionConfig, redis: ConnectionManager) -> Self {
        Self {
            redis: Arc::new(redis),
            secure_cookie: session_config.secure_cookie,
        }
    }
}

impl SessionService {
    pub async fn init_session(&self, csrf_token: &CsrfToken) -> Result<String, AppError> {
        let mut con = self.redis.as_ref().clone();

        let session_id = self.generate_session_id();
        let session_key = format!("{}:{}", SESSION_KEY_PREFIX, session_id);

        let _: () = pipe()
            .hset(&session_key, CSRF_TOKEN_KEY, csrf_token.secret())
            .expire(&session_key, FIVE_MINUTES)
            .query_async(&mut con)
            .await
            .map_err(|e| Error::RedisOperationError(e.to_string()))?;

        Ok(session_id)
    }

    pub async fn validate_session(&self, session_id: &String) -> Result<Session, AppError> {
        let session = self.get_session_by_id(session_id).await?.ok_or(Error::SessionNotFound)?;

        Ok(session)
    }

    pub async fn validate_init_session(&self, session_id: &String, csrf_token: &CsrfToken) -> Result<(), AppError> {
        let mut con = self.redis.as_ref().clone();
        let session_key = format!("{}:{}", SESSION_KEY_PREFIX, session_id);

        // Check if the session exists with cookie session id and state csrf_token
        match con.hget::<_, _, Option<String>>(&session_key, CSRF_TOKEN_KEY).await {
            Ok(Some(token)) if token.as_str() == csrf_token.secret() => Ok(()),
            Ok(_) => Err(Error::SessionNotFound.into()),
            Err(err) => Err(Error::RedisOperationError(err.to_string()).into()),
        }
    }

    pub async fn invalidate_session(&self, session_id: &String) -> Result<(), AppError> {
        let mut con = self.redis.as_ref().clone();
        let session_key = format!("{}:{}", SESSION_KEY_PREFIX, session_id);

        let _: () = con.del::<_, ()>(&session_key).await.map_err(|e| Error::RedisOperationError(e.to_string()))?;

        Ok(())
    }

    pub async fn refresh_session_ttl(&self, session_id: &str) -> Result<(), AppError> {
        let mut con = self.redis.as_ref().clone();
        let session_key = format!("{}:{}", SESSION_KEY_PREFIX, session_id);

        let _: () = con.expire(&session_key, ONE_MONTH).await.map_err(|e| Error::RedisOperationError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_session_by_id(&self, session_id: &String) -> Result<Option<Session>, AppError> {
        let mut con = self.redis.as_ref().clone();
        let session_key = format!("{}:{}", SESSION_KEY_PREFIX, session_id);

        let session_exists = con.exists::<_, bool>(&session_key).await.map_err(|e| Error::RedisOperationError(e.to_string()))?;

        if !session_exists {
            return Err(Error::SessionNotFound.into());
        }

        let session = match con.hgetall::<_, Session>(&session_key).await {
            Ok(session) => Ok(Some(session)),
            Err(e) if e.kind() == ErrorKind::TypeError => Err(Error::InvalidSession(e.to_string())),
            Err(e) => Err(Error::RedisOperationError(e.to_string())),
        }?;

        Ok(session)
    }

    pub fn create_session_cookie(&self, session_id: String, expires_in: i64) -> Cookie<'static> {
        Cookie::build((SESSION_COOKIE_NAME, session_id))
            .path("/")
            .secure(self.secure_cookie)
            .http_only(true)
            .same_site(SameSite::Lax)
            .expires(OffsetDateTime::now_utc() + Duration::seconds(expires_in))
            .build()
    }

    pub async fn save_session(
        &self,
        session_id: &String,
        tokens: &BasicTokenResponse,
        user_id: &String,
        user_role: &UserRole,
    ) -> Result<(), AppError> {
        let mut con = self.redis.as_ref().clone();
        let session_key = format!("{}:{}", SESSION_KEY_PREFIX, session_id);

        let session_fields = [
            (USER_ID_KEY, user_id),
            (USER_ROLE_KEY, &user_role.to_string()),
            (DISCORD_ACCESS_TOKEN_KEY, tokens.access_token().secret()),
            (DISCORD_REFRESH_TOKEN_KEY, tokens.refresh_token().unwrap().secret()),
        ];

        let discord_access_token_expires_in = i64::try_from(tokens.expires_in().unwrap().as_secs()).unwrap() - 5;

        debug!("Saving session - {} - {}", &user_id, &user_role.to_string());
        let _: () = pipe()
            .hset_multiple(&session_key, &session_fields)
            .hexpire(
                &session_key,
                discord_access_token_expires_in,
                ExpireOption::NONE,
                DISCORD_ACCESS_TOKEN_KEY,
            )
            .expire(&session_key, ONE_MONTH)
            .query_async(&mut con)
            .await
            .map_err(|e| Error::RedisOperationError(e.to_string()))?;

        Ok(())
    }

    pub fn generate_session_id(&self) -> String {
        let mut bytes = [0u8; 512];
        rand::rng().fill_bytes(&mut bytes);
        encode(bytes)
    }
}
