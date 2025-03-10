use crate::config::{DiscordConfig, RedisConfig};
use crate::model::discord::{DiscordAuthorizationCodeRequest, DiscordRefreshTokenRequest, DiscordToken, User};
use crate::web::error::Error;
use redis::{Commands, ExpireOption};
use redis_pool::{RedisPool, SingleRedisPool};
use reqwest::Client;
use tracing::debug;
use urlencoding::encode;

#[derive(Clone)]
pub struct DiscordAuthService {
    client: Client,
    api_version: String,
    client_id: String,
    client_secret: String,
    redirect_url: String,
    scopes: String,
    redis: SingleRedisPool,
}

impl DiscordAuthService {
    pub fn new(discord_config: &DiscordConfig, redis_config: &RedisConfig) -> Self {
        let redis = redis::Client::open(redis_config.url.to_string()).expect("Failed to create Redis client");
        let redis_pool = RedisPool::from(redis);

        Self {
            client: Client::new(),
            api_version: discord_config.api_version.clone(),
            client_id: discord_config.client_id.clone(),
            client_secret: discord_config.client_secret.clone(),
            redirect_url: discord_config.redirect_url.clone(),
            scopes: discord_config.scopes.clone(),
            redis: redis_pool,
        }
    }

    pub fn get_oauth_url(&self) -> String {
        self.oauth_url_for(&format!(
            "authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",
            &self.client_id,
            encode(&self.redirect_url),
            encode(&self.scopes)
        ))
    }

    pub async fn authenticate(&self, code: &str) -> Result<String, Error> {
        let token = self.get_discord_token_by_code(code).await;
        let user = self.get_discord_self_user(&token.access_token).await?;

        self.store_token(&user.id, &token).await?;

        Ok(user.id)
    }

    /// Exchanges the authorization code for an access token.
    pub async fn get_discord_token_by_code(&self, code: &str) -> DiscordToken {
        self.client
            .post(&self.api_url_for("oauth2/token"))
            .form(&DiscordAuthorizationCodeRequest {
                grant_type: "authorization_code".to_string(),
                code: code.to_string(),
                redirect_uri: self.redirect_url.to_string(),
            })
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .expect("Failed to send token request")
            .json::<DiscordToken>()
            .await
            .expect("Failed to deserialize token response")
    }

    /// Exchanges the refresh token for an access token.
    pub async fn get_discord_token_by_refresh_token(&self, refresh_token: &str) -> DiscordToken {
        self.client
            .post(&self.api_url_for("oauth2/token"))
            .form(&DiscordRefreshTokenRequest {
                grant_type: "refresh_token".to_string(),
                refresh_token: refresh_token.to_string(),
                redirect_uri: self.redirect_url.to_string(),
            })
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .expect("Failed to send token request")
            .json::<DiscordToken>()
            .await
            .expect("Failed to deserialize token response")
    }

    /// Calls Discord’s `/users/@me` endpoint with the given access token to fetch user info.
    pub async fn get_discord_self_user(&self, access_token: &str) -> Result<User, Error> {
        let user = self
            .client
            .get(&self.api_url_for("users/@me"))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| Error::DiscordApiError(e.to_string()))?
            .json::<User>()
            .await
            .map_err(|e| Error::DiscordApiError(e.to_string()))?;

        Ok(user)
    }

    fn api_url_for(&self, path: &str) -> String {
        format!("https://discord.com/api/v{}/{}", self.api_version, path)
    }

    fn oauth_url_for(&self, path: &str) -> String {
        format!("https://discord.com/oauth2/{}", path)
    }

    async fn store_token(&self, user_id: &str, token: &DiscordToken) -> Result<(), Error> {
        let mut conn = self.redis.get_connection().map_err(|_e| Error::RedisConnectionError)?;
        let user_key = format!("user:{}", user_id);
        let redis_operations = [("access_token", &token.access_token), ("refresh_token", &token.refresh_token)];

        debug!("Storing token information in Redis");
        let _: () = conn.hset_multiple(&user_key, &redis_operations).map_err(|e| {
                debug!("Failed to store tokens: {:?}", e);
                Error::RedisOperationError(e.to_string())
            })?;

        debug!("Setting token expiration");
        let _: () = conn.hexpire(&user_key, token.expires_in - 5, ExpireOption::NONE, "access_token")
            .map_err(|e| {
                debug!("Failed to set expiration: {:?}", e);
                Error::RedisOperationError(e.to_string())
            })?;

        Ok(())
    }
}
