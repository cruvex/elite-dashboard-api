use crate::app::error::AppError;
use crate::config::{JwtConfig, RedisConfig};
use crate::db::redis::RedisConnection;
use crate::model::discord::User;
use crate::web::error::Error;
use hex::encode;
use oauth2::CsrfToken;
use oauth2::basic::BasicTokenResponse;
use rand::RngCore;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

#[derive(Clone)]
pub struct SessionService {
    redis: Arc<Mutex<RedisConnection>>,
    pub secure_cookie: bool,
}

impl SessionService {
    pub fn new(jwt_config: &JwtConfig, redis: RedisConnection) -> Self {
        Self {
            redis: Arc::new(Mutex::new(redis)),
            secure_cookie: jwt_config.secure_cookie,
        }
    }
}

impl SessionService {
    pub async fn init_session(&self, csrf_token: &CsrfToken) -> String {
        let mut con = self.redis.lock().await;

        let session_id = self.generate_session_id();
        let session_key = format!("session:{}", session_id);

        //TODO: implement better error handling
        let _: () = con.hset(session_key, "csrf_token", csrf_token.secret()).await.expect("Failed to set csrf token");

        session_id
    }

    pub async fn validate_session(&self, session_id: &str, csrf_token: CsrfToken) -> Result<bool, AppError> {
        let mut con = self.redis.lock().await;
        let session_key = format!("session:{}", session_id);

        match con.hget::<_, _, Option<String>>(session_key, "csrf_token").await {
            Ok(Some(token)) if token == csrf_token.secret().to_string() => Ok(true),
            _ => Err(Error::NoSessionFound.into()),
        }
    }

    pub fn get_session_by_id(&self, session_id: String) {
        todo!()
    }

    pub fn generate_session_id(&self) -> String {
        let mut bytes = [0u8; 512];
        rand::rng().fill_bytes(&mut bytes);
        encode(bytes)
    }
}
