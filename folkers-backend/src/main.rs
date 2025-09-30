use axum::{Router, http::Method, routing};
use tower_http::cors::{self, CorsLayer};

mod auth;
mod routers;
mod middleware;

const ENDPOINT: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialization
    dotenvy::dotenv()?;
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("Starting application...");

    // HTTP Cors
    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_headers(cors::Any)
        .allow_methods([Method::POST, Method::GET, Method::DELETE]);

    // App State
    
    let jwt_config = auth::jwt::JwtConfig::new()?;
    let jwt_service = auth::jwt::JwtService::new(jwt_config);
    let user_repo = auth::UserRepository::new();

    let app_state = routers::AppState {
        user_repo,
        jwt_service
    };

    // Main Application Router
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

    let listener = tokio::net::TcpListener::bind(ENDPOINT).await?;

    log::info!("Listening on {ENDPOINT}...");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
