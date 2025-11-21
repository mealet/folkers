//! # Folkers Backend Server
//! **Folkers Backend** is the main toolchain to safely communicate with database. It provides
//! secure JWT authorization, passwords Argon2 hashing, media uploads manager and user roles
//! verification to avoid unauthorized access.
//!
//! ## Technical Information
//! - **Database:** SurrealDB
//! - **Web Framework:** Axum
//! - **Password Hasher:** Argon2
//! - **Media Hasher:** Sha256
//! - **Authorization:** JSON Web Tokens (Bearer)
//!
//! ## Environment Variables
//! List of required environment variables (duplicated in README.md):
//! ```env
//! FOLKERS_JWT_SECRET=secret string for jwt tokens
//! FOLKERS_BASE64_SALT=base64 encoded salt for hash
//! FOLKERS_UPLOAD_DIR=path to directory with uploaded media
//!
//! FOLKERS_DB_USERNAME=database username
//! FOLKERS_DB_PASSWORD=database password
//! FOLKERS_DB_NAMESPACE=database namespace (surrealdb)
//! FOLKERS_DB_DATABASE=database base name (surrealdb)
//! FOLKERS_DB_ENDPOINT=database endpoint
//!
//! FOLKERS_STATIC_ADMIN_USERNAME=admin that will be created every start
//! FOLKERS_STATIC_ADMIN_PASSWORD=static admin password
//! ```
//!
//! ## API
//! **❗ Each endpoint, which requires authorization will return `401 UNAUTHORIZED` if: <br/>**
//! **- JWT Token is not provided / wrong <br/>**
//! **- JWT Token is expired**
//!
//! ----
//! - ### GET `/` <br/>
//! > **Returns:** HTML markup with message
//! ----
//! - ### GET `/health` <br/>
//! > **Returns:** `200 OK`
//! ----
//! - ### POST `/login` <br/>
//! > **Payload:** [LoginRequest](auth::LoginRequest) <br/>
//! > **Errors:** <br/>
//! > - `401 UNAUTHORIZED` User doesn't exists, Verification failed <br/>
//! > - `500 INTERNAL SERVER ERROR` JWT generation error <br/>
//! >
//! > **Returns:** [AuthResponse](auth::AuthResponse) (JWT Token Structure)
//! ----
//! - ### POST `/upload` <br/>
//! > **Payload:** Image File <br/>
//! > **Authorization:** Required, Role: [Editor](auth::user::UserRole::Editor)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `500 INTERNAL SERVER ERROR` Server IO error <br/>
//! >
//! > **Returns:** [String], image hash (required for getter)
//! ----
//! - ### GET `/media/{hash}` <br/>
//! > **Authorization:** Required, Role: [Watcher](auth::user::UserRole::Watcher)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` Media not found <br/>
//! > - `500 INTERNAL SERVER ERROR` Server IO error <br/>
//! >
//! > **Returns:** Image
//! ----
//! - ### GET `/persons` <br/>
//! > **Authorization:** Required, Role: [Watcher](auth::user::UserRole::Watcher)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** List of [PersonRecord](database::person::PersonRecord)
//! ----
//! - ### POST `/persons/create` <br/>
//! > **Authorization:** Required, Role: [Editor](auth::user::UserRole::Editor)^ <br/>
//! > **Payload:** [CreatePersonRecord](database::person::CreatePersonRecord) <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `409 CONFLICT` Unique record already exists <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** [PersonRecord](database::person::PersonRecord)
//! ----
//! - ### GET `/persons/{id}` <br/>
//! > **Authorization:** Required, Role: [Watcher](auth::user::UserRole::Watcher)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` Record not found <br/>
//! >
//! > **Returns:** [PersonRecord](database::person::PersonRecord)
//! ----
//! - ### PATCH `/persons/{id}` <br/>
//! > **Authorization:** Required, Role: [Editor](auth::user::UserRole::Editor)^ <br/>
//! > **Payload:** [CreatePersonRecord](database::person::CreatePersonRecord) <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions, Not author of record <br/>
//! > - `404 NOT FOUND` Record not found <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** [PersonRecord](database::person::PersonRecord)
//! ----
//! - ### DELETE `/persons/{id}` <br/>
//! > **Authorization:** Required, Role: [Editor](auth::user::UserRole::Editor)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions, Not author of record <br/>
//! > - `404 NOT FOUND` Record not found <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** [PersonRecord](database::person::PersonRecord)
//! ----
//! - ### GET `/persons/{id}/verify` <br/>
//! > **Authorization:** Required, Role: [Watcher](auth::user::UserRole::Watcher)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Record verification error <br/>
//! > - `404 NOT FOUND` Record not found <br/>
//! > - `500 INTERNAL SERVER ERRROR` Database/signature error <br/>
//! >
//! > **Returns:** [RecordSignatureRecord](database::signature::RecordSignatureRecord)
//! ----
//! - ### GET `/users` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` Record not found <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** List of [UserRecord](database::user::UserRecord)
//! ----
//! - ### POST `/users/create` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Payload:** [CreateUserRecord](database::user::CreateUserRecord) <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `500 INTERNAL SERVER ERROR` Hashing error, Database error <br/>
//! >
//! > **Returns:** [UserRecord](database::user::UserRecord)
//! ----
//! - ### GET `/users/{username}` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` User not found <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** [UserRecord](database::user::UserRecord)
//! ----
//! - ### PATCH `/users/{username}` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Payload:** [CreateUserRecord](database::user::CreateUserRecord) <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` User not found <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** [UserRecord](database::user::UserRecord)
//! ----
//! - ### DELETE `/users/{username}` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` User not found <br/>
//! > - `500 INTERNAL SERVER ERROR` Database error <br/>
//! >
//! > **Returns:** [UserRecord](database::user::UserRecord)
//! ----
//! - ### POST `/signature-keygen` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! >
//! > **Returns:** [String] which contains private key
//! ----
//! - ### POST `/persons/{id}/sign` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Payload:** [SignRecordPayload](database::signature::SignRecordPayload) <br/>
//! > **Errors:** <br/>
//! > - `400 BAD REQUEST` Signature error
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` Record not found <br/>
//! > - `409 CONFLICT` Record already signed <br/>
//! > - `500 INTERNAL SERVER ERRROR` Database error <br/>
//! >
//! > **Returns:** [RecordSignatureRecord](database::signature::RecordSignatureRecord)
//! ----
//! - ### DELETE `/persons/{id}/unsign` <br/>
//! > **Authorization:** Required, Role: [Admin](auth::user::UserRole::Admin)^ <br/>
//! > **Errors:** <br/>
//! > - `403 FORBIDDEN` Not enough permissions <br/>
//! > - `404 NOT FOUND` Record not found <br/>
//! > - `500 INTERNAL SERVER ERRROR` Database/signature error <br/>
//! >
//! > **Returns:** [RecordSignatureRecord](database::signature::RecordSignatureRecord)

use axum::{extract::DefaultBodyLimit, http::Method, routing, Router};
use std::sync::LazyLock;
use tower_http::cors::{self, CorsLayer};

mod auth;
mod database;
mod signatures;
mod middleware;
mod routers;
mod uploads;

const ENDPOINT: &str = "0.0.0.0:3001";

pub static DATABASE: LazyLock<database::DatabaseClient> =
    LazyLock::new(database::DatabaseClient::init);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialization
    let _ = dotenvy::dotenv();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let base64_salt = std::env::var("FOLKERS_BASE64_SALT").ok().or_else(|| {
        log::error!("💣 Critical Security Error. Salt is not found in environment variables.");
        log::error!("Please provide your BASE64 encoded salt in environment variable named `FOLKERS_BASE64_SALT`");

        std::process::exit(1);
    }).unwrap();

    let _: Result<argon2::password_hash::SaltString, String> =
        argon2::password_hash::SaltString::from_b64(&base64_salt).map_err(|err| {
            log::error!("💣 Critical Security Error. Encoded salt string failed verification!");
            log::error!("Error: {}", err);

            std::process::exit(1);
        });

    log::info!("🚀 Folkers Backend Server");
    log::info!("⚙️ Starting initialization...");

    // Database setup

    log::info!("- Setting up database...");

    let db_username = std::env::var("FOLKERS_DB_USERNAME")?;
    let db_password = std::env::var("FOLKERS_DB_PASSWORD")?;
    let db_namespace = std::env::var("FOLKERS_DB_NAMESPACE")?;
    let db_database = std::env::var("FOLKERS_DB_DATABASE")?;
    let db_endpoint = std::env::var("FOLKERS_DB_ENDPOINT")?;

    DATABASE
        .setup(
            &db_endpoint,
            &db_namespace,
            &db_database,
            &db_username,
            &db_password,
        )
        .await?;

    // Uploads Setup

    log::info!("- Setting up uploads...");
    uploads::init_uploads().await?;

    // HTTP Cors
    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_headers(cors::Any)
        .allow_methods([Method::POST, Method::GET, Method::DELETE]);

    // App State

    log::info!("- Setting up JWT Service...");

    let jwt_config = auth::jwt::JwtConfig::new()?;
    let jwt_service = auth::jwt::JwtService::new(jwt_config);
    let user_repo = auth::UserRepository::new();

    let app_state = routers::AppState {
        user_repo,
        jwt_service,
    };

    // Main Application Router

    log::info!("- Setting up routers...");

    let public_routers = Router::new()
        .route("/", routing::get(routers::root_handler))
        .route("/health", routing::get(routers::health_handler))
        .route("/login", routing::post(routers::login_handler));

    let watchers_routers = Router::new()
        .route("/me", routing::get(routers::me_handler))
        .route("/media/{hash}", routing::get(routers::media_handler))
        .route("/persons", routing::get(routers::persons_handler))
        .route("/persons/{id}", routing::get(routers::persons_id_handler))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth_middleware,
        ));

    let editors_routers = Router::new()
        .route("/upload", routing::post(routers::upload_handler))
        .layer(DefaultBodyLimit::disable())
        .route(
            "/persons/create",
            routing::post(routers::persons_create_handler),
        )
        .route(
            "/persons/{id}",
            routing::patch(routers::persons_patch_handler),
        )
        .route(
            "/persons/{id}",
            routing::delete(routers::persons_delete_handler),
        )
        .route(
            "/persons/{id}/verify",
            routing::get(routers::persons_id_verify_handler),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth_middleware,
        ));

    let admin_routers = Router::new()
        .route("/users", routing::get(routers::users_handler))
        .route(
            "/users/create",
            routing::post(routers::users_create_handler),
        )
        .route(
            "/users/{username}",
            routing::get(routers::users_username_handler),
        )
        .route(
            "/users/{username}",
            routing::delete(routers::users_username_delete_handler),
        )
        .route(
            "/users/{username}",
            routing::patch(routers::users_username_patch_handler),
        )
        .route(
            "/signature-keygen",
            routing::post(routers::signature_keygen_handler),
        )
        .route(
            "/signature-reset",
            routing::delete(routers::signature_reset_handler),
        )
        .route(
            "/persons/{id}/sign",
            routing::post(routers::persons_id_sign_handler),
        )
        .route(
            "/persons/{id}/unsign",
            routing::delete(routers::persons_id_unsign_handler),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth_middleware,
        ));

    let app = Router::new()
        .merge(public_routers)
        .merge(watchers_routers)
        .merge(editors_routers)
        .merge(admin_routers)
        .with_state(app_state)
        .layer(cors);

    log::info!("- Binding TCP Listener...");

    let listener = tokio::net::TcpListener::bind(ENDPOINT).await?;

    log::info!("🔗 Listening on http://{ENDPOINT}...");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
