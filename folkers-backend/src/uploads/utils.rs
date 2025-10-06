pub fn get_file_extension(content_type: &str) -> &str {
    match content_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        _ => "bin",
    }
}

pub fn get_content_type(filename: &str) -> &str {
    match filename {
        fname if fname.ends_with(".jpg") || fname.ends_with(".jpeg") => "image/jpeg",
        fname if fname.ends_with(".png") => "image/png",
        fname if fname.ends_with(".gif") => "image/gif",
        fname if fname.ends_with(".webp") => "image/webp",
        fname if fname.ends_with(".svg") => "image/svg+xml",
        _ => "application/octet-stream"
    }
}
