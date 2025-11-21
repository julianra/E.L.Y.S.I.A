// Elysia Rust - api/mod.rs
// ===============================================
// FILE: src/api/mod.rs
// ROLE: API Core
// PART OF: API Layer
// PURPOSE:
// - Beheer van externe API requests
// - Definities van externe request structs
// ===============================================
pub mod external;
use axum::{Router, routing::post};
use crate::api::external::add_agenda_http;

pub fn api_router() -> Router {
    Router::new()
        .route("/external/agenda/add", post(add_agenda_http))
        // ... andere routes
}
