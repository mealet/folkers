//! Database interaction module.
//! You can adapt it and use with axum's `with_state`, but I'd recommend to make
//! it static with LazyLock.

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal
};

mod user;

const USER: &str = "user";

pub struct DatabaseClient {
    connection: Surreal<Client>
}

impl DatabaseClient {
    pub fn init() -> Self {
        Self {
            connection: Surreal::init()
        }
    }

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

        Ok(())
    }

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
                    creation_datetime: chrono::Utc::now()
                }
            ).await;

        created_user
    }

    pub async fn get_user(
        &self,
        id: &str
    ) -> Option<user::UserRecord> {
        let user_record: Option<user::UserRecord> = self.connection
            .select((USER, id))
            .await
            .ok()?;

        user_record
    }

    pub async fn update_user(
        &self,
        id: &str,
        user: user::CreateUserRecord
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let updated_user = self.connection
            .update((USER, id))
            .content(user)
            .await;

        updated_user
    }

    pub async fn delete_user(
        &self,
        id: &str
    ) -> Result<Option<user::UserRecord>, surrealdb::Error> {
        let deleted_user = self.connection
            .delete((USER, id))
            .await;

        deleted_user
    }

    pub async fn list_users(
        &self
    ) -> Result<Vec<user::UserRecord>, surrealdb::Error> {
        let users = self.connection.select(USER).await;

        users
    }
}
