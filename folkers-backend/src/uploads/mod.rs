use axum::{
    extract::{Multipart, Json},
    http::{StatusCode, HeaderMap},
    response::IntoResponse
};
use sha2::{Sha256, Digest};
use std::sync::LazyLock;

mod utils;

static UPLOADS_DIR: LazyLock<String> = LazyLock::new(|| {
    if let Ok(upload_dir) = std::env::var("FOLKERS_UPLOADS_DIR") {
        upload_dir
    } else {
        log::warn!("Environment variable `UPLOADS_DIR` is not found, default is `./uploads`");
        String::from("./uploads")
    }
});

pub async fn init_uploads() -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all(UPLOADS_DIR.to_owned()).await
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
            let filepath = format!("{}/{}", UPLOADS_DIR.to_owned(), filename);

            if !tokio::fs::try_exists(&filepath).await.unwrap_or(false) {
                tokio::fs::write(&filepath, &data).await
                    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            }

            return Ok(Json(hash_hex))
        }
    }
    
    Err((StatusCode::BAD_REQUEST, "No photo found".to_string()))
}

pub async fn get_photo(hash: impl AsRef<str>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let hash = hash.as_ref();
    let entries = tokio::fs::read_dir(UPLOADS_DIR.to_owned()).await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let mut files = tokio::fs::ReadDir::from(entries);

    while let Some(entry) = files.next_entry().await.unwrap() {
        let filename = entry.file_name().to_string_lossy().to_string();

        if filename.starts_with(&hash) {
            let filepath = format!("{}/{}", UPLOADS_DIR.to_owned(), filename);

            match tokio::fs::read(&filepath).await {
                Ok(content) => {
                    let mut headers = HeaderMap::new();

                    headers.insert("content-type", utils::get_content_type(&filename).parse().unwrap());
                    headers.insert("x-file-hash", hash.parse().unwrap());

                    return Ok((headers, content))
                },
                Err(_) => break
            }
        }
    }

    Err((StatusCode::NOT_FOUND, "Photo not found".to_string()))
}
