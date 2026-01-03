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
//   - Na volledige upload: automatische install
// ======================================================================

use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

// Hyper 1 streaming
use http_body_util::BodyExt;

// install trigger
use crate::http::install;

pub async fn upload_module(mut req: Request<Body>) -> impl IntoResponse {
    log::info!("[CORE][UPLOAD] Incoming module upload");

    // --------------------------------------------------
    // Filename uit header
    // --------------------------------------------------
    let filename = match req.headers().get("x-filename") {
        Some(v) => match v.to_str() {
            Ok(s) => s,
            Err(_) => {
                log::warn!("[CORE][UPLOAD] Invalid X-Filename header");
                return StatusCode::BAD_REQUEST;
            }
        },
        None => {
            log::warn!("[CORE][UPLOAD] Missing X-Filename header");
            return StatusCode::BAD_REQUEST;
        }
    };

    if !filename.to_lowercase().ends_with(".zip") {
        log::warn!("[CORE][UPLOAD] Rejected non-zip upload: {}", filename);
        return StatusCode::UNSUPPORTED_MEDIA_TYPE;
    }

    // --------------------------------------------------
    // AppData/Local/elysia/uploads/modules
    // --------------------------------------------------
    let base_dir = match dirs::data_local_dir() {
        Some(p) => p.join("elysia").join("uploads").join("modules"),
        None => {
            log::error!("[CORE][UPLOAD] data_local_dir unavailable");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if let Err(e) = fs::create_dir_all(&base_dir).await {
        log::error!("[CORE][UPLOAD] Failed to create upload dir: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let upload_id = Uuid::new_v4().to_string();
    let path: PathBuf = base_dir.join(format!("{}.zip", upload_id));

    log::info!(
        "[CORE][UPLOAD] Writing upload {} → {}",
        upload_id,
        path.display()
    );

    let mut file = match fs::File::create(&path).await {
        Ok(f) => f,
        Err(e) => {
            log::error!(
                "[CORE][UPLOAD] Failed to create file {}: {}",
                path.display(),
                e
            );
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // --------------------------------------------------
    // STREAM body → disk
    // --------------------------------------------------
    let mut body = req.into_body();
    let mut total_bytes: u64 = 0;

    while let Some(frame) = body.frame().await {
        let frame = match frame {
            Ok(f) => f,
            Err(e) => {
                log::error!(
                    "[CORE][UPLOAD] Stream error for {}: {}",
                    upload_id,
                    e
                );
                let _ = fs::remove_file(&path).await;
                return StatusCode::BAD_REQUEST;
            }
        };

        if let Some(data) = frame.data_ref() {
            total_bytes += data.len() as u64;

            if let Err(e) = file.write_all(data).await {
                log::error!(
                    "[CORE][UPLOAD] Write failed for {}: {}",
                    upload_id,
                    e
                );
                let _ = fs::remove_file(&path).await;
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }

    // --------------------------------------------------
    // Flush → upload is NU pas klaar
    // --------------------------------------------------
    if let Err(e) = file.flush().await {
        log::error!(
            "[CORE][UPLOAD] Flush failed for {}: {}",
            upload_id,
            e
        );
        let _ = fs::remove_file(&path).await;
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    log::info!(
        "[CORE][UPLOAD] Upload complete | id={} bytes={}",
        upload_id,
        total_bytes
    );

    // --------------------------------------------------
    // AUTO-INSTALL (PAS NA VOLLEDIGE UPLOAD)
    // --------------------------------------------------
    log::info!(
        "[CORE][UPLOAD] Triggering install for {}",
        upload_id
    );

    let _ = install::install_module(Path(upload_id.clone())).await;

    log::info!(
        "[CORE][UPLOAD] Install finished for {}",
        upload_id
    );

    StatusCode::OK
}
