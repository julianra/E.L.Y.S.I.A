// ======================================================================
// 📍 FILE: modules/marthe/src/lib.rs
//
// 📝 Beschrijving:
//   MARTHE: planning, agenda en automatische scheduling.
//   - ontvangt events van de kernel
//   - bouwt AgendaItem structs
//   - vult ontbrekende velden aan (AI stub)
//   - slaat alles op in SQLite via r2d2 pool
// ======================================================================

// ======================================================================
// 📍 FILE: modules/marthe/src/lib.rs
// ======================================================================

use elysia_core::{ElysiaModule, KernelContext, Router, EventBus};
use elysia_core::events::KernelEvent;
use elysia_core::register_module;

mod models;
mod storage;

use models::AgendaItem;

pub struct MartheModule;

impl Default for MartheModule {
    fn default() -> Self {
        MartheModule
    }
}

register_module!(MartheModule);

impl ElysiaModule for MartheModule {
    fn name(&self) -> &'static str {
        "marthe"
    }

    fn init(&self, ctx: &KernelContext) {
        log::info!("[MARTHE] Initialised");

        let conn = ctx.db();
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS marthe_tasks (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                duration_minutes INTEGER,
                exact_start TEXT,
                exact_end TEXT,
                priority TEXT,
                location TEXT,
                energy_cost INTEGER
            );
        "#).expect("[MARTHE] Failed to create table marthe_tasks");
    }


    fn register_routes(&self, router: &mut Router) {
        router.add_route("GET", "/marthe/tasks");
        log::info!("[MARTHE] Registered route /marthe/tasks");
    }

    fn register_event_handlers(&self, bus: &mut EventBus) {
        bus.register_handler("marthe.task_requested");
        log::info!("[MARTHE] Registered handler marthe.task_requested");
    }

    fn handle_event(&self, ctx: &KernelContext, event: KernelEvent) {
        if event.name != "task_requested" {
            return;
        }

        log::info!("[MARTHE] Received task request: {:?}", event.payload);

        match serde_json::from_value::<AgendaItem>(event.payload) {
            Ok(mut item) => {
                item.fill_defaults();
                if let Err(e) = storage::save_task(ctx, &item) {
                    log::error!("[MARTHE] Failed to save task: {}", e);
                } else {
                    log::info!("[MARTHE] Task saved: {}", item.id);
                }
            }
            Err(e) => {
                log::error!("[MARTHE] Invalid task payload: {}", e);
            }
        }
    }

    fn box_clone(&self) -> Box<dyn ElysiaModule> {
        Box::new(Self)
    }
}
