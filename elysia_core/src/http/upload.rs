// ======================================================================
// 📍 FILE: elysia_core/src/http/upload.rs
// 📝 ROLE:
//   Inerte module upload (ZIP only)
//   - Admin-only
//   - Opslag in uploads/modules (AppData)
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
    // --------------------------------------------------
    // Lees multipart field
    // --------------------------------------------------
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

    // --------------------------------------------------
    // Bepaal productie-correct datapad
    // %LOCALAPPDATA%/elysia/uploads/modules
    // --------------------------------------------------
    let base_dir = match dirs::data_local_dir() {
        Some(p) => p.join("elysia").join("uploads").join("modules"),
        None => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let id = Uuid::new_v4().to_string();
    let path: PathBuf = base_dir.join(format!("{}.zip", id));

    // --------------------------------------------------
    // Zorg dat directory bestaat
    // --------------------------------------------------
    if let Err(_) = fs::create_dir_all(&base_dir).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // --------------------------------------------------
    // Schrijf ZIP bestand
    // --------------------------------------------------
    if let Err(_) = fs::write(&path, data).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
