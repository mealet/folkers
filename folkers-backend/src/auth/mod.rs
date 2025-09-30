use serde::{Serialize, Deserialize};

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

    fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    pub async fn find_by_username(&self, username: &str) -> Option<user::User> {
        if username == "mealet" {
            return Some(user::User {
                id: 0,
                username: "mealet".to_string(),
                password_hash: Self::hash_password("123").unwrap_or(String::new()),
                role: user::UserRole::Admin
            });
        }

        None
    }

    pub async fn verify_password(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.find_by_username(username).await {
            return bcrypt::verify(password, &user.password_hash).unwrap_or(false);
        }
        
        false
    }
}
