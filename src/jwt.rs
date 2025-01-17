use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents the claims in the JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // Subject (e.g., user ID)
    pub username: String,
    pub iat: usize,     // Issued-at timestamp
    pub exp: usize,     // Expiration timestamp
}

impl Claims {
    /// Creates a new `Claims` object with the given subject and username.
    /// `iat` and `exp` will be set during token generation.
    pub fn new(sub: String, username: String) -> Self {
        Self {
            sub,
            username,
            iat: 0, // Placeholder; will be set during token generation
            exp: 0, // Placeholder; will be set during token generation
        }
    }
}

/// Handles JWT operations (generation and validation) for access and refresh tokens.
pub struct JwtHandler {
    access_key: String,       // Secret for signing/verifying access tokens
    refresh_key: String,      // Secret for signing/verifying refresh tokens
    access_token_exp: usize,  // Expiration time in seconds for access tokens
    refresh_token_exp: usize, // Expiration time in seconds for refresh tokens
}

impl JwtHandler {
    /// Creates a new `JwtHandler` with secrets for access and refresh tokens.
    pub fn new(access_secret: &str, refresh_secret: &str, access_token_exp: usize, refresh_token_exp: usize) -> Self {
        Self {
            access_key: access_secret.to_string(),
            refresh_key: refresh_secret.to_string(),
            access_token_exp,
            refresh_token_exp,
        }
    }

    /// Generates an access token with the specified claims.
    pub fn generate_access_token(&self, claims: &mut Claims) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(claims, &self.access_key, self.access_token_exp)
    }

    /// Generates a refresh token with the specified claims.
    pub fn generate_refresh_token(&self, claims: &mut Claims) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(claims, &self.refresh_key, self.refresh_token_exp)
    }

    /// Validates and decodes an access token, returning the claims if valid.
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        self.validate_token(token, &self.access_key)
    }

    /// Validates and decodes a refresh token, returning the claims if valid.
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        self.validate_token(token, &self.refresh_key)
    }

    /// Internal method to generate a token with a specific expiration time and secret.
    fn generate_token(
        &self,
        claims: &mut Claims,
        secret: &str,
        expiration: usize,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        claims.iat = now;             // Set issued-at time
        claims.exp = now + expiration; // Set expiration time

        let header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        encode(&header, claims, &encoding_key)
    }

    /// Internal method to validate and decode a token with a specific secret.
    fn validate_token(
        &self,
        token: &str,
        secret: &str,
    ) -> Result<Claims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation {
            algorithms: vec![Algorithm::HS512],
            ..Default::default()
        };

        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
