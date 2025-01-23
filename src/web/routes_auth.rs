use crate::service::JwtService;
use crate::web::error::Error;
use crate::web::REFRESH_TOKEN_COOKIE;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tower_cookies::Cookies;
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/auth/refresh", post(auth_refresh)).with_state(state)
}

pub async fn auth_refresh(cookies: Cookies, State(jwt): State<JwtService>) -> Result<impl IntoResponse, Error> {
    debug!("{:<12} - {}", "HANDLER", "auth_refresh");

    let refresh_token = cookies
        .get(REFRESH_TOKEN_COOKIE)
        .ok_or(Error::CookieNotFound)?
        .value()
        .to_string();

    debug!("Refresh token: {}", refresh_token);

    let claims = jwt.validate_refresh_token(&refresh_token);

    Ok((StatusCode::OK, "balbla").into_response())
}
