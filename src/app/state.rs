use crate::app::config::AppConfig;
use crate::error::Error;
use crate::service::{DiscordApiService, DiscordAuthService, EliteService, IgnTrackerService, SessionService};
use axum::extract::FromRef;
use axum_macros::FromRef;
use deadpool_postgres::Pool;
use redis::aio::ConnectionManager;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub discord: DiscordState,
    pub session: SessionService,
    pub elite: EliteService,
    pub ign_tracker: IgnTrackerService,
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
    pub async fn initialize(db_pool: Pool, redis: ConnectionManager, config: &AppConfig) -> Result<Self, Error> {
        let session = SessionService::new(&config.session, redis);

        let elite = EliteService::new(db_pool.clone());

        let discord_api = DiscordApiService::new(&config.discord);
        let discord_auth = DiscordAuthService::new(&config.discord);

        let ign_tracker = IgnTrackerService::new(db_pool.clone());

        Ok(Self {
            discord: DiscordState {
                api: discord_api,
                auth: discord_auth,
            },
            session,
            elite,
            ign_tracker,
        })
    }
}
