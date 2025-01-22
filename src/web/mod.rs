pub mod middleware;

pub mod error;
pub mod routes_auth;
pub mod routes_auth_discord;

pub const ACCESS_TOKEN_COOKIE: &str = "auth-token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh-token";
