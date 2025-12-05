// ======================================================================
// 📍 FILE: elysia_core/src/http/auth_routes.rs
//
// 📝 BESCHRIJVING:
//   HTTP-router voor alle auth-gerelateerde routes van ELYSIA Core.
//
//   Mount de volgende endpoints:
//     - GET  /auth/initial_state  → Bestaat er al een admin?
//     - POST /auth/create_admin   → Eerste admin-account aanmaken
//     - POST /auth/login          → Inloggen en token ontvangen
//
//   Deze router wordt vanuit `http::build_http_router` gemerged met
//   de rest van de Core routes, zodat de Vite/Svelte UI één uniforme
//   API krijgt op poort 2022.
// ======================================================================

use axum::{routing::{get, post}, Router};
use crate::auth::*;
use crate::kernel::KernelState;

pub fn auth_routes(state: KernelState) -> Router {
    Router::new()
        .route("/auth/initial_state", get(get_initial_state))
        .route("/auth/create_admin", post(create_admin))
        .route("/auth/login", post(login))
        .with_state(state)
}
