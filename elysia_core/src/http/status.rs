// ======================================================================
// 📍 FILE: elysia_core/src/http/status.rs
// ======================================================================

use axum::Json;
use serde::Serialize;
use std::sync::Arc;

use crate::KernelState;

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub version: String,
    pub db: String,
    pub modules: usize,
}

pub async fn status(state: Arc<KernelState>) -> Json<StatusResponse> {
    let count = state.modules.read().await.len();

    Json(StatusResponse {
        status: "online".into(),
        version: "2.1".into(),
        db: "ok".into(),
        modules: count,
    })
}
