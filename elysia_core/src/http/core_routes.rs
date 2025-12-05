// ======================================================================
// 📍 FILE: elysia_core/src/http/core_routes.rs
//
// 📝 BESCHRIJVING:
//   Bevat ALLE kernel-native routes van ELYSIA Core.
//   Dit is de “OS API”. Modules komen onder /api/…
//
//   Routes:
//     GET /health
//     GET /status
//     GET /nodes
//     GET /meta
//
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
                    "modules": state.modules.len(),
                    "db": "online",
                    "port": 2022,
                    "version": "2.0"
                }))
            }
        }))
        .route("/nodes", get({
            move || async {
                // Fase 3: node discovery output
                Json(json!({
                    "nodes": []
                }))
            }
        }))
        .route("/meta", get({
            let state = state.clone();
            move || async move {
                let meta = state.ctx.meta.read().unwrap().clone();
                Json(json!({ "meta": meta }))
            }
        }))
}
