use crate::app::error::AppError;
use crate::config::DiscordConfig;
use crate::model::discord::User;
use crate::model::session::UserRole;
use crate::service::discord::error::Error;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::url::Url;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, RedirectUrl, Scope,
    StandardRevocableToken, TokenUrl,
};
use reqwest::Client;

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
    discord_config: DiscordConfig,
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
            discord_config: discord_config.clone(),
        }
    }

    /// Returns an oauth url for logging in with discord as well as a csrf token for safe oauth flow
    pub fn init_auth(&self) -> (Url, CsrfToken) {
        self.oauth_client.authorize_url(CsrfToken::new_random).add_scope(Scope::new(self.discord_config.scopes.clone())).url()
    }

    /// Exchanges oauth code for discord tokens
    pub async fn exchanged_code_for_tokens(&self, code: &str) -> Result<BasicTokenResponse, AppError> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&self.http_client)
            .await
            .map_err(|e| Error::DiscordApiRequestError(e.to_string()))?;

        Ok(token)
    }

    /// Calls Discord’s `/users/@me` endpoint with the given access token to fetch user info.
    pub async fn get_discord_self_user_id(&self, access_token: &AccessToken) -> Result<String, Error> {
        let user = self
            .http_client
            .get(&self.api_url_for("users/@me"))
            .bearer_auth(access_token.secret())
            .send()
            .await
            .map_err(|e| Error::DiscordApiRequestError(e.to_string()))?
            .json::<User>()
            .await
            .map_err(|e| Error::DiscordApiRequestError(e.to_string()))?;

        Ok(user.id)
    }

    pub fn get_role_for_member(&self, roles: &Vec<String>) -> Result<UserRole, Error> {
        if roles.contains(&self.discord_config.elite_staff_role_id) {
            Ok(UserRole::Staff)
        } else {
            Ok(UserRole::Member)
        }
    }

    fn api_url_for(&self, path: &str) -> String {
        format!("https://discord.com/api/v{}/{}", self.discord_config.api_version, path)
    }
}
