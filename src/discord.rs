use urlencoding::encode;
use crate::config::DiscordConfig;

pub struct Discord;

impl Discord {
    pub fn get_oauth_url(config: &DiscordConfig) -> String {
        format!("https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}", config.client_id, encode(&config.redirect_url), config.scopes)
    }
}