// ======================================================================
// 📍 FILE: elysia/modules/marthe/src/lib.rs
//
// 📝 BESCHRIJVING:
//   MARTHE is de planning-, agenda- en schedulingmodule van ELYSIA.
//   Deze crate registreert zichzelf automatisch via `inventory`.
//
// 🔧 TAKEN:
//   - Zichzelf registreren via register_module!
//   - Lifecycle implementeren (init, routes, events, background tasks)
//   - Later: agenda-invoer verwerken, conflictdetectie, AI-planning
// ======================================================================

use elysia_core::{ElysiaModule, KernelContext, Router, EventBus};
use elysia_core::register_module;

// De eigenlijke module-struct
pub struct MartheModule;

// Module moet Default zijn omdat `inventory` hem moet kunnen creëren
impl Default for MartheModule {
    fn default() -> Self {
        MartheModule
    }
}

// Automatische registratie
register_module!(MartheModule);

impl ElysiaModule for MartheModule {
    fn name(&self) -> &'static str { "marthe" }

    fn init(&self, ctx: &mut KernelContext) {
        ctx.set_meta("marthe_version", "0.1.0");
        log::info!("[MARTHE] MartheModule init done.");
    }

    fn register_routes(&self, router: &mut Router) {
        router.add_route("POST", "/marthe/add_task");
        log::info!("[MARTHE] Registered /marthe/add_task");
    }

    fn register_event_handlers(&self, bus: &mut EventBus) {
        bus.register_handler("marthe.task_requested");
        log::info!("[MARTHE] Registered handler for marthe.task_requested");
    }

    fn start_background_tasks(&self, _ctx: &KernelContext) {
        log::info!("[MARTHE] Background tasks started.");
    }
}
