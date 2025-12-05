// ======================================================================
// 📍 FILE: elysia_core/src/http/mod.rs
//
// 📝 BESCHRIJVING:
//   De hoofdrouter van ELYSIA Core. Combineert:
//      - Core routes (/health, /nodes, /status)
//      - Automatisch gemounte module routes (/api/<module>)
//   Dit is de centrale HTTP entrypoint van het OS.
//
// ======================================================================

use axum::Router;
use crate::kernel::KernelState;

mod core_routes;
mod module_router;
pub use core_routes::*;
pub use module_router::mount_module_routes;

pub fn build_http_router(state: KernelState) -> Router {
    let mut app = Router::new();

    // CORE ROUTES
    app = app.merge(core_routes::routes(state.clone()));

    // MODULE ROUTES
    app = app.merge(mount_module_routes(state));

    app
}
