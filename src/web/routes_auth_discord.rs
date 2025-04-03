use axum::extract::{Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use oauth2::{CsrfToken, Scope};
use serde::Deserialize;
use serde_json::{Value, json};
use serde_with::NoneAsEmptyString;
use serde_with::serde_as;
use tokio::fs;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::AppState;
use crate::app::error::Result;
use crate::service::DiscordAuthService;
use crate::service::jwt::Claims;
use crate::web::error::Error;
use crate::web::{ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE};

pub fn routes(state: AppState) -> Router {
    Router::new().route("/auth/discord", get(auth_discord)).route("/auth/discord/callback", get(auth_discord_callback)).with_state(state)
}

pub async fn auth_discord(State(discord_auth): State<DiscordAuthService>) -> Result<Json<Value>> {
    debug!("{:<12} - {}", "HANDLER", "auth_discord");
    let (auth_url, csrf_token) = discord_auth.init_auth();

    Ok(Json(json!({
        "url": auth_url
    })))
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct DiscordCallbackQueryParams {
    #[serde_as(as = "NoneAsEmptyString")]
    code: Option<String>,
}

pub async fn auth_discord_callback(
    cookies: Cookies,
    State(state): State<AppState>,
    Query(params): Query<DiscordCallbackQueryParams>,
) -> Result<Html<String>> {
    debug!("{:<12} - {}", "HANDLER", "auth_discord_callback");

    let code = params.code.as_ref().ok_or_else(|| {
        debug!("No authorization code provided");
        Error::NoDiscordCodeInPath
    })?;

    // let user_id = state.discord.auth.authenticate(code).await?;
    //
    // let mut claims = Claims::new(&user_id);
    //
    // let access_token = state.jwt.generate_access_token(&mut claims).map_err(|_e| Error::JwtTokenGenerationError)?;
    // let mut access_token_cookie = Cookie::new(ACCESS_TOKEN_COOKIE, access_token);
    // access_token_cookie.set_http_only(true);
    // access_token_cookie.set_path("/");
    // access_token_cookie.set_same_site(SameSite::Strict);
    // access_token_cookie.set_secure(state.jwt.secure_cookie);
    //
    // let refresh_token = state.jwt.generate_refresh_token(&mut claims).map_err(|_e| Error::JwtTokenGenerationError)?;
    // let mut refresh_token_cookie = Cookie::new(REFRESH_TOKEN_COOKIE, refresh_token);
    // refresh_token_cookie.set_http_only(true);
    // refresh_token_cookie.set_path("/auth/refresh");
    // refresh_token_cookie.set_same_site(SameSite::Strict);
    // refresh_token_cookie.set_secure(state.jwt.secure_cookie);
    //
    // cookies.add(access_token_cookie);
    // cookies.add(refresh_token_cookie);
    //
    // // TODO: Properly implement response html with error state
    // // Return the appropriate HTML response
    render_callback_response().await
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
