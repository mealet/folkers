use super::auth;
use axum::{
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};

/// JWT token verification middleware
pub async fn auth_middleware(
    State(app_state): State<super::routers::AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token_from_headers(request.headers()).ok_or(StatusCode::BAD_REQUEST)?;

    let claims = app_state
        .jwt_service
        .verify_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = AuthUser {
        id: claims.sub,
        username: claims.username,
        role: auth::user::UserRole::from_str(&claims.role),
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("authorization")?.to_str().ok()?;

    auth_header
        .strip_prefix("Bearer ")
        .map(|value| value.to_string())
}

/// Extract for user authentication
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub role: auth::user::UserRole,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
