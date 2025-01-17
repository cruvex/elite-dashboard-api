use std::fmt;
use std::str::FromStr;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html, IntoResponse};
use redis::{Commands, ExpireOption, RedisResult};
use tracing::{debug, info};
use serde::{de, Deserialize, Deserializer};
use serde_json::{json, Value};
use tokio::fs;
use tower_http::services::ServeFile;
use crate::{AppState, Discord};
use crate::web::Result;

pub async fn oauth_url(State(state): State<AppState>) -> Result<Json<Value>> {
    Ok(Json(json!({
        "url": state.discord.get_oauth_url()
    })))
}

#[derive(Debug, Deserialize)]
pub struct DiscordCallbackQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    code: Option<String>,
}

pub async fn callback(
    State(state): State<AppState>,
    Query(params): Query<DiscordCallbackQueryParams>
) -> impl IntoResponse {
    debug!("{:>12} - {}", "HANDLER", "auth_discord_callback");
    match &params.code {
        Some(code) => {
            info!("Got code: {:?}", code);

            // Exchange the code for a Discord token
            let discord_token = state.discord.get_discord_token_by_code(&code).await;
            info!("Got token: {:?}", &discord_token);

            // Fetch the user using the access token
            let discord_user = state.discord.get_discord_self_user(&discord_token.access_token).await.unwrap();
            info!("Got user: {:?}", &discord_user);

            debug!("Getting redis connection");
            let mut conn = state.redis.get_connection().expect("Failed to retrieve connection from pool");

            // conn.hset("discord:tokens", discord_user.id, &serde_json::to_string(&discord_token).unwrap())
            //     .expect("Failed to store token in Redis");

            debug!("Setting token info in Redis");
            let user_key = format!("user:{}", discord_user.id);
            let _: () = conn.hset_multiple(&user_key, &[
                ("access_token", discord_token.access_token),
                ("refresh_token", discord_token.refresh_token),
            ]).expect("Failed to store token info ");

            debug!("Setting token expiration");
            let _: () = conn.hexpire(&user_key, discord_token.expires_in, ExpireOption::NONE, "access_token").expect("Failed to set expiration");
            debug!("Done");
        }
        None => {
            info!("No code provided.");
        }
    }

    let fallback = Html(r#"
        <html>
            <body>
                <script>
                    // If opened as a popup, this will close the window.
                    // Optionally notify the parent window with postMessage if needed.
                    window.opener?.postMessage({ type: "discordAuthComplete" }, "*");
                    window.close();
                    // If the window cannot be closed for whatever reason remove code from url bar
                    window.history.replaceState({}, document.title, '/auth/discord/callback');
                </script>
                <p>You can close this window.</p>
            </body>
        </html>
    "#.to_string());

    match fs::read_to_string("auth-redirect.html").await {
        Ok(contents) => Html(contents),
        Err(err) => {
            debug!("Failed to read 'auth-redirect.html': {}", err);
            fallback
        },
    }
}

fn empty_string_as_none<'de, D, T>(de: D) -> core::result::Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s)
            .map_err(de::Error::custom)
            .map(Some),
    }
}