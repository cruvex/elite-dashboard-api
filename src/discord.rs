use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use urlencoding::encode;
use crate::config::DiscordConfig;

#[derive(Clone)]
pub struct Discord {
    pub client: Client,
    pub api_version: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub scopes: String,
}

impl Discord {

    pub fn new(config: &DiscordConfig) -> Self {
        Self {
            client: Client::new(),
            api_version: config.api_version.clone(),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            redirect_url: config.redirect_url.clone(),
            scopes: config.scopes.clone(),
        }
    }

    pub fn get_oauth_url(&self) -> String {
        format!("{}/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}", &self.base_oauth_url(), &self.client_id, encode(&self.redirect_url), encode(&self.scopes))
    }

    /// Exchanges the authorization code for an access token.
    pub async fn get_discord_token_by_code(&self, code: &str) -> DiscordToken {
        Client::new()
            .post(format!("{}/oauth2/token", &self.base_api_url()))
            .form(&DiscordAuthorizationCodeRequest {
                grant_type: "authorization_code",
                code,
                redirect_uri: &self.redirect_url,
            })
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .expect("Failed to send token request")
            .json::<DiscordToken>()
            .await
            .expect("Failed to deserialize token response")
    }

    /// Exchanges the refresh token for an access token.
    pub async fn get_discord_token_by_refresh_token(&self, refresh_token: &str) -> DiscordToken {
        Client::new()
            .post(format!("{}/oauth2/token", &self.base_api_url()))
            .form(&DiscordRefreshTokenRequest {
                grant_type: "refresh_token",
                refresh_token,
                redirect_uri: &self.redirect_url,
            })
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .expect("Failed to send token request")
            .json::<DiscordToken>()
            .await
            .expect("Failed to deserialize token response")
    }

    /// Calls Discord’s `/users/@me` endpoint with the given access token to fetch user info.
    pub async fn get_discord_self_user(&self, access_token: &str) -> Result<DiscordUser, Box<dyn Error>> {
        let client = Client::new();
        let user = client
            .get(format!("{}/users/@me", &self.base_api_url()))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<DiscordUser>()
            .await?;

        Ok(user)
    }

    fn base_api_url(&self) -> String {
        format!("https://discord.com/api/{}", self.api_version)
    }

    fn base_oauth_url(&self) -> String {
        "https://discord.com/oauth2".to_string()
    }
}

#[derive(Debug, Serialize)]
pub struct DiscordAuthorizationCodeRequest<'a> {
    grant_type: &'a str,      // "authorization_code"
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Serialize)]
pub struct DiscordRefreshTokenRequest<'a> {
    grant_type: &'a str,      // "refresh_token"
    refresh_token: &'a str,
    redirect_uri: &'a str,
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

/// Represents the Discord User Object.
/// Reference: https://discord.com/developers/docs/resources/user#user-object
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    /// The user's id (snowflake)
    pub id: String,

    /// The user's username, not unique across the platform
    pub username: String,

    /// The user's 4-digit discord-tag
    pub discriminator: String,

    /// The user's display name (if set). For bots, this is the application name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,

    /// The user's avatar hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// Whether the user is part of an OAuth2 application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,

    /// Whether the user is an Official Discord System user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,

    /// Whether the user has two-factor auth enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,

    /// The user's banner hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The user's banner color as an integer representation of the hex color code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    /// The user's chosen language option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// Whether the email on this account has been verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    /// The user's email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The flags on a user's account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,

    /// The type of Nitro subscription on a user's account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<u32>,

    /// The public flags on a user's account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<u32>,

    /// Data for the user's avatar decoration (structure not fully documented)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Value>,
}