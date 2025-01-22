use crate::config::DiscordConfig;

#[derive(Clone)]
pub struct DiscordApiService {
    pub elite_guild_id: String,
    pub api_version: String,
    pub bot_token: String,
}

impl DiscordApiService {
    pub fn new(config: &DiscordConfig) -> Self {
        Self {
            elite_guild_id: config.elite_guild_id.to_string(),
            api_version: config.api_version.to_string(),
            bot_token: config.bot_token.to_string(),
        }
    }

    // pub fn get_elite_guild(&self) -> Guild {
    //     let url = self.api_url_for(&format!("guilds/{}", self.elite_guild_id));
    //
    //
    // }

    fn api_url_for(&self, path: &str) -> String {
        format!("https://discord.com/api/v{}/{}", self.api_version, path)
    }
}
