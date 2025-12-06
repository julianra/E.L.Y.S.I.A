// ======================================================================
// 📍 FILE: elysia_core/src/http/mod.rs
// ======================================================================

use axum::{Router, Extension, middleware::from_fn};
use crate::kernel::KernelState;

mod core_routes;
mod pairing_routes;
mod module_router;
pub mod auth_middleware;

pub fn build_http_router(state: KernelState) -> Router {
    Router::new()
        .merge(core_routes::routes(state.clone()))
        .nest("/auth", crate::auth::http_router(state.clone()))
        .merge(pairing_routes::pairing_routes(state.clone()))
        .merge(module_router::mount_module_routes(&state))

        .layer(from_fn(auth_middleware::auth_layer))
        .layer(Extension(state))
}
