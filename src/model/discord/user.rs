use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the Discord User Object.
/// Reference: https://discord.com/developers/docs/resources/user#user-object
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
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
