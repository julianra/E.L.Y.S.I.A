// ======================================================================
// 📍 FILE: elysia_core/src/http/mod.rs
// 📝 Central HTTP router for ELYSIA Core
// ======================================================================

use axum::{
    routing::{get, post},
    Router,
    middleware,
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

pub fn build_router(state: Arc<KernelState>) -> Router {
    Router::new()

        // ---------- AUTH ----------
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

        // ---------- STATUS ----------
        .route("/status", get({
            let s = state.clone();
            move || status(s.clone())
        }))

        // ---------- AI ----------
        .route("/ai/execute", post({
            let s = state.clone();
            move |payload| ai_execute(s.clone(), payload)
        }))

        // ---------- MODULES ----------
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

        // ---------- GLOBAL GUARD ----------
        .layer(middleware::from_fn({
            let s = state.clone();
            move |req, next| http_access_guard(s.clone(), req, next)
        }))
        .route("/modules/upload", post(upload::upload_module))

}
