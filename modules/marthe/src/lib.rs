// ======================================================================
// 📍 FILE: modules/marthe/src/lib.rs
//
// 📝 Beschrijving:
//   MARTHE: planning, agenda en automatische scheduling.
//   - ontvangt events
//   - bouwt AgendaItem structs
//   - vult ontbrekende velden aan
//   - plant taken automatisch in via scheduler
//   - detecteert conflicten
//   - slaat alles op in SQLite via r2d2 pool
// ======================================================================

use elysia_core::{ElysiaModule, KernelContext, Router, EventBus};
use elysia_core::events::KernelEvent;
use elysia_core::register_module;

mod models;
mod storage;
mod slot_engine;
mod scheduler;

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

    // ------------------------------------------------------------------
    // INIT
    // ------------------------------------------------------------------
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
                energy_cost INTEGER,
                deadline_end TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
        "#).expect("[MARTHE] Failed to create table marthe_tasks");
    }

    // ------------------------------------------------------------------
    // REGISTER ROUTES
    // ------------------------------------------------------------------
    fn register_routes(&self, router: &mut Router) {
        router.add_route("GET", "/marthe/tasks");
        log::info!("[MARTHE] Registered route /marthe/tasks");
    }

    // ------------------------------------------------------------------
    // REGISTER EVENT HANDLERS
    // ------------------------------------------------------------------
    fn register_event_handlers(&self, bus: &mut EventBus) {
        bus.register_handler("marthe.task_requested");
        log::info!("[MARTHE] Registered handler marthe.task_requested");
    }

    // ------------------------------------------------------------------
    // HANDLE EVENTS  (volledige planner)
    // ------------------------------------------------------------------
    fn handle_event(&self, ctx: &KernelContext, event: KernelEvent) {
        if event.name != "task_requested" {
            return;
        }

        log::info!("[MARTHE] Received task request: {:?}", event.payload);

        // 1. JSON → AgendaItem
        let mut item = match serde_json::from_value::<AgendaItem>(event.payload) {
            Ok(i) => i,
            Err(e) => {
                log::error!("[MARTHE] Invalid task payload: {}", e);
                return;
            }
        };

        // 2. Defaults aanvullen
        item.fill_defaults();

        // 3. Datum extraheren (YYYY-MM-DD)
        let date = item.exact_start
            .as_ref()
            .unwrap()[..10]
            .to_string();

        // 4. Bestaande taken voor dezelfde dag ophalen
        let day_tasks = match storage::load_tasks_for_day(ctx, &date) {
            Ok(t) => t,
            Err(e) => {
                log::error!("[MARTHE] Failed to load tasks for day: {}", e);
                return;
            }
        };

        // --------------------------------------------------------------
        // 5. PLANNING LOGICA: automatisch inplannen
        // --------------------------------------------------------------
        scheduler::schedule_task(&mut item, &day_tasks, &date);

        // --------------------------------------------------------------
        // 6. Conflict detectie (na scheduling)
        // --------------------------------------------------------------
        for t in &day_tasks {
            if item.overlaps(t) {
                log::warn!(
                    "[MARTHE] CONFLICT: Task '{}' overlapt met '{}'",
                    item.name, t.name
                );
            }
        }

        // --------------------------------------------------------------
        // 7. Opslaan
        // --------------------------------------------------------------
        if let Err(e) = storage::save_task(ctx, &item) {
            log::error!("[MARTHE] Failed to save task: {}", e);
        } else {
            log::info!("[MARTHE] Task saved: {}", item.id);
        }
    }

    fn box_clone(&self) -> Box<dyn ElysiaModule> {
        Box::new(Self)
    }
}
