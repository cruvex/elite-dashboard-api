pub mod app;

use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
}

#[serde_inline_default]
#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    #[serde_inline_default(String::from("::"))]
    pub address: String,
    #[serde_inline_default(8080)]
    pub port: u16,
}

#[serde_inline_default]
#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde_inline_default(10)]
    pub max_pool_size: usize,
}

#[serde_inline_default]
#[derive(Deserialize, Clone)]
pub struct DiscordConfig {
    pub api_version: String,
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: String,
    pub bot_token: String,
    pub elite_guild_id: String,
}

#[serde_inline_default]
#[derive(Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[serde_inline_default]
#[derive(Deserialize, Clone)]
pub struct JwtConfig {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    #[serde_inline_default(3600)]
    pub access_token_exp: usize,
    #[serde_inline_default(604800)]
    pub refresh_token_exp: usize,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        let server = envy::prefixed("SERVER_").from_env::<ServerConfig>()?;
        let database = envy::prefixed("DATABASE_").from_env::<DatabaseConfig>()?;
        let discord = envy::prefixed("DISCORD_").from_env::<DiscordConfig>()?;
        let redis = envy::prefixed("REDIS_").from_env::<RedisConfig>()?;
        let jwt = envy::prefixed("JWT_").from_env::<JwtConfig>()?;

        Ok(AppConfig {
            server,
            database,
            discord,
            redis,
            jwt,
        })
    }
}
