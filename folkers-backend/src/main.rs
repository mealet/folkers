use axum::{Router, http::Method, routing};
use tower_http::cors::{self, CorsLayer};
use std::sync::LazyLock;

mod auth;
mod routers;
mod middleware;
mod database;

const ENDPOINT: &str = "0.0.0.0:3000";

pub static DATABASE: LazyLock<database::DatabaseClient> = LazyLock::new(database::DatabaseClient::init);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialization
    dotenvy::dotenv()?;
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("🚀 Folkers Backend Server");
    log::info!("⚙️ Starting initialization...");

    // Database setup

    log::info!("- Setting up database...");

    let db_username = std::env::var("FOLKERS_DB_USERNAME")?;
    let db_password = std::env::var("FOLKERS_DB_PASSWORD")?;
    let db_namespace = std::env::var("FOLKERS_DB_NAMESPACE")?;
    let db_database = std::env::var("FOLKERS_DB_DATABASE")?;
    let db_endpoint = std::env::var("FOLKERS_DB_ENDPOINT")?;

    DATABASE.setup(&db_endpoint, &db_namespace, &db_database, &db_username, &db_password).await?;

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
        jwt_service
    };

    // Main Application Router

    log::info!("- Setting up routers...");

    let public_routers = Router::new()
        .route("/", routing::get(routers::root_handler))
        .route("/login", routing::post(routers::login_handler));

    let protected_routers = Router::new()
        .route("/verify", routing::get(routers::verify_handler))
        .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), middleware::auth_middleware));

    let app = Router::new()
        .merge(public_routers)
        .merge(protected_routers)
        .with_state(app_state)
        .layer(cors);

    log::info!("- Binding TCP Listener...");

    let listener = tokio::net::TcpListener::bind(ENDPOINT).await?;

    log::info!("🔗 Listening on http://{ENDPOINT}...");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
