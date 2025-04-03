use crate::config::AppConfig;
use crate::error::Error;
use crate::service::{DiscordApiService, DiscordAuthService, JwtService};
use axum::extract::FromRef;
use axum_macros::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub jwt: JwtService,
    pub discord: DiscordState,
}

#[derive(Clone, FromRef)]
pub struct DiscordState {
    pub api: DiscordApiService,
    pub auth: DiscordAuthService,
}

impl FromRef<AppState> for DiscordAuthService {
    fn from_ref(state: &AppState) -> Self {
        state.discord.auth.clone()
    }
}

impl AppState {
    /// Initialize the application state with all required services.
    pub async fn initialize(config: &AppConfig) -> Result<Self, Error> {
        let jwt = JwtService::new(&config.jwt);

        // Discord
        let discord_api = DiscordApiService::new(&config.discord);

        let discord_auth = DiscordAuthService::new(&config.discord, &config.redis);

        let discord = DiscordState {
            api: discord_api,
            auth: discord_auth,
        };

        Ok(Self { jwt, discord })
    }
}
