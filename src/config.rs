use dotenv::dotenv;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
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
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: String,

}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();

        let server = envy::prefixed("SERVER_").from_env::<ServerConfig>()?;
        let database = envy::prefixed("DATABASE_").from_env::<DatabaseConfig>()?;
        let discord = envy::prefixed("DISCORD_").from_env::<DiscordConfig>()?;

        Ok(Config { server, database, discord })
    }
}
