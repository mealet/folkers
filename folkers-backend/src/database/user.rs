use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, sql::Thing};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRecord {
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_by: String,
    pub creation_datetime: Datetime,
    pub public_key: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRecord {
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_by: String,
}
