use config::{Case, Config, Environment};
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use std::fmt::Error;

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
    pub secure_cookie: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Error> {
        let config =
            Config::builder().add_source(Environment::default().separator("__").convert_case(Case::Snake)).build().expect("Failed to build config");

        let config = config.try_deserialize::<AppConfig>().expect("Failed to deserialize config");

        Ok(config)
    }
}
