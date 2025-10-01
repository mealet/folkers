use serde::{Serialize, Deserialize};
use crate::DATABASE;

pub mod user;
pub mod jwt;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}

/// Wrapper for encapsulating access to database
#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    pub async fn find_by_username(&self, username: &str) -> Option<user::User> {
        if let Ok(opt) = DATABASE.get_user_by_username(username.to_string()).await {
            return opt.map(|record| user::User {
                id: record.id.map(|id| id.to_string()).unwrap_or("none".to_string()),
                username: record.username,
                password_hash: record.password,
                role: user::UserRole::from_str(record.role)
            })
        }

        None
    }

    pub async fn verify_password(&self, username: &str, password_hash: &str) -> bool {
        if let Some(user) = self.find_by_username(username).await {
            return user.password_hash == password_hash;
        }
        
        false
    }
}
