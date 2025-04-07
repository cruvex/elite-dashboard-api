use crate::config::AppConfig;
use crate::error::Error;
use crate::service::{DiscordApiService, DiscordAuthService, EliteService, SessionService};
use axum::extract::FromRef;
use axum_macros::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub discord: DiscordState,
    pub session: SessionService,
    pub elite: EliteService,
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

impl FromRef<AppState> for DiscordApiService {
    fn from_ref(state: &AppState) -> Self {
        state.discord.api.clone()
    }
}

impl AppState {
    /// Initialize the application state with all required services.
    pub async fn initialize(config: &AppConfig) -> Result<Self, Error> {
        let redis = crate::db::redis::init_redis(&config.redis).await.expect("Failed to init Redis");
        let db_pool = crate::db::init_db(&config.database).await.expect("Failed to initialize database pool");

        let session = SessionService::new(&config.session, redis);

        let elite = EliteService::new(db_pool.clone());

        let discord_api = DiscordApiService::new(&config.discord);
        let discord_auth = DiscordAuthService::new(&config.discord);

        Ok(Self {
            discord: DiscordState {
                api: discord_api,
                auth: discord_auth,
            },
            session,
            elite,
        })
    }
}
