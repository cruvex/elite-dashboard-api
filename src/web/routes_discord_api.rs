use crate::app::error::Result;
use crate::app::state::{AppState, DiscordState};
use crate::web::error::Error;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/discord/guild/elite", get(elite_guild)).route("/discord/elite/member/{user_id}", get(elite_member)).with_state(state)
}

pub async fn elite_guild(State(discord): State<DiscordState>) -> Result<impl IntoResponse> {
    debug!("{:<12} - {}", "HANDLER", "discord_guild_elite");

    let guild = discord.api.get_elite_guild().await.map_err(|e| Error::DiscordApiError(e.to_string()))?;
    Ok((StatusCode::OK, Json(json!({"guild": guild}))).into_response())
}

pub async fn elite_member(State(discord): State<DiscordState>, Path(user_id): Path<String>) -> Result<impl IntoResponse> {
    debug!("{:<12} - {}", "HANDLER", "discord_guild_elite");

    let member = discord.api.get_elite_guild_member(&user_id).await.map_err(|e| Error::DiscordApiError(e.to_string()))?;

    Ok((StatusCode::OK, Json(json!({"member": member}))).into_response())
}
