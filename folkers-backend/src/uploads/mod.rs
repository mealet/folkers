use axum::{
    extract::{Json, Multipart},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use sha2::{Digest, Sha256};
use std::sync::LazyLock;
use tokio::io::AsyncWriteExt;

mod utils;

static UPLOADS_DIR: LazyLock<String> = LazyLock::new(|| {
    if let Ok(upload_dir) = std::env::var("FOLKERS_UPLOADS_DIR") {
        upload_dir
    } else {
        log::warn!(
            "Environment variable `FOLKERS_UPLOADS_DIR` is not found, default is `./uploads`"
        );
        String::from("./uploads")
    }
});
pub const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50 MB

pub async fn init_uploads() -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all(UPLOADS_DIR.to_owned()).await
}

pub async fn upload_photo(mut multipart: Multipart) -> Result<Json<String>, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");

        if name == "photo" {
            let content_type = field.content_type().unwrap_or("").to_string();
            let file_extension = utils::get_file_extension(&content_type);

            let mut hasher = Sha256::new();
            let temp_path = format!(
                "{}/tmp_upload_{}.{}",
                UPLOADS_DIR.to_owned(),
                uuid::Uuid::new_v4(),
                file_extension
            );

            let mut file = match tokio::fs::File::create(&temp_path).await {
                Ok(file) => file,
                Err(error) => {
                    log::error!("Failed to create temporary uploader file: {}", error);

                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to create file: {}", error),
                    ));
                }
            };

            let mut total_size = 0usize;

            let mut stream = field;
            while let Ok(Some(chunk)) = stream.chunk().await {
                total_size += chunk.len();

                if total_size > MAX_FILE_SIZE {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    return Err((StatusCode::PAYLOAD_TOO_LARGE, "File too large".to_string()));
                }

                hasher.update(&chunk);

                if let Err(error) = file.write_all(&chunk).await {
                    let _ = tokio::fs::remove_file(&temp_path).await;

                    log::error!("Failed to write file: {}", error);

                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to write file: {}", error),
                    ));
                }
            }

            let hash_hex = hex::encode(hasher.finalize());

            let filename = format!("{}.{}", hash_hex, file_extension);
            let filepath = format!("{}/{}", UPLOADS_DIR.to_owned(), filename);

            if tokio::fs::try_exists(&filepath).await.unwrap_or(false) {
                let _ = tokio::fs::remove_file(&temp_path).await;
            } else if let Err(error) = tokio::fs::rename(&temp_path, &filepath).await {
                log::error!("Failed to move file: {}", error);

                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to move file: {}", error),
                ));
            }

            return Ok(Json(hash_hex));
        }
    }

    Err((StatusCode::BAD_REQUEST, "No photo found".to_string()))
}

pub async fn get_photo(hash: impl AsRef<str>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let hash = hash.as_ref();
    let entries = tokio::fs::read_dir(UPLOADS_DIR.to_owned())
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let mut files = entries;

    while let Some(entry) = files.next_entry().await.unwrap() {
        let filename = entry.file_name().to_string_lossy().to_string();

        if filename.starts_with(hash) {
            let filepath = format!("{}/{}", UPLOADS_DIR.to_owned(), filename);

            match tokio::fs::read(&filepath).await {
                Ok(content) => {
                    let mut headers = HeaderMap::new();

                    headers.insert(
                        "content-type",
                        utils::get_content_type(&filename).parse().unwrap(),
                    );
                    headers.insert("x-file-hash", hash.parse().unwrap());

                    return Ok((headers, content));
                }
                Err(_) => break,
            }
        }
    }

    Err((StatusCode::NOT_FOUND, "Photo not found".to_string()))
}
