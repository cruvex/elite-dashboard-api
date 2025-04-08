use axum::extract::{Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use oauth2::{CsrfToken, TokenResponse};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::fs;
use tower_cookies::Cookies;
use tracing::{debug, warn};

use crate::AppState;
use crate::app::error::Result;
use crate::service::constant::{FIVE_MINUTES, ONE_MONTH, SESSION_COOKIE};
use crate::service::{DiscordApiService, DiscordAuthService, SessionService};
use crate::web::error::Error;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/auth/discord", get(auth_discord))
        .route("/auth/discord/callback", get(auth_discord_callback))
        .with_state(state)
}

pub async fn auth_discord(
    State(discord_auth): State<DiscordAuthService>,
    State(session): State<SessionService>,
    cookies: Cookies,
) -> Result<Json<Value>> {
    debug!("{:<12} - {}", "HANDLER", "auth_discord");
    let (auth_url, csrf_token) = discord_auth.init_auth();

    let session_id = session.init_session(&csrf_token).await?;

    // Create and set session cookie
    // User has to complete initial auth flow within 5 minutes. When auth flow succeeds session cookie expiration will be increased
    let session_cookie = session.create_session_cookie(session_id.clone(), FIVE_MINUTES);
    cookies.add(session_cookie);

    // csrf_token is set in 'state' query parameter
    Ok(Json(json!({
        "url": auth_url
    })))
}

#[derive(Debug, Deserialize)]
pub struct DiscordCallbackQueryParams {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub async fn auth_discord_callback(
    State(session_store): State<SessionService>,
    State(discord_auth): State<DiscordAuthService>,
    State(discord_api): State<DiscordApiService>,
    Query(params): Query<DiscordCallbackQueryParams>,
    cookies: Cookies,
) -> Result<Html<String>> {
    debug!("{:<12} - {}", "HANDLER", "auth_discord_callback");

    // Check for error
    if params.error.is_some() || params.error_description.is_some() {
        return Ok(handle_callback_error(&params.error, &params.error_description));
    }

    // code and state are required in callback
    let code = params.code.ok_or(Error::NoCodeInDiscordCallbackPath)?;
    let state = params.state.ok_or(Error::NoStateInDiscordCallbackPath)?;

    let session_cookie = cookies.get(SESSION_COOKIE).ok_or(Error::SessionCookieNotFound)?;
    let session_id = session_cookie.value().to_string();

    debug!("{:<12} - Session ID: {:?}", "HANDLER", &session_id);

    // Check valid session by session_id and state (csrf_token)
    session_store.validate_init_session(&session_id, &CsrfToken::new(state.clone())).await?;

    debug!("{:<12} - Validated session", "HANDLER");

    let tokens = discord_auth.exchanged_code_for_tokens(&code).await?;

    debug!("{:<12} - Exchanged code for tokens", "HANDLER");

    let self_user_id = discord_auth.get_discord_self_user_id(tokens.access_token()).await?;

    let elite_member = discord_api.get_elite_guild_member(&self_user_id).await?.ok_or(Error::NotInEliteGuild)?;

    let user_role = discord_auth.get_role_for_member(&elite_member.roles).ok_or(Error::NotInElite)?;

    session_store.save_session(&session_id, &tokens, &self_user_id, &user_role).await?;

    // Discord oauth flow successful. Session cookie valid for 1 month
    let session_cookie = session_store.create_session_cookie(session_id.clone(), ONE_MONTH);
    cookies.add(session_cookie);

    // TODO: Properly implement response html with error state
    // Return the appropriate HTML response
    render_callback_response().await
}

fn handle_callback_error(error: &Option<String>, error_description: &Option<String>) -> Html<String> {
    warn!(
        "{:<12} - Discord Oauth flow failed: Error: {:?} - Description: {:?}",
        "HANDLER", error, error_description
    );
    // Html(DISCORD_AUTH_FAILED_HTML)
    todo!()
}

async fn render_callback_response() -> Result<Html<String>> {
    let fallback = generate_fallback_html();

    match fs::read_to_string("auth-redirect.html").await {
        Ok(contents) => Ok(Html(contents)),
        Err(err) => {
            debug!("Failed to read 'auth-redirect.html': {}", err);
            Ok(Html(fallback))
        }
    }
}

fn generate_fallback_html() -> String {
    r#"
        <html>
            <body>
                <script>
                    window.opener?.postMessage({ type: "discordAuthComplete" }, "*");
                    window.close();
                    window.history.replaceState({}, document.title, '/auth/discord/callback');
                </script>
                <p>You can close this window.</p>
            </body>
        </html>
    "#
    .to_string()
}
