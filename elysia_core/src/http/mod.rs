// ======================================================================
// 📍 FILE: elysia_core/src/http/mod.rs
// 📝 Central HTTP router for ELYSIA Core
//
// BELANGRIJK:
// - Geen CORS (Electron / native UI)
// - Body size limit UIT (vereist voor grote uploads)
// - GEEN multipart (raw binary upload)
// - Guard blijft actief
// ======================================================================

use axum::{
    routing::{get, post},
    Router,
    middleware,
    extract::DefaultBodyLimit,
};
use std::sync::Arc;

use crate::KernelState;

mod guard;
mod auth;
mod modules;
mod status;
mod ai;
pub mod upload;
pub mod install;

use guard::http_access_guard;
use auth::*;
use modules::*;
use status::*;
use ai::*;

use tower_http::limit::RequestBodyLimitLayer;

pub fn build_router(state: Arc<KernelState>) -> Router {
    Router::new()

        // ==================================================
        // ❗ LIMITS UIT (RAW BODY STREAMING, MULTI-GB)
        // ==================================================
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(usize::MAX))

        // ==================================================
        // AUTH
        // ==================================================
        .route("/auth/has_admin", get({
            let s = state.clone();
            move || has_admin(s.clone())
        }))
        .route("/auth/create_admin", post({
            let s = state.clone();
            move |payload| create_admin(s.clone(), payload)
        }))
        .route("/auth/login", post({
            let s = state.clone();
            move |payload| login(s.clone(), payload)
        }))

        // ==================================================
        // STATUS
        // ==================================================
        .route("/status", get({
            let s = state.clone();
            move || status(s.clone())
        }))

        // ==================================================
        // AI
        // ==================================================
        .route("/ai/execute", post({
            let s = state.clone();
            move |payload| ai_execute(s.clone(), payload)
        }))

        // ==================================================
        // MODULES
        // ==================================================
        .route("/modules", get({
            let s = state.clone();
            move || list_modules(s.clone())
        }))
        .route("/modules/:id/pair", post({
            let s = state.clone();
            move |path| pair_module(s.clone(), path)
        }))
        .route("/modules/:id/unpair", post({
            let s = state.clone();
            move |path| unpair_module(s.clone(), path)
        }))

        // ==================================================
        // MODULE UPLOAD (RAW BODY, STREAMING)
        // ==================================================
        .route("/modules/upload", post(upload::upload_module))

        // ==================================================
        // MODULE INSTALL
        // ==================================================
        .route("/modules/install/:upload_id", post(install::install_module))

        // ==================================================
        // 🔒 GUARD (ALTIJD LAATST)
        // ==================================================
        .layer(middleware::from_fn({
            let s = state.clone();
            move |req, next| http_access_guard(s.clone(), req, next)
        }))
}
