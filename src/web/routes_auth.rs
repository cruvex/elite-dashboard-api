use crate::service::jwt::Claims;
use crate::{service::JwtService, web::ACCESS_TOKEN_COOKIE};
use crate::web::error::Error;
use crate::web::REFRESH_TOKEN_COOKIE;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/auth/refresh", post(auth_refresh)).with_state(state)
}

pub async fn auth_refresh(cookies: Cookies, State(jwt): State<JwtService>) -> Result<impl IntoResponse, Error> {
    debug!("{:<12} - {}", "HANDLER", "auth_refresh");

    debug!("Secure: {}", jwt.secure_cookie);

    let refresh_token = cookies
        .get(REFRESH_TOKEN_COOKIE)
        .ok_or(Error::AuthCookieNotFound)?
        .value()
        .to_string();

    debug!("Refresh token: {}", refresh_token);

    let old_claims = jwt.validate_refresh_token(&refresh_token)?;

    let mut claims = Claims::new(&old_claims.sub);

    let access_token =
        jwt
        .generate_access_token(&mut claims)
        .map_err(|_e| Error::JwtTokenGenerationError)?;
    let mut access_token_cookie = Cookie::new(ACCESS_TOKEN_COOKIE, access_token);
    access_token_cookie.set_http_only(true);
    access_token_cookie.set_path("/");
    access_token_cookie.set_same_site(SameSite::Strict);
    access_token_cookie.set_secure(jwt.secure_cookie);

    let refresh_token =
        jwt
        .generate_refresh_token(&mut claims)
        .map_err(|_e| Error::JwtTokenGenerationError)?;
    let mut refresh_token_cookie = Cookie::new(REFRESH_TOKEN_COOKIE, refresh_token);
    refresh_token_cookie.set_http_only(true);
    refresh_token_cookie.set_path("/auth/refresh");
    refresh_token_cookie.set_same_site(SameSite::Strict);
    refresh_token_cookie.set_secure(jwt.secure_cookie);

    cookies.add(access_token_cookie);
    cookies.add(refresh_token_cookie);

    Ok(StatusCode::OK.into_response())
}
