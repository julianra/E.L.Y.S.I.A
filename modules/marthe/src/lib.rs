// ======================================================================
// 📍 FILE: elysia/modules/marthe/src/lib.rs
//
// 📝 BESCHRIJVING:
//   MARTHE is de planning-, agenda- en schedulingmodule van ELYSIA.
//   Registreert zichzelf automatisch via `register_module!` macro.
//
// 🔧 TAKEN:
//   - Automatische module-registratie
//   - Agenda-logica (later)
//   - EventBus V2 handlers registreren
// ======================================================================

use elysia_core::{ElysiaModule, KernelContext, Router, EventBus, EventType};
use elysia_core::register_module;

pub struct MartheModule;

impl Default for MartheModule {
    fn default() -> Self {
        MartheModule
    }
}

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

    // ⭐ Nieuwe EventBus API (async subscribe)
    fn register_event_handlers(&self, bus: &mut EventBus) {
        use EventType::*;

        // Luistert naar "marthe.task_requested"
        bus.subscribe("marthe.task_requested", |event| async move {
            log::info!("[MARTHE] Event ontvangen: {:?}", event);

            if let MartheTaskAdded(payload) = event {
                log::info!("[MARTHE] Nieuwe taak: {} = {}", payload.id, payload.name);
            }
        });

        log::info!("[MARTHE] Event handler geregistreerd voor marthe.task_requested");
    }

    fn start_background_tasks(&self, _ctx: &KernelContext) {
        log::info!("[MARTHE] Background tasks started.");
    }
}
