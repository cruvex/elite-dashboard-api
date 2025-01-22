use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DiscordAuthorizationCodeRequest {
    pub grant_type: String, // "authorization_code"
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize)]
pub struct DiscordRefreshTokenRequest {
    pub grant_type: String, // "refresh_token"
    pub refresh_token: String,
    pub redirect_uri: String,
}

/// Represents the Discord OAuth2 token response.
#[derive(Debug, Deserialize)]
pub struct DiscordToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}
