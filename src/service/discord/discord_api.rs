use crate::config::DiscordConfig;
use crate::model::discord::{Guild, Member};
use crate::service::discord::error::Error;
use crate::service::discord::error::Error::DiscordApiRequestError;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use tracing::debug;

#[derive(Clone)]
pub struct DiscordApiService {
    client: Client,
    elite_guild_id: String,
    api_version: String,
    bot_token: String,
}

impl DiscordApiService {
    pub fn new(config: &DiscordConfig) -> Self {
        Self {
            client: Client::new(),
            elite_guild_id: config.elite_guild_id.to_string(),
            api_version: config.api_version.to_string(),
            bot_token: config.bot_token.to_string(),
        }
    }

    pub async fn get_elite_guild(&self) -> Result<Guild, Error> {
        let url = self.api_url_for(&format!("guilds/{}", self.elite_guild_id));
        let guild = self.request::<Guild>(self.client.get(url)).await?;

        Ok(guild)
    }

    pub async fn get_elite_guild_member(&self, user_id: &str) -> Result<Member, Error> {
        let url = self.api_url_for(&format!("guilds/{}/members/{}", self.elite_guild_id, user_id));
        let member = self.request::<Member>(self.client.get(url)).await?;

        Ok(member)
    }

    async fn request<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T, Error> {
        let response = request
            .header("Authorization", format!("Bot {}", self.bot_token))
            .send()
            .await
            .map_err(|e| DiscordApiRequestError(e.to_string()))?;

        // request.try_clone().map(|req| req.me)
        // response.status().
        // response.

        if !response.status().is_success() {
            debug!("Failed to get response: {:?}", response);
            return Err(DiscordApiRequestError(response.status().to_string()));
        }

        response.json::<T>().await.map_err(|e| {
            debug!("Failed to deserialize response: {:?}", e);
            DiscordApiRequestError(e.to_string())
        })
    }

    fn api_url_for(&self, path: &str) -> String {
        format!("https://discord.com/api/v{}/{}", self.api_version, path)
    }
}
