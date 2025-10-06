use axum::{
    extract::{Multipart, Json},
    http::StatusCode
};
use sha2::{Sha256, Digest};

mod utils;

const UPLOADS_DIR: &str = ".uploads";

pub async fn init_uploads() -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all(UPLOADS_DIR).await
}

pub async fn upload_photo(mut multipart: Multipart) -> Result<Json<String>, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");

        if name == "photo" {
            let content_type = field.content_type().unwrap_or("").to_string();
            let data = field.bytes().await.unwrap();

            let hash= Sha256::digest(&data);
            let hash_hex = hex::encode(hash);

            let file_extension = utils::get_file_extension(&content_type);
            let filename = format!("{}.{}", hash_hex, file_extension);
            let filepath = format!("{}/{}", UPLOADS_DIR, filename);

            if !tokio::fs::try_exists(&filepath).await.unwrap_or(false) {
                tokio::fs::write(&filepath, &data).await
                    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            }

            return Ok(Json(hash_hex))
        }
    }
    
    Err((StatusCode::BAD_REQUEST, "No photo found".to_string()))
}
