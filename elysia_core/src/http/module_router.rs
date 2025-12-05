// ======================================================================
// 📍 FILE: elysia_core/src/http/module_router.rs
//
// 📝 BESCHRIJVING:
//   Automatisch mounten van module API-routes onder:
//
//       /api/<module name>/*
//
//   Dit systeem ondersteunt ALLE modules zonder extra code in de kernel.
//
// ======================================================================

use axum::{Router};
use crate::kernel::KernelState;

pub fn mount_module_routes(state: KernelState) -> Router {
    let mut app = Router::new();

    for module in state.modules.iter() {
        let name = module.name();
        let router = module.routes();

        let mounted = Router::new()
            .nest(&format!("/api/{}", name), router);

        app = app.merge(mounted);
    }

    app
}
