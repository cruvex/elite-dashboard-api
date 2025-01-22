use crate::model::discord::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,   // snowflake
    pub name: String, // 2-100 characters, no leading/trailing whitespace
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String, // snowflake
    pub permissions: Option<String>,
    pub region: Option<String>,         // deprecated
    pub afk_channel_id: Option<String>, // snowflake
    pub afk_timeout: i32,               // in seconds
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>, // snowflake
    pub verification_level: i32,
    pub default_message_notifications: i32,
    pub explicit_content_filter: i32,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub mfa_level: i32,
    pub application_id: Option<String>,    // snowflake
    pub system_channel_id: Option<String>, // snowflake
    pub system_channel_flags: i32,
    pub rules_channel_id: Option<String>, // snowflake
    pub max_presences: Option<i32>,
    pub max_members: Option<i32>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: i32,
    pub premium_subscription_count: Option<i32>,
    pub preferred_locale: String,                  // defaults to "en-US"
    pub public_updates_channel_id: Option<String>, // snowflake
    pub max_video_channel_users: Option<i32>,
    pub max_stage_video_channel_users: Option<i32>,
    pub approximate_member_count: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: i32,
    pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: bool,
    pub safety_alerts_channel_id: Option<String>, // snowflake
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emoji {
    // Define emoji fields here
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeScreen {
    // Define welcome screen fields here
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {
    // Define sticker fields here
}
