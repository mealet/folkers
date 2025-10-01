use axum::{
    Json,
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Html},
};

use super::{
    DATABASE,
    auth, middleware, database
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: auth::UserRepository,
    pub jwt_service: auth::jwt::JwtService
}

// INFO: Public Routers

/// GET `/`
pub async fn root_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html("<h1>Backend API is working</h1>")
    )
}

/// POST `/login`
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

    log::info!("User `{} ({})` [POST /login] authenticated via JWT token", user.username, user.id);

    Ok(Json(auth::AuthResponse {
        token,
        token_type: "Bearer".to_string()
    }))
}

// INFO: Editors Routers

/// GET `/verify`
pub async fn verify_handler(
    auth_user: middleware::AuthUser 
) -> impl IntoResponse {
    Html(format!("{:?}", auth_user))
}

// INFO: Admins Routers

/// GET `/users`
pub async fn users_handler(
    auth_user: middleware::AuthUser,
) -> Result<Json<Vec<database::user::UserRecord>>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let users_list = DATABASE.list_users().await.or_else(|err| {
        log::error!("`{} ({})` [GET /users] got database error: {}", auth_user.username, auth_user.id, err);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    return Ok(Json(users_list));
}

/// POST `/users/create`
pub async fn users_create_handler(
    auth_user: middleware::AuthUser,
    new_record: Json<database::user::CreateUserRecord>
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let option_record = DATABASE.create_user(new_record.0).await.or_else(|err| {
        log::error!("`{} ({})` [POST /users/create] got database error: {}", auth_user.username, auth_user.id, err);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    if option_record.is_none() {
        log::error!("`{} ({})` [POST /users/create] got empty database response", auth_user.username, auth_user.id);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let user_record = option_record.unwrap();

    log::info!("`{} ({})` [POST /users/create] created `{} ({}) role: {}`", auth_user.username, auth_user.id, user_record.username, user_record.id.clone().map(|id| id.id.to_string()).unwrap_or_default(), user_record.role);

    return Ok(Json(user_record))
}


/// GET `/users/{username}`
pub async fn users_username_handler(
    auth_user: middleware::AuthUser,
    Path(username): Path<String>,
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let option_record = DATABASE.get_user_by_username(username).await.or_else(|err| {
        log::error!("`{} ({})` [GET /users/{{username}}] got database error: {}", auth_user.username, auth_user.id, err);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    match option_record {
        Some(record) => Ok(Json(record)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

/// DELETE `/users/{username}`
pub async fn users_username_delete_handler(
    auth_user: middleware::AuthUser,
    Path(username): Path<String>,
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_record = DATABASE.get_user_by_username(username).await.or_else(|err| {
        log::error!("`{} ({})` [DELETE /users/{{username}}] got database error: {}", auth_user.username, auth_user.id, err);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    match user_record {
        Some(record) => {
            let _ = DATABASE.delete_user(record.id.clone().unwrap().id.to_string()).await
                .or_else(|err| {
                    log::error!("`{} ({})` [DELETE /users/{{username}}] got database error: {}", auth_user.username, auth_user.id, err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                })?;

            log::info!("`{} ({})` [DELETE /users/{{username}}] deleted user `{} ({}) role: {}`", auth_user.username, auth_user.id, record.username, record.id.as_ref().map(|id| id.id.to_string()).unwrap(), record.role);
            Ok(Json(record))
        },
        None => Err(StatusCode::NOT_FOUND)
    }
}

/// PATCH `/users/{username}`
pub async fn users_username_patch_handler(
    auth_user: middleware::AuthUser,
    Path(username): Path<String>,
    mut patched: Json<database::user::CreateUserRecord>
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_record = DATABASE.get_user_by_username(username).await.or_else(|err| {
        log::error!("`{} ({})` [PATCH /users/{{username}}] got database error: {}", auth_user.username, auth_user.id, err);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    match user_record {
        Some(record) => {
            // disabling `created_by` field patching

            patched.created_by = record.created_by.clone();

            let _ = DATABASE.update_user(record.id.clone().unwrap().id.to_string(), patched.0.clone()).await
                .or_else(|err| {
                    log::error!("`{} ({})` [PATCH /users/{{username}}] got database error: {}", auth_user.username, auth_user.id, err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                })?;

            let password_changed=  record.password != patched.password;

            log::info!("`{} ({})` [PATCH /users/{{username}}] updated user `{} ({}), role: {}` -> `{}, role: {}, password changed: {}`", auth_user.username, auth_user.id, record.username, record.id.as_ref().map(|id| id.id.to_string()).unwrap(), record.role, patched.username, patched.role, password_changed);

            Ok(Json(record))
        },
        None => Err(StatusCode::NOT_FOUND)
    }
}
