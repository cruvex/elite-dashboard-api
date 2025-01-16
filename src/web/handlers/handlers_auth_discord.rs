use std::fmt;
use std::str::FromStr;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html, IntoResponse};
use log::debug;
use serde::{de, Deserialize, Deserializer};
use serde_json::{json, Value};
use tokio::fs;
use tower_http::services::ServeFile;
use crate::{AppState, Discord};

pub async fn redirect_url(State(state): State<AppState>) -> crate::web::Result<Json<Value>> {
    Ok(Json(json!({
        "url": Discord::get_oauth_url(&state.config.discord)
    })))
}

#[derive(Debug, Deserialize)]
pub struct DiscordCallbackQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    code: Option<String>,
}

pub async fn callback(Query(params): Query<DiscordCallbackQueryParams>) -> impl IntoResponse {
    debug!("{:>12} - {}", "HANDLER", "auth_discord_callback");
    debug!("Discord callback: {:?}", params);

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
    "#);

    match fs::read_to_string("auth-redirect.html").await {
        Ok(contents) => Html(contents),
        Err(err) => {
            debug!("Failed to read 'auth-redirect.html': {}", err);
            fallback
        },
    }
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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