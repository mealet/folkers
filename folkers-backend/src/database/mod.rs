//! Database interaction module.
//! You can adapt it and use with axum's `with_state`, but I'd recommend to make
//! it static with LazyLock.

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal
};

pub mod user;

const USER: &str = "user";

pub struct DatabaseClient {
    connection: Surreal<Client>
}

impl DatabaseClient {
    /// Initialization for LazyLock (singleton)
    pub fn init() -> Self {
        Self {
            connection: Surreal::init()
        }
    }

    /// Setup database connection and define necessary fields
    pub async fn setup(&self, endpoint: &str, namespace: &str, database: &str, username: &str, password: &str) -> Result<(), surrealdb::Error> {
        self.connection.connect::<Ws>(endpoint).await?;

        self.connection.signin(Root {
            username,
            password
        }).await?;

        self.connection.use_ns(namespace).use_db(database).await?;

        self.connection.query(format!("
DEFINE TABLE IF NOT EXISTS {USER} SCHEMALESS
    PERMISSIONS FOR
        CREATE, SELECT WHERE $auth,
        FOR UPDATE, DELETE WHERE created_by = $auth;

DEFINE FIELD IF NOT EXISTS username ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS password ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS role ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE {USER} TYPE string;
DEFINE FIELD IF NOT EXISTS creation_datetime ON TABLE {USER} TYPE datetime;

DEFINE INDEX IF NOT EXISTS unique_name ON TABLE {USER} FIELDS username UNIQUE;
")).await?;

        match (std::env::var("FOLKERS_STATIC_ADMIN_USERNAME"), std::env::var("FOLKERS_STATIC_ADMIN_PASSWORD")) {
            (Ok(admin_username), Ok(admin_password)) => {
                let _ = self.create_user(user::CreateUserRecord {
                    username: admin_username,
                    password: crate::auth::UserRepository::hash_password(&admin_password).unwrap(),
                    role: crate::auth::user::UserRole::Admin.to_string(),
                    created_by: String::from("system")
                }).await;
            }
            _ => {
                log::warn!("Static admin wasn't initialized (environment variables fetch error)");
            }
        };

        Ok(())
    }

    /// Create new user
    pub async fn create_user(
        &self,
        user: user::CreateUserRecord
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let created_user = self.connection
            .create(USER)
            .content(
                user::UserRecord {
                    id: None,
                    username: user.username,
                    password: user.password,
                    role: user.role,
                    created_by: user.created_by,
                    creation_datetime: surrealdb::Datetime::from(chrono::Utc::now())
                }
            ).await;

        created_user
    }

    /// Get user by SurrealDB Identifier
    pub async fn get_user(
        &self,
        id: impl AsRef<str>
    ) -> Option<user::UserRecord> {
        let user_record: Option<user::UserRecord> = self.connection
            .select((USER, id.as_ref()))
            .await
            .ok()?;

        user_record
    }

    /// Get user by his username
    pub async fn get_user_by_username(
        &self,
        username: String
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let mut query = self.connection
            .query(format!("SELECT * FROM {USER} WHERE username = $username"))
            .bind(("username", username))
            .await?;

        let result: Option<user::UserRecord> = query.take(0usize)?;
        return Ok(result)
    }

    /// Update user data by SurrealDB ID
    pub async fn update_user(
        &self,
        id: impl AsRef<str>,
        user: user::CreateUserRecord
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let updated_user = self.connection
            .update((USER, id.as_ref()))
            .content(user)
            .await;

        updated_user
    }

    /// Delete user by SurrealDB ID
    pub async fn delete_user(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let id = id.as_ref();
        let deleted_user = self.connection
            .delete((USER, id))
            .await?;

        Ok(deleted_user)
    }

    /// List all users
    pub async fn list_users(
        &self
    ) -> Result<Vec<user::UserRecord>, surrealdb::Error> {
        let users = self.connection.select(USER).await;

        users
    }
}
