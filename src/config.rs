use dotenv::dotenv;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

#[serde_inline_default]
#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    #[serde_inline_default(String::from("::"))]
    pub address: String,
    #[serde_inline_default(8080)]
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();

        let server = envy::prefixed("SERVER_").from_env::<ServerConfig>()?;

        Ok(Config { server })
    }
}
