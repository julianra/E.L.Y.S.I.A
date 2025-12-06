// ======================================================================
// 📍 FILE: elysia_core/src/http/mod.rs
//
// 📝 BESCHRIJVING:
//   Combineert alle HTTP-routes van de kernel.
//   Mount enterprise security middleware.
// ======================================================================

use axum::{Router, middleware::from_fn};
use crate::kernel::KernelState;

mod core_routes;
mod auth_routes;
mod pairing_routes;
mod module_router;
pub mod auth_middleware;

pub fn build_http_router(state: KernelState) -> Router {
    Router::new()
        .merge(core_routes::routes(state.clone()))
        .merge(auth_routes::auth_routes(state.clone()))
        .merge(pairing_routes::pairing_routes(state.clone()))
        .merge(module_router::mount_module_routes(&state))
        // ENTERPRISE SECURITY MIDDLEWARE
        .layer(from_fn(auth_middleware::auth_layer))
        .layer(axum::Extension(state))
}
