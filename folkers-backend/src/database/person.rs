use serde::{Serialize, Deserialize};
use surrealdb::{sql::Thing, Datetime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRecord {
    pub id: Option<Thing>,

    pub name: String,
    pub surname: String,
    pub patronymic: String,

    pub birthday: Datetime,
    pub city: String,
    pub intented_address: String,

    pub summary: String,
    pub past: String,
    pub traits_good: String,
    pub traits_bad: String,

    pub avatar: Option<String>,
    pub media: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonRecord {
    pub name: String,
    pub surname: String,
    pub patronymic: String,
    
    pub birthday: Datetime,
    pub city: String,
    pub intented_address: String,

    pub summary: String,
    pub past: String,
    pub traits_good: String,
    pub traits_bad: String,

    pub avatar: Option<String>,
    pub media: Vec<String>
}
