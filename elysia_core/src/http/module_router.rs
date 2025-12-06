// ======================================================================
// 📍 FILE: elysia_core/src/http/module_router.rs
// ======================================================================

use axum::{Router, routing::get};
use crate::kernel::KernelState;

pub fn mount_module_routes(state: &KernelState) -> Router {
    let mut router = Router::new();

    for module in state.modules.iter() {
        let name = module.name();

        let r = Router::new()
            .route("/info", get({
                let name = name.to_string();
                move || async move { format!("Module '{}' is online", name) }
            }))
            .with_state(state.clone());

        router = router.nest(&format!("/api/{}", name), r);
    }

    router
}
