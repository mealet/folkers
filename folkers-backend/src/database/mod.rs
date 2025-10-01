//! Database interaction module.
//! You can adapt it and use with axum's `with_state`, but I'd recommend to make
//! it static with LazyLock.

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal
};

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

        self.connection.query("
DEFINE TABLE IF NOT EXISTS user SCHEMALESS
    PERMISSIONS FOR
        CREATE, SELECT WHERE $auth,
        FOR UPDATE, DELETE WHERE created_by = $auth;

DEFINE FIELD IF NOT EXISTS username ON TABLE user TYPE string;
DEFINE FIELD IF NOT EXISTS password ON TABLE user TYPE string;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE user TYPE record<user>;
DEFINE FIELD IF NOT EXISTS creation_datetime ON TABLE user TYPE datetime;

DEFINE INDEX IF NOT EXISTS unique_name ON TABLE user FIELDS username UNIQUE;
").await?;

        Ok(())
    }
}
