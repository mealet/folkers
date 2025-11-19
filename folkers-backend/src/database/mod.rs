//! Database interaction module.

use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::{database::signature::RecordSignatureRecord, signatures::RecordSignature};

pub mod signature;
pub mod person;
pub mod user;

const USER: &str = "user";
const PERSON: &str = "person";
const SIGNATURES: &str = "signatures";

/// Database interaction client.
/// You can adapt it and use with axum's `with_state`, but I'd recommend to make
/// it static with LazyLock.
pub struct DatabaseClient {
    connection: Surreal<Client>,
}

impl DatabaseClient {
    /// Initialization for LazyLock (singleton)
    pub fn init() -> Self {
        Self {
            connection: Surreal::init(),
        }
    }

    /// Setup database connection and define necessary fields
    pub async fn setup(
        &self,
        endpoint: &str,
        namespace: &str,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<(), surrealdb::Error> {
        self.connection.connect::<Ws>(endpoint).await?;

        self.connection.signin(Root { username, password }).await?;

        self.connection.use_ns(namespace).use_db(database).await?;

        self.connection.query(format!("
-- Users Table
DEFINE TABLE IF NOT EXISTS {USER} SCHEMAFULL
    PERMISSIONS FOR
        CREATE, SELECT WHERE $auth,
        FOR UPDATE, DELETE WHERE created_by = $auth;

DEFINE FIELD IF NOT EXISTS username ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS password ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS role ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS creation_datetime ON TABLE {USER} TYPE datetime;

DEFINE INDEX IF NOT EXISTS unique_name ON TABLE {USER} COLUMNS username UNIQUE;

-- Persons Records Table

DEFINE TABLE IF NOT EXISTS {PERSON} SCHEMAFULL
    PERMISSIONS FOR
        CREATE, SELECT WHERE $auth,
        FOR UPDATE, DELETE WHERE created_by = $auth;

DEFINE FIELD IF NOT EXISTS name ON TABLE {PERSON} TYPE string;
DEFINE FIELD IF NOT EXISTS surname ON TABLE {PERSON} TYPE string;
DEFINE FIELD IF NOT EXISTS patronymic ON TABLE {PERSON} TYPE string;

DEFINE FIELD IF NOT EXISTS birthday ON TABLE {PERSON} TYPE datetime;
DEFINE FIELD IF NOT EXISTS city ON TABLE {PERSON} TYPE string;
DEFINE FIELD IF NOT EXISTS intented_address ON TABLE {PERSON} TYPE string;

DEFINE FIELD IF NOT EXISTS summary ON TABLE {PERSON} TYPE string;
DEFINE FIELD IF NOT EXISTS past ON TABLE {PERSON} TYPE string;
DEFINE FIELD IF NOT EXISTS traits_good ON TABLE {PERSON} TYPE string;
DEFINE FIELD IF NOT EXISTS traits_bad ON TABLE {PERSON} TYPE string;

DEFINE FIELD IF NOT EXISTS avatar ON TABLE {PERSON} TYPE string;
DEFINE FIELD IF NOT EXISTS media ON TABLE {PERSON} TYPE array<string>;

DEFINE FIELD IF NOT EXISTS author ON TABLE {PERSON} TYPE string;

DEFINE INDEX IF NOT EXISTS unique_person ON TABLE {PERSON} COLUMNS surname, name, patronymic UNIQUE;

-- Person Records Signatures Table

DEFINE TABLE IF NOT EXISTS {SIGNATURES} SCHEMAFULL
    PERMISSIONS FOR
        CREATE, SELECT WHERE $auth
        FOR UPDATE, DELETE WHERE created_by = $auth;

DEFINE FIELD record_id ON TABLE {SIGNATURES} TYPE string;
DEFINE FIELD base64 ON TABLE {SIGNATURES} TYPE string;
DEFINE FIELD pubkey ON TABLE {SIGNATURES} TYPE string;

-- Functions

DEFINE FUNCTION IF NOT EXISTS fn::find_person($query: string) {{
    LET $q = string::trim(string::lowercase($query));
    LET $words = $q.split(' ');
    
    RETURN SELECT * FROM person 
    WHERE 
        string::lowercase(surname) CONTAINS $q
        OR string::lowercase(name) CONTAINS $q
        OR string::lowercase(patronymic) CONTAINS $q
        OR string::lowercase(surname + ' ' + name + ' ' + patronymic) CONTAINS $q
        OR string::lowercase(name + ' ' + patronymic + ' ' + surname) CONTAINS $q
        OR string::lowercase(patronymic + ' ' + name + ' ' + surname) CONTAINS $q
        OR string::lowercase(surname + ' ' + patronymic + ' ' + name) CONTAINS $q
        OR array::len($words) >= 2 AND (
            (string::lowercase(surname) CONTAINS array::at($words, 0) AND string::lowercase(name) CONTAINS array::at($words, 1))
            OR (string::lowercase(surname) CONTAINS array::at($words, 1) AND string::lowercase(name) CONTAINS array::at($words, 0))
            OR (string::lowercase(name) CONTAINS array::at($words, 0) AND string::lowercase(patronymic) CONTAINS array::at($words, 1))
            OR (string::lowercase(name) CONTAINS array::at($words, 1) AND string::lowercase(patronymic) CONTAINS array::at($words, 0))
        )
        OR array::len($words) = 3 AND (
            (string::lowercase(surname) CONTAINS array::at($words, 0) AND string::lowercase(name) CONTAINS array::at($words, 1) AND string::lowercase(patronymic) CONTAINS array::at($words, 2))
            OR (string::lowercase(surname) CONTAINS array::at($words, 0) AND string::lowercase(name) CONTAINS array::at($words, 2) AND string::lowercase(patronymic) CONTAINS array::at($words, 1))
            OR (string::lowercase(surname) CONTAINS array::at($words, 1) AND string::lowercase(name) CONTAINS array::at($words, 0) AND string::lowercase(patronymic) CONTAINS array::at($words, 2))
            OR (string::lowercase(surname) CONTAINS array::at($words, 1) AND string::lowercase(name) CONTAINS array::at($words, 2) AND string::lowercase(patronymic) CONTAINS array::at($words, 0))
            OR (string::lowercase(surname) CONTAINS array::at($words, 2) AND string::lowercase(name) CONTAINS array::at($words, 0) AND string::lowercase(patronymic) CONTAINS array::at($words, 1))
            OR (string::lowercase(surname) CONTAINS array::at($words, 2) AND string::lowercase(name) CONTAINS array::at($words, 1) AND string::lowercase(patronymic) CONTAINS array::at($words, 0))
        )
    ORDER BY
        surname, name, patronymic
    LIMIT 25;
}};
")).await?;

        match (
            std::env::var("FOLKERS_STATIC_ADMIN_USERNAME"),
            std::env::var("FOLKERS_STATIC_ADMIN_PASSWORD"),
        ) {
            (Ok(admin_username), Ok(admin_password)) => {
                let _ = self
                    .create_user(user::CreateUserRecord {
                        username: admin_username,
                        password: crate::auth::UserRepository::hash_password(&admin_password)
                            .unwrap(),
                        role: crate::auth::user::UserRole::Admin.to_string(),
                        created_by: String::from("system"),
                    })
                    .await;
            }
            _ => {
                log::warn!("Static admin wasn't initialized (environment variables fetch error)");
            }
        };

        Ok(())
    }

    // INFO: Users Records Section

    /// Create new user
    pub async fn create_user(
        &self,
        user: user::CreateUserRecord,
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        self.connection
            .create(USER)
            .content(user::UserRecord {
                id: None,
                username: user.username,
                password: user.password,
                role: user.role,
                created_by: user.created_by,
                creation_datetime: surrealdb::Datetime::from(chrono::Utc::now()),
            })
            .await
    }

    /// Get user by SurrealDB Identifier
    pub async fn get_user(&self, id: impl AsRef<str>) -> Option<user::UserRecord> {
        let user_record: Option<user::UserRecord> =
            self.connection.select((USER, id.as_ref())).await.ok()?;

        user_record
    }

    /// Get user by his username
    pub async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let mut query = self
            .connection
            .query(format!("SELECT * FROM {USER} WHERE username = $username"))
            .bind(("username", username))
            .await?;

        let result: Option<user::UserRecord> = query.take(0usize)?;
        Ok(result)
    }

    /// Update user data by SurrealDB ID
    pub async fn update_user(
        &self,
        id: impl AsRef<str>,
        user: user::CreateUserRecord,
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        self.connection
            .update((USER, id.as_ref()))
            .merge(user)
            // .content(user)
            .await
    }

    /// Delete user by SurrealDB ID
    pub async fn delete_user(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let id = id.as_ref();
        let deleted_user = self.connection.delete((USER, id)).await?;

        Ok(deleted_user)
    }

    /// List all users
    pub async fn list_users(&self) -> Result<Vec<user::UserRecord>, surrealdb::Error> {
        self.connection.select(USER).await
    }

    // INFO: Persons Records Section

    // Add new Person record
    pub async fn add_person(
        &self,
        person: person::CreatePersonRecord,
        author: impl AsRef<str>,
    ) -> Result<Option<person::PersonRecord>, surrealdb::Error> {
        self.connection
            .create(PERSON)
            .content(person::PersonRecord {
                id: None,

                name: person.name,
                surname: person.surname,
                patronymic: person.patronymic,

                birthday: person.birthday,
                city: person.city,
                intented_address: person.intented_address,

                summary: person.summary,
                past: person.past,
                traits_good: person.traits_good,
                traits_bad: person.traits_bad,

                avatar: person.avatar,
                media: person.media,

                author: author.as_ref().to_owned(),
            })
            .await
    }

    /// Get Person record by SurrealDB Identifier
    pub async fn get_person(&self, id: impl AsRef<str>) -> Option<person::PersonRecord> {
        let user_record: Option<person::PersonRecord> =
            self.connection.select((PERSON, id.as_ref())).await.ok()?;

        user_record
    }

    /// Find person by search query
    pub async fn find_person(
        &self,
        search_query: impl AsRef<str>,
    ) -> Result<Vec<person::PersonRecord>, surrealdb::Error> {
        let search_query = search_query.as_ref().to_string();
        let mut query = self
            .connection
            .query("SELECT * FROM fn::find_person($query)")
            .bind(("query", search_query))
            .await?;

        query.take(0usize)
    }

    /// Update person by SurrealDB Identifier
    pub async fn update_person(
        &self,
        id: impl AsRef<str>,
        person: person::CreatePersonRecord,
    ) -> Result<Option<person::PersonRecord>, surrealdb::Error> {
        self.connection
            .update((PERSON, id.as_ref()))
            .merge(person)
            .await
    }

    /// Delete person by SurrealDB Identifier
    pub async fn delete_person(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<person::PersonRecord>, surrealdb::Error> {
        let id = id.as_ref();
        let deleted_record = self.connection.delete((PERSON, id)).await?;

        Ok(deleted_record)
    }

    /// Get Persons Records list
    pub async fn list_persons(&self) -> Result<Vec<person::PersonRecord>, surrealdb::Error> {
        self.connection.select(PERSON).await
    }

    // INFO: Signatures Section

    pub async fn add_signature(&self, signature: RecordSignature) -> Result<Option<RecordSignatureRecord>, surrealdb::Error> {
        self.connection
            .create(SIGNATURES)
            .content(RecordSignatureRecord {
                id: None,
                record_id: signature.record_id,
                base64: signature.base64,
                pubkey: signature.pubkey
            })
            .await
    }

    pub async fn get_signature(&self, record_id: String) -> Result<Option<RecordSignatureRecord>, surrealdb::Error> {
        let mut query = self.connection
            .query(format!("SELECT * FROM {SIGNATURES} WHERE record_id = $record_id"))
            .bind(("record_id", record_id))
            .await?;

        let result: Option<RecordSignatureRecord> = query.take(0usize)?;
        Ok(result)
    }
}
