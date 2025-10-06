use serde::{Serialize, Deserialize};
use surrealdb::{sql::Thing, Datetime};

/// Record about Person's full information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRecord {
    /// SurrealDB Identifier
    pub id: Option<Thing>,

    pub name: String,
    pub surname: String,
    pub patronymic: String,

    pub birthday: Datetime,
    pub city: String,
    pub intented_address: String,

    /// Summary description about person
    pub summary: String,
    /// Shortly described person's past
    pub past: String,
    pub traits_good: String,
    pub traits_bad: String,

    /// Optional string URL to image.
    /// String may start with `@/` which stands for self link.
    pub avatar: Option<String>,

    /// Optional strings of URLS to images list.
    /// String may start with `@/` which stands for self link.
    pub media: Vec<String>,

    /// SurrealDB reference to `user` record.
    pub author: Thing
}

/// JSON Payload to create new Person's record
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

    /// String URL to the image.
    /// String may start with `@/` which stands for self link.
    pub avatar: Option<String>,
    /// List of string URLs to images.
    /// String may start with `@/` which stands for self link.
    pub media: Vec<String>
}

/// Search JSON payload. <br/>
/// Possible options for query:
/// `name surname patronymic`, `name patronymic`, `name surname`, `surname patronymic` and etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPersonRecord {
    /// Optional string that can contains `name`, `surname`, `patronymic`
    pub search_query: String
}
