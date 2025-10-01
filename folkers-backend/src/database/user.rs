use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRecord {
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_by: String,
    pub creation_datetime: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRecord {
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_by: String
}
