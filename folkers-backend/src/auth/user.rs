//! User specifications module

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Watcher,
    Editor,
    Admin
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl UserRole {
    pub fn from_str(role: impl AsRef<str>) -> Self {
        match role.as_ref().to_lowercase().as_str() {
            "watcher" => Self::Watcher,
            "editor" => Self::Editor,
            "admin" => Self::Admin,
            _ => Self::Watcher
        }
    }

    pub fn get_id(&self) -> usize {
        match self {
            UserRole::Watcher => 0,
            UserRole::Editor => 1,
            UserRole::Admin => 2,
        }
    }
}

impl PartialOrd for UserRole {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_id().partial_cmp(&other.get_id())
    }
}
