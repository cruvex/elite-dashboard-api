use crate::config::JwtConfig;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone)]
pub struct JwtService {
    access_token_secret: String,
    refresh_token_secret: String,
    access_token_exp: usize,
    refresh_token_exp: usize,
    pub secure_cookie: bool,
}

impl JwtService {
    pub fn new(config: &JwtConfig) -> Self {
        Self {
            access_token_secret: config.access_token_secret.to_string(),
            refresh_token_secret: config.refresh_token_secret.to_string(),
            access_token_exp: config.access_token_exp,
            refresh_token_exp: config.refresh_token_exp,
            secure_cookie: config.secure_cookie,
        }
    }
}

/// Represents the claims in the JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: &str) -> Self {
        Self {
            sub: sub.to_string(),
            iat: 0, // Placeholder
            exp: 0, // Placeholder
        }
    }
}

impl JwtService {
    /// Generates an access token with the specified claims.
    pub fn generate_access_token(&self, claims: &mut Claims) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(claims, &self.access_token_secret, &self.access_token_exp)
    }

    /// Generates a refresh token with the specified claims.
    pub fn generate_refresh_token(&self, claims: &mut Claims) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(claims, &self.refresh_token_secret, &self.refresh_token_exp)
    }

    /// Validates and decodes an access token, returning the claims if valid.
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, crate::web::error::Error> {
        let validation = Validation::new(Algorithm::HS512);

        self.validate_token(token, &self.access_token_secret, validation)
    }

    /// Validates and decodes a refresh token, returning the claims if valid.
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, crate::web::error::Error> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.validate_exp = false;

        self.validate_token(token, &self.refresh_token_secret, validation)
    }

    /// method to generate a token with a specific expiration time and secret.
    fn generate_token(
        &self,
        claims: &mut Claims,
        secret: &str,
        expiration: &usize,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        claims.iat = now; // Set issued-at time
        claims.exp = now + expiration; // Set expiration time

        let header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        encode(&header, claims, &encoding_key)
    }

    /// method to validate and decode a token with a specific secret.
    fn validate_token(&self, token: &str, secret: &str, validation: Validation) -> Result<Claims, crate::web::error::Error> {
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|_e| crate::web::error::Error::JwtTokenValidationError)?;
        Ok(token_data.claims)
    }
}
