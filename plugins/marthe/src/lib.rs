use axum::{Router, routing::get};

use elysia_core::{
    ElysiaModule,
    module::registry::ModuleRegistry,
    kernel::KernelState,
    events::KernelEvent,
};

// ---------------------------------------------------------------
//  FIX: Forceer Windows om iets te exporteren
// ---------------------------------------------------------------
#[no_mangle]
pub static ELYSIA_PLUGIN: u8 = 0;

// ---------------------------------------------------------------
//  MARTHE MODULE STRUCT
// ---------------------------------------------------------------
pub struct MartheModule;

impl ElysiaModule for MartheModule {
    fn name(&self) -> &'static str {
        "marthe"
    }

    fn routes(&self) -> Router {
        Router::new().route("/marthe/hello", get(|| async {
            "Hello from MARTHE!"
        }))
    }

    fn handle_event(&self, _state: &KernelState, _event: &KernelEvent) {
        // Niets voor hello world
    }
}

// ---------------------------------------------------------------
//  ENTRYPOINT DIE CORE LAADT
// ---------------------------------------------------------------
#[no_mangle]
pub extern "C" fn elysia_register(registry: &mut ModuleRegistry) {
    println!("[PLUGIN][MARTHE] Registering module…");
    registry.register_module(Box::new(MartheModule));
}
