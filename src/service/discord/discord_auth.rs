use crate::app::error::AppError;
use crate::config::{DiscordConfig, RedisConfig};
use crate::model::discord::{DiscordAuthorizationCodeRequest, DiscordRefreshTokenRequest, DiscordToken, User};
use crate::web::error::Error;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, RedirectUrl, Scope, StandardRevocableToken, TokenUrl,
};
use redis::{Commands, ExpireOption, ToRedisArgs};
use redis_pool::{RedisPool, SingleRedisPool};
use reqwest::Client;
use tracing::debug;
use urlencoding::encode;

pub type DiscordAuthClient = BasicClient;

#[derive(Clone)]
pub struct DiscordAuthService {
    http_client: Client,
    // Not really happy with this but what do I know
    pub oauth_client: oauth2::Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    pub scopes: String,
}

impl DiscordAuthService {
    pub fn new(discord_config: &DiscordConfig) -> Self {
        let discord_oauth_url = "https://discord.com/oauth2/authorize".to_string();
        let discord_token_url = format!("https://discord.com/api/v{}/oauth2/token", &discord_config.api_version);

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let oauth_client = DiscordAuthClient::new(ClientId::new(discord_config.client_id.to_string()))
            .set_client_secret(ClientSecret::new(discord_config.client_secret.to_string()))
            .set_auth_uri(AuthUrl::new(discord_oauth_url).expect("Failed to parse Discord OAuth URL"))
            .set_token_uri(TokenUrl::new(discord_token_url).expect("Failed to parse Discord token URL"))
            .set_redirect_uri(RedirectUrl::new(discord_config.redirect_url.to_string()).expect("Failed to parse Discord redirect URL"));

        Self {
            http_client,
            oauth_client,
            scopes: discord_config.scopes.to_string(),
        }
    }

    pub fn init_auth(&self) -> (Url, CsrfToken) {
        self.oauth_client.authorize_url(CsrfToken::new_random).add_scope(Scope::new(self.scopes.clone())).url()
    }

    pub async fn exchanged_code_for_tokens(&self, code: &str) -> Result<BasicTokenResponse, AppError> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&self.http_client)
            .await
            .map_err(|e| Error::DiscordTokenError(e.to_string()))?;

        Ok(token)
    }

    // pub async fn authenticate(&self, code: &str) -> Result<String, Error> {
    //     let (auth_url, csrf_token) = client.authorize_url(CsrfToken::new_random).add_scope(Scope::new("identify".to_string())).url();
    //
    //     let token = self.get_discord_token_by_code(code).await;
    //     let user = self.get_discord_self_user(&token.access_token).await?;
    //
    //     self.store_token(&user.id, &token).await?;
    //
    //     Ok(user.id)
    // }
    //
    // /// Exchanges the authorization code for an access token.
    // pub async fn get_discord_token_by_code(&self, code: &str) -> DiscordToken {
    //     self.client
    //         .post(&self.api_url_for("oauth2/token"))
    //         .form(&DiscordAuthorizationCodeRequest {
    //             grant_type: "authorization_code".to_string(),
    //             code: code.to_string(),
    //             redirect_uri: self.redirect_url.to_string(),
    //         })
    //         .basic_auth(&self.client_id, Some(&self.client_secret))
    //         .send()
    //         .await
    //         .expect("Failed to send token request")
    //         .json::<DiscordToken>()
    //         .await
    //         .expect("Failed to deserialize token response")
    // }
    //
    // /// Exchanges the refresh token for an access token.
    // pub async fn get_discord_token_by_refresh_token(&self, refresh_token: &str) -> DiscordToken {
    //     self.client
    //         .post(&self.api_url_for("oauth2/token"))
    //         .form(&DiscordRefreshTokenRequest {
    //             grant_type: "refresh_token".to_string(),
    //             refresh_token: refresh_token.to_string(),
    //             redirect_uri: self.redirect_url.to_string(),
    //         })
    //         .basic_auth(&self.client_id, Some(&self.client_secret))
    //         .send()
    //         .await
    //         .expect("Failed to send token request")
    //         .json::<DiscordToken>()
    //         .await
    //         .expect("Failed to deserialize token response")
    // }
    //
    // /// Calls Discord’s `/users/@me` endpoint with the given access token to fetch user info.
    // pub async fn get_discord_self_user(&self, access_token: &str) -> Result<User, Error> {
    //     let user = self
    //         .client
    //         .get(&self.api_url_for("users/@me"))
    //         .bearer_auth(access_token)
    //         .send()
    //         .await
    //         .map_err(|e| Error::DiscordApiError(e.to_string()))?
    //         .json::<User>()
    //         .await
    //         .map_err(|e| Error::DiscordApiError(e.to_string()))?;
    //
    //     Ok(user)
    // }
    //
    // fn api_url_for(&self, path: &str) -> String {
    //     format!("https://discord.com/api/v{}/{}", self.api_version, path)
    // }
    //
    // fn oauth_url_for(&self, path: &str) -> String {
    //     format!("https://discord.com/oauth2/{}", path)
    // }
    //
    // async fn store_token(&self, user_id: &str, token: &DiscordToken) -> Result<(), Error> {
    //     let mut conn = self.redis.get_connection().map_err(|_e| Error::RedisConnectionError)?;
    //     let user_key = format!("user:{}", user_id);
    //     let redis_operations = [("access_token", &token.access_token), ("refresh_token", &token.refresh_token)];
    //
    //     debug!("Storing token information in Redis");
    //     let _: () = conn.hset_multiple(&user_key, &redis_operations).map_err(|e| {
    //         debug!("Failed to store tokens: {:?}", e);
    //         Error::RedisOperationError(e.to_string())
    //     })?;
    //
    //     debug!("Setting token expiration");
    //     let _: () = conn.hexpire(&user_key, token.expires_in - 5, ExpireOption::NONE, "access_token").map_err(|e| {
    //         debug!("Failed to set expiration: {:?}", e);
    //         Error::RedisOperationError(e.to_string())
    //     })?;
    //
    //     Ok(())
    // }
}
