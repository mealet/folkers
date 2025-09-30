use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Html},
};

use super::{auth, middleware};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: auth::UserRepository,
    pub jwt_service: auth::jwt::JwtService
}

// Routers

pub async fn root_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html("<h1>Backend API is working</h1>")
    )
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<auth::LoginRequest>
) -> Result<Json<auth::AuthResponse>, StatusCode> {
    let user = state.user_repo.find_by_username(&payload.username).await.ok_or(StatusCode::UNAUTHORIZED)?;

    if !state.user_repo.verify_password(&payload.username, &payload.password).await {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = state.jwt_service
        .generate_token(&user)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    log::info!("User `{}` authenticated via JWT token", payload.username);

    Ok(Json(auth::AuthResponse {
        token,
        token_type: "Bearer".to_string()
    }))
}

pub async fn verify_handler(
    auth_user: middleware::AuthUser 
) -> impl IntoResponse {
    Html(format!("{:?}", auth_user))
}
