//! JWT - Json Web Tokens, used for users auth and verification (to avoid external access).

use super::user;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// Claims for JWT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// User ID
    pub sub: String,
    /// User Name
    pub username: String,
    /// User Role
    pub role: String,
    /// Token Expiration Timestamp
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: chrono::Duration,
}

impl JwtConfig {
    pub fn new() -> Result<Self, anyhow::Error> {
        match std::env::var("FOLKERS_JWT_SECRET") {
            Ok(secret_var) => Ok(Self {
                secret: secret_var,
                expiration: chrono::Duration::hours(2),
            }),
            Err(error) => Err(anyhow::Error::msg(format!(
                "Unable to fetch FOLKERS_JWT_SECRET: {error}"
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(&self, user: &user::User) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(self.config.expiration)
            .expect("unable to calculate expiration")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            role: user.role.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_ref()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
    }
}
