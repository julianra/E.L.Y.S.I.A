// ======================================================================
// 📍 FILE: elysia_core/src/http/upload.rs
// 📝 ROLE:
//   Inerte module upload (ZIP only)
//   - Admin-only
//   - Opslag in uploads/modules
//   - GEEN extractie
//   - GEEN validatie
//   - GEEN side effects
// ======================================================================

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use std::path::PathBuf;
use tokio::fs;

pub async fn upload_module(
    mut multipart: Multipart,
) -> impl IntoResponse {
    let Some(field) = multipart.next_field().await.ok().flatten() else {
        return StatusCode::BAD_REQUEST;
    };

    let filename = field.file_name().unwrap_or("");
    if !filename.ends_with(".zip") {
        return StatusCode::UNSUPPORTED_MEDIA_TYPE;
    }

    let data = match field.bytes().await {
        Ok(d) => d,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let id = Uuid::new_v4().to_string();
    let mut path = PathBuf::from("elysia_data/uploads/modules");
    path.push(format!("{}.zip", id));

    if let Err(_) = fs::create_dir_all(path.parent().unwrap()).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    if let Err(_) = fs::write(&path, data).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
