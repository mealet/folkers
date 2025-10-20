//! Authentication manager module.

use crate::DATABASE;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use serde::{Deserialize, Serialize};

pub mod jwt;
pub mod user;

/// Login Request Payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login Server Response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}

/// Wrapper for encapsulating database access to authenticate user
#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let env_var = std::env::var("FOLKERS_BASE64_SALT").unwrap(); // already
        // checked in main function

        let salt = SaltString::from_b64(&env_var).unwrap();
        Argon2::default()
            .hash_password(password.as_ref(), &salt)
            .map(|hash| hash.to_string())
    }

    pub async fn find_by_username(&self, username: &str) -> Option<user::User> {
        if let Ok(opt) = DATABASE.get_user_by_username(username.to_string()).await {
            return opt.map(|record| user::User {
                id: record
                    .id
                    .map(|id| id.id.to_string())
                    .unwrap_or("none".to_string()),
                username: record.username,
                password_hash: record.password,
                role: user::UserRole::from_str(record.role),
            });
        }

        None
    }

    pub async fn verify_password(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.find_by_username(username).await {
            let password_hash =
                PasswordHash::parse(&user.password_hash, argon2::password_hash::Encoding::B64);
            if password_hash.is_err() {
                return false;
            };

            return Argon2::default()
                .verify_password(password.as_ref(), &password_hash.unwrap())
                .is_ok();
        }

        false
    }
}
