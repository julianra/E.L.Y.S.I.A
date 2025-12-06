// ======================================================================
// 📍 FILE: elysia_core/src/module/router.rs
// ======================================================================

use axum::Router;
use crate::module::registry::ModuleRegistry;

pub fn build_module_router(reg: &ModuleRegistry) -> Router {
    let mut router = Router::new();

    for module in reg.iter() {
        router = router.merge(module.routes());
    }

    router
}
