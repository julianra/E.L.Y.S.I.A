// ======================================================================
// 📍 FILE: elysia_core/src/http/core_routes.rs
// ======================================================================

use axum::{Router, routing::get, Json};
use serde_json::json;
use crate::kernel::KernelState;

pub fn routes(state: KernelState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/status", get({
            let state = state.clone();
            move || async move {
                Json(json!({
                    "status": "running",
                    "modules": state.modules.len(),   // ✔ werkt nu via registry.len()
                    "db": "online",
                    "port": 2022,
                    "version": "2.0"
                }))
            }
        }))
        .route("/nodes", get(|| async {
            Json(json!({ "nodes": [] }))
        }))
        .route("/meta", get({
            let state = state.clone();
            move || async move {
                let meta = state.ctx.meta.read().unwrap().clone();
                Json(json!({ "meta": meta }))
            }
        }))
}
