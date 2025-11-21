use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

use super::{DATABASE, auth, database, middleware, uploads, signatures};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: auth::UserRepository,
    pub jwt_service: auth::jwt::JwtService,
}

// INFO: Public Routers

/// GET `/`
pub async fn root_handler() -> impl IntoResponse {
    (StatusCode::OK, Html("<h1>Backend API is working</h1>"))
}

/// GET `/health`
pub async fn health_handler() -> impl IntoResponse {
    StatusCode::OK
}

/// POST `/login`
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<auth::LoginRequest>,
) -> Result<Json<auth::AuthResponse>, StatusCode> {
    let user = state
        .user_repo
        .find_by_username(&payload.username)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    if !state
        .user_repo
        .verify_password(&payload.username, &payload.password)
        .await
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = state
        .jwt_service
        .generate_token(&user)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    log::info!(
        "User `{} ({})` [POST /login] authenticated via JWT token",
        user.username,
        user.id
    );

    Ok(Json(auth::AuthResponse {
        token,
        token_type: "Bearer".to_string(),
    }))
}

// INFO: Watchers Routers

/// GET `/me`
pub async fn me_handler(
    auth_user: middleware::AuthUser,
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    let user = DATABASE.get_user(auth_user.id).await;

    match user {
        Some(record) => Ok(Json(record)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// GET `/persons`
pub async fn persons_handler(
    auth_user: middleware::AuthUser,
    // Json(payload): Json<Option<database::person::SearchPersonRecord>>
) -> Result<Json<Vec<database::person::PersonRecord>>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Watcher {
        return Err(StatusCode::FORBIDDEN);
    }

    // let records_list = match payload {
    //     Some(search_payload) => {
    //         DATABASE.find_person(&search_payload.search_query).await.or_else(|err| {
    //             log::error!("`{} ({})` [GET /persons] [PAYLOAD: '{}'] got database error: {}", auth_user.username, auth_user.id, search_payload.search_query, err);
    //             Err(StatusCode::INTERNAL_SERVER_ERROR)
    //         })?
    //     },
    //     None => {
    //         DATABASE.list_persons().await.or_else(|err| {
    //             log::error!("`{} ({})` [GET /persons] [NO PAYLOAD] got database error: {}", auth_user.username, auth_user.id, err);
    //             Err(StatusCode::INTERNAL_SERVER_ERROR)
    //         })?
    //     }
    // };

    let records_list = DATABASE.list_persons().await.map_err(|err| {
        log::error!(
            "`{} ({})` [GET /persons] got database error: {}",
            auth_user.username,
            auth_user.id,
            err
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(records_list))
}

/// GET `/persons/{id}`
pub async fn persons_id_handler(
    auth_user: middleware::AuthUser,
    Path(id): Path<String>,
) -> Result<Json<database::person::PersonRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Watcher {
        return Err(StatusCode::FORBIDDEN);
    }

    match DATABASE.get_person(id).await {
        Some(record) => Ok(Json(record)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn media_handler(
    auth_user: middleware::AuthUser,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    if auth_user.role < auth::user::UserRole::Watcher {
        return Err((StatusCode::FORBIDDEN, "Not enough permissions".to_string()));
    }

    uploads::get_photo(hash).await
}

// INFO: Editors Routers

pub async fn upload_handler(
    auth_user: middleware::AuthUser,
    multipart: Multipart,
) -> Result<Json<String>, (StatusCode, String)> {
    if auth_user.role < auth::user::UserRole::Editor {
        return Err((StatusCode::FORBIDDEN, "Not enough permissions".to_string()));
    }

    let result = uploads::upload_photo(multipart).await?;

    log::info!(
        "`{} ({})` uploaded photo with hash: '{}'",
        auth_user.username,
        auth_user.id,
        result.0
    );

    Ok(result)
}

/// POST `/persons/create`
pub async fn persons_create_handler(
    auth_user: middleware::AuthUser,
    new_record: Json<database::person::CreatePersonRecord>,
) -> Result<Json<database::person::PersonRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Editor {
        return Err(StatusCode::FORBIDDEN);
    }

    // unique record verification
    if !DATABASE
        .find_person(format!(
            "{} {} {}",
            new_record.surname, new_record.name, new_record.patronymic
        ))
        .await
        .map_err(|err| {
            log::error!(
                "`{} ({})` [POST /persons/create] got database verification error: {}",
                auth_user.username,
                auth_user.id,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_empty()
    {
        return Err(StatusCode::CONFLICT);
    }

    let option_record = DATABASE
        .add_person(new_record.0, &auth_user.username)
        .await
        .map_err(|err| {
            log::error!(
                "`{} ({})` [POST /persons/create] got database error: {}",
                auth_user.username,
                auth_user.id,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if option_record.is_none() {
        log::error!(
            "`{} ({})` [POST /persons/create] got empty database response",
            auth_user.username,
            auth_user.id
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let person_record = option_record.unwrap();

    log::info!(
        "`{} ({})` [POST /persons/create] created person record: {} {} {}",
        auth_user.username,
        auth_user.id,
        person_record.name,
        person_record.surname,
        person_record.patronymic
    );

    Ok(Json(person_record))
}

/// PATCH `/persons/{id}`
pub async fn persons_patch_handler(
    auth_user: middleware::AuthUser,
    Path(id): Path<String>,
    patched: Json<database::person::CreatePersonRecord>,
) -> Result<Json<database::person::PersonRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Editor {
        return Err(StatusCode::FORBIDDEN);
    }

    let person_record = DATABASE.get_person(&id).await;

    match person_record {
        Some(record) => {
            // verifying if we have access

            if auth_user.role < auth::user::UserRole::Admin && record.author != auth_user.username {
                return Err(StatusCode::FORBIDDEN);
            }

            let record = DATABASE
                .update_person(id, patched.0.clone())
                .await
                .map_err(|err| {
                    log::error!(
                        "`{} ({})` [PATCH /persons/{{id}}] got database error: {}",
                        auth_user.username,
                        auth_user.id,
                        err
                    );
                    StatusCode::INTERNAL_SERVER_ERROR
                })?
                .unwrap_or(record);

            log::info!(
                "`{} ({})` [PATCH /persons/{{id}}] updated `{} {} {}` -> `{} {} {}`",
                auth_user.username,
                auth_user.id,
                record.surname,
                record.name,
                record.patronymic,
                patched.surname,
                patched.name,
                patched.patronymic
            );

            Ok(Json(record))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// DELETE `/persons/{id}`
pub async fn persons_delete_handler(
    auth_user: middleware::AuthUser,
    Path(id): Path<String>,
) -> Result<Json<database::person::PersonRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Editor {
        return Err(StatusCode::FORBIDDEN);
    }

    let person_record = DATABASE.get_person(&id).await;

    match person_record {
        Some(record) => {
            // verifying if we have access

            if auth_user.role < auth::user::UserRole::Admin && record.author != auth_user.username {
                return Err(StatusCode::FORBIDDEN);
            }

            let _ = DATABASE.delete_person(id).await.map_err(|err| {
                log::error!(
                    "`{} ({})` [DELETE /persons/{{id}}] got database error: {}",
                    auth_user.username,
                    auth_user.id,
                    err
                );
                StatusCode::INTERNAL_SERVER_ERROR
            });

            log::info!(
                "`{} ({})` [DELETE /persons/{{id}}] deleted `{} {} {}`",
                auth_user.username,
                auth_user.id,
                record.surname,
                record.name,
                record.patronymic
            );

            Ok(Json(record))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// INFO: Admins Routers

/// GET `/users`
pub async fn users_handler(
    auth_user: middleware::AuthUser,
) -> Result<Json<Vec<database::user::UserRecord>>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let users_list = DATABASE.list_users().await.map_err(|err| {
        log::error!(
            "`{} ({})` [GET /users] got database error: {}",
            auth_user.username,
            auth_user.id,
            err
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(users_list))
}

/// POST `/users/create`
pub async fn users_create_handler(
    auth_user: middleware::AuthUser,
    mut new_record: Json<database::user::CreateUserRecord>,
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // hashing password
    new_record.password =
        auth::UserRepository::hash_password(&new_record.password).map_err(|err| {
            log::error!(
                "`{} ({})` [POST /users/create] got HASHING ERROR: {}",
                auth_user.username,
                auth_user.id,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // replacing field with current username (to avoid replacements)
    new_record.created_by = auth_user.username.clone();

    let option_record = DATABASE.create_user(new_record.0).await.map_err(|err| {
        log::error!(
            "`{} ({})` [POST /users/create] got database error: {}",
            auth_user.username,
            auth_user.id,
            err
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if option_record.is_none() {
        log::error!(
            "`{} ({})` [POST /users/create] got empty database response",
            auth_user.username,
            auth_user.id
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let user_record = option_record.unwrap();

    log::info!(
        "`{} ({})` [POST /users/create] created `{} ({}) role: {}`",
        auth_user.username,
        auth_user.id,
        user_record.username,
        user_record
            .id
            .clone()
            .map(|id| id.id.to_string())
            .unwrap_or_default(),
        user_record.role
    );

    Ok(Json(user_record))
}

/// GET `/users/{username}`
pub async fn users_username_handler(
    auth_user: middleware::AuthUser,
    Path(username): Path<String>,
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin && auth_user.username != username {
        return Err(StatusCode::FORBIDDEN);
    }

    let option_record = DATABASE
        .get_user_by_username(username)
        .await
        .map_err(|err| {
            log::error!(
                "`{} ({})` [GET /users/{{username}}] got database error: {}",
                auth_user.username,
                auth_user.id,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match option_record {
        Some(record) => Ok(Json(record)),
        None => Err(StatusCode::NOT_FOUND),
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

    let user_record = DATABASE
        .get_user_by_username(username)
        .await
        .map_err(|err| {
            log::error!(
                "`{} ({})` [DELETE /users/{{username}}] got database error: {}",
                auth_user.username,
                auth_user.id,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match user_record {
        Some(record) => {
            let _ = DATABASE
                .delete_user(record.id.clone().unwrap().id.to_string())
                .await
                .map_err(|err| {
                    log::error!(
                        "`{} ({})` [DELETE /users/{{username}}] got database error: {}",
                        auth_user.username,
                        auth_user.id,
                        err
                    );
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            log::info!(
                "`{} ({})` [DELETE /users/{{username}}] deleted user `{} ({}) role: {}`",
                auth_user.username,
                auth_user.id,
                record.username,
                record.id.as_ref().map(|id| id.id.to_string()).unwrap(),
                record.role
            );
            Ok(Json(record))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// PATCH `/users/{username}`
pub async fn users_username_patch_handler(
    auth_user: middleware::AuthUser,
    Path(username): Path<String>,
    mut patched: Json<database::user::CreateUserRecord>,
) -> Result<Json<database::user::UserRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_record = DATABASE
        .get_user_by_username(username)
        .await
        .map_err(|err| {
            log::error!(
                "`{} ({})` [PATCH /users/{{username}}] got database error: {}",
                auth_user.username,
                auth_user.id,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match user_record {
        Some(record) => {
            // disabling `created_by` field patching

            patched.created_by = record.created_by.clone();
            
            if patched.password.is_empty() {
                patched.password = record.password.clone();
            } else {
                patched.password = auth::UserRepository::hash_password(&patched.password).map_err(|err| {
                    log::error!(
                        "`{} ({})` [POST /users/create] got HASHING ERROR: {}",
                        auth_user.username,
                        auth_user.id,
                        err
                    );
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            }

            let _ = DATABASE
                .update_user(record.id.clone().unwrap().id.to_string(), patched.0.clone())
                .await
                .map_err(|err| {
                    log::error!(
                        "`{} ({})` [PATCH /users/{{username}}] got database error: {}",
                        auth_user.username,
                        auth_user.id,
                        err
                    );
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            let password_changed =
                record.password != patched.password && !patched.password.is_empty();

            log::info!(
                "`{} ({})` [PATCH /users/{{username}}] updated user `{} ({}), role: {}` -> `{}, role: {}, password changed: {}`",
                auth_user.username,
                auth_user.id,
                record.username,
                record.id.as_ref().map(|id| id.id.to_string()).unwrap(),
                record.role,
                patched.username,
                patched.role,
                password_changed
            );

            Ok(Json(record))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST `/signature-keygen`
pub async fn signature_keygen_handler(
    auth_user: middleware::AuthUser,
) -> Result<Json<String>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user = DATABASE.get_user(&auth_user.id).await;

    match user {
        Some(user) => {
            if user.public_key.is_some() {
                return Err(StatusCode::CONFLICT);
            }

            let (private_key, public_key) = signatures::generate_signing_keypair();
            let _ = DATABASE.update_user_pubkey(&auth_user.id, Some(public_key)).await.or_else(|err| {
                log::error!("`{} ({})` [POST /signature-keygen] got database error: {}", auth_user.username, auth_user.id, err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            })?;

            return Ok(Json(private_key));
        },
        None => Err(StatusCode::NOT_FOUND)
    }
}

/// DELETE `/signature-reset`
pub async fn signature_reset_handler(
    auth_user: middleware::AuthUser,
) -> Result<(), StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user = DATABASE.get_user(&auth_user.id).await;

    match user {
        Some(user) => {
            if user.public_key.is_none() {
                return Err(StatusCode::NO_CONTENT);
            }

            let _ = DATABASE.update_user_pubkey(&auth_user.id, None).await.or_else(|err| {
                log::error!("`{} ({})` [POST /signature-keygen] got database error: {}", auth_user.username, auth_user.id, err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            })?;

            return Ok(());
        },
        None => Err(StatusCode::NOT_FOUND)
    }
}

/// POST `/persons/{id}/sign`
pub async fn persons_id_sign_handler(
    auth_user: middleware::AuthUser,
    Path(id): Path<String>,
    payload: Json<database::signature::SignRecordPayload>,
) -> Result<Json<database::signature::RecordSignatureRecord>, StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // verifying that record isn't unsigned yet
    
    let existing_signature = DATABASE.get_signature(&id).await;

    if let Ok(existing_signature) = existing_signature {
        if existing_signature.is_some() {
            return Err(StatusCode::CONFLICT);
        }
    }

    // now signing this record
    
    let record = DATABASE.get_person(&id).await;

    match record {
        Some(record) => {
            let signature = signatures::sign_record(record.clone(), payload.private_key.clone()).or_else(|err| {
                log::error!("`{} ({})` [POST /persons/{{id}}] got signature error: {}", auth_user.username, auth_user.id, err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            })?;

            let db_result = DATABASE.add_signature(signature, &auth_user.username).await.or_else(|err| {
                log::error!("`{} ({})` [POST /persons/{{id}}] got database error: {}", auth_user.username, auth_user.id, err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)               
            })?.unwrap();

            log::info!("`{} ({})` [POST /persons/{{id}}] signed record `{}`", auth_user.username, auth_user.id, record.id.map(|x| x.id.to_string()).unwrap_or_default());

            Ok(Json(db_result))
        },
        None => Err(StatusCode::NOT_FOUND)
    }
}

/// DELETE `/persons/{id}/unsign`
pub async fn persons_id_unsign_handler(
    auth_user: middleware::AuthUser,
    Path(id): Path<String>,
) -> Result<(), StatusCode> {
    if auth_user.role < auth::user::UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // verifying that record isn't unsigned yet
    
    let existing_signature = DATABASE.get_signature(&id).await.or_else(|err| {
        log::error!("`{} ({})` [DELETE /persons/{{id}}/unsign] got database error: {}", auth_user.username, auth_user.id, err);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    if let Some(signature) = existing_signature {
        let static_admin = std::env::var("FOLKERS_STATIC_ADMIN_USERNAME").unwrap_or_default();

        if auth_user.username != signature.signed_by && auth_user.username != static_admin {
            return Err(StatusCode::FORBIDDEN);
        }

        let _ = DATABASE.delete_signature(&id).await.or_else(|err| {
            log::error!("`{} ({})` [DELETE /persons/{{id}}/unsign] got database error: {}", auth_user.username, auth_user.id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        })?;

        log::info!("`{} ({})` [DELETE /persons/{{id}}/unsign] deleted signature on record `{}`", auth_user.username, auth_user.id, signature.record_id);

        return Ok(());
    }

    return Err(StatusCode::NOT_FOUND);
}


/// GET `/persons/{id}/verify`
pub async fn persons_id_verify_handler(
    auth_user: middleware::AuthUser,
    Path(id): Path<String>,
) -> Result<Json<database::signature::RecordSignatureRecord>, StatusCode> {
    let record = DATABASE.get_person(&id).await;

    match record {
        Some(record) => {
            let signature_record = DATABASE.get_signature(&id).await.or_else(|err| {
                log::error!("`{} ({})` [GET /persons/{{id}}/verify] got database error: {}", auth_user.username, auth_user.id, err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            })?;

            if let Some(signature) = signature_record {
                let author_record = DATABASE.get_user_by_username(signature.signed_by.clone()).await.or_else(|err| {
                    log::error!("`{} ({})` [GET /persons/{{id}}/verify] got database error: {}", auth_user.username, auth_user.id, err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                })?;

                if let Some(author) = author_record {
                    if author.public_key.is_none() {
                        return Err(StatusCode::FORBIDDEN);
                    }

                    let verification = signatures::verify_record(record, signatures::RecordSignature {
                        record_id: signature.record_id.clone(),
                        base64: signature.base64.clone(),
                        pubkey: author.public_key.unwrap()
                    }).or_else(|_| {
                        return Err(StatusCode::FORBIDDEN)
                    })?;
                    
                    if !verification {
                        return Err(StatusCode::FORBIDDEN)
                    }

                    return Ok(Json(signature));
                }
            }

            return Err(StatusCode::NOT_FOUND);
        },
        None => Err(StatusCode::NOT_FOUND)
    }
}
