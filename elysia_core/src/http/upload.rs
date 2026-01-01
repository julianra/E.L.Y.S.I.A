// ======================================================================
// 📍 FILE: elysia_core/src/http/upload.rs
// 📝 ROLE:
//   Inerte module upload (RAW ZIP stream)
//   - Admin-only (guard)
//   - Content-Type: application/octet-stream
//   - X-Filename header verplicht
//   - Streaming → disk (multi-GB safe)
//   - GEEN multipart
//   - GEEN buffering
//   - GEEN side effects
// ======================================================================

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use tokio::{
    fs,
    io::AsyncWriteExt,
};
use uuid::Uuid;
use std::path::PathBuf;

// CRUCIAAL: Hyper 1 streaming trait
use http_body_util::BodyExt;

pub async fn upload_module(
    mut req: Request<Body>,
) -> impl IntoResponse {
    // --------------------------------------------------
    // Filename uit header
    // --------------------------------------------------
    let filename = match req.headers().get("x-filename") {
        Some(v) => match v.to_str() {
            Ok(s) => s,
            Err(_) => return StatusCode::BAD_REQUEST,
        },
        None => return StatusCode::BAD_REQUEST,
    };

    if !filename.to_lowercase().ends_with(".zip") {
        return StatusCode::UNSUPPORTED_MEDIA_TYPE;
    }

    // --------------------------------------------------
    // %LOCALAPPDATA%/elysia/uploads/modules
    // --------------------------------------------------
    let base_dir = match dirs::data_local_dir() {
        Some(p) => p.join("elysia").join("uploads").join("modules"),
        None => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    if fs::create_dir_all(&base_dir).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let id = Uuid::new_v4().to_string();
    let path: PathBuf = base_dir.join(format!("{}.zip", id));

    let mut file = match fs::File::create(&path).await {
        Ok(f) => f,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // --------------------------------------------------
    // STREAM body → disk (frame-based, correct)
    // --------------------------------------------------
    let mut body = req.into_body();

    while let Some(frame) = body.frame().await {
        let frame = match frame {
            Ok(f) => f,
            Err(_) => {
                let _ = fs::remove_file(&path).await;
                return StatusCode::BAD_REQUEST;
            }
        };

        if let Some(data) = frame.data_ref() {
            if file.write_all(data).await.is_err() {
                let _ = fs::remove_file(&path).await;
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }

    if file.flush().await.is_err() {
        let _ = fs::remove_file(&path).await;
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
