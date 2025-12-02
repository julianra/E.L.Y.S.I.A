// ======================================================================
// 📍 FILE: modules/marthe/src/lib.rs
// ======================================================================
//
//  ⭐ MARTHE v2.0 — AI-enabled planning
//
//  ✔ RAW text → AI → parsed event
//  ✔ KernelState (i.p.v. KernelContext)
//  ✔ tokio::spawn async AI-call
//  ✔ title → name fix
//  ✔ automatic scheduling + conflict detection
//  ✔ automatic database storage
//  ✔ correct EventBus key matching
//
// ======================================================================

use elysia_core::{ElysiaModule, Router, EventBus};
use elysia_core::kernel::KernelState;
use elysia_core::events::KernelEvent;
use elysia_core::register_module;

use serde_json::Value;
use log::{error, info, warn};
use tokio::task;
use chrono::TimeZone;


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
    fn init(&self, ctx: &elysia_core::KernelContext) {
        info!("[MARTHE] Initialised");

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
    // ROUTES
    // ------------------------------------------------------------------
    fn register_routes(&self, router: &mut Router) {
        router.add_route("GET", "/marthe/tasks");
        info!("[MARTHE] Registered route /marthe/tasks");
    }

    // ------------------------------------------------------------------
    // EVENTS
    // ------------------------------------------------------------------
    fn register_event_handlers(&self, bus: &mut EventBus) {
        // Registratie EXACT zoals EventBus verwacht:
        // module.name() + "." + event_name
        // = "marthe.task_requested_raw"
        // = "marthe.task_requested"
        bus.register_handler("marthe.task_requested_raw");
        bus.register_handler("marthe.task_requested");

        info!("[MARTHE] Registered raw & parsed handlers");
    }

    // ------------------------------------------------------------------
    // HANDLE EVENTS (KernelState!)
    // ------------------------------------------------------------------
    fn handle_event(&self, state: &KernelState, event: KernelEvent) {
        match event.name.as_str() {
            "task_requested_raw" => self.handle_raw_task(state, event.payload.clone()),
            "task_requested" => self.handle_parsed_task(state, event.payload.clone()),
            _ => {}
        }
    }

    fn box_clone(&self) -> Box<dyn ElysiaModule> {
        Box::new(Self)
    }
}
// ============================================================
// ⭐ FLEXIBELE DATETIME PARSER
// Accepteert:
//   - 2025-12-04T08:00:00
//   - 2025-12-04T08:00:00.000
//   - elke Local-tijd, geconverteerd naar RFC3339
// ============================================================
fn parse_datetime_any(s: &str) -> Option<chrono::DateTime<chrono::Local>> {
    // Formaat 1: zonder milliseconden
    if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        if let Some(dt) = chrono::Local.from_local_datetime(&ndt).single() {
            return Some(dt);
        }
    }

    // Formaat 2: met milliseconden
    if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.3f") {
        if let Some(dt) = chrono::Local.from_local_datetime(&ndt).single() {
            return Some(dt);
        }
    }

    None
}

// ======================================================================
// RAW → AI → PARSED
// ======================================================================
impl MartheModule {
  fn handle_raw_task(&self, state: &KernelState, payload: Value) {
    let raw_opt = payload.get("name").and_then(|v| v.as_str());

    if raw_opt.is_none() {
        error!("[MARTHE] RAW zonder 'name'");
        return;
    }

    let raw = raw_opt.unwrap().to_string();
    info!("[MARTHE] RAW → AI: '{}'", raw);

    let state_clone = state.clone();
    let payload_clone = payload.clone();

    task::spawn(async move {
        // -------------------------------
        // USER OVERRIDES AI
        // -------------------------------
        let user_provided_timing =
            payload_clone.get("exact_start").is_some() ||
            payload_clone.get("exact_end").is_some() ||
            payload_clone.get("deadline_end").is_some() ||
            payload_clone.get("duration_minutes").is_some();

        let parsed_json = if user_provided_timing {
            info!("[MARTHE] User gave timing → skipping AI");
            payload_clone
        } else {
            // -------------------------------
            // AI PARSING
            // -------------------------------
            let ai_result = state_clone.ctx.ai.send(
                elysia_ai::intents::AiIntent::MartheParseTask,
                &raw
            ).await;

            let ai_str = match ai_result {
                Ok(v) => v,
                Err(e) => {
                    error!("[MARTHE] AI error: {}", e);
                    return;
                }
            };

            info!("[MARTHE] AI returned: {}", ai_str);

            match serde_json::from_str::<Value>(&ai_str) {
                Ok(v) => v,
                Err(e) => {
                    error!("[MARTHE] Failed to parse AI JSON: {}", e);
                    return;
                }
            }
        };

        // -------------------------------
        // DISPATCH TO PARSED HANDLER
        // -------------------------------
        let evt = KernelEvent {
            name: "task_requested".to_string(),
            payload: parsed_json,
        };

        state_clone.bus.dispatch(evt, &state_clone.modules, &state_clone);
    });
}

    // ==================================================================
    // PARSED EVENT → agenda logic
    // ==================================================================
    fn handle_parsed_task(&self, state: &KernelState, payload: Value) {
    info!("[MARTHE] Parsed task: {:?}", payload);

    let ctx = &state.ctx;

    // ==========================================================
    // 1. JSON → AgendaItem
    // ==========================================================
    let mut item = match serde_json::from_value::<AgendaItem>(payload.clone()) {
        Ok(i) => i,
        Err(e) => {
            error!("[MARTHE] Invalid parsed payload: {}", e);
            return;
        }
    };

    // ==========================================================
    // 2. User-Supplied Datetimes → parse using flexible parser
    // ==========================================================
    let user_has_timing =
        payload.get("exact_start").is_some() ||
        payload.get("exact_end").is_some() ||
        payload.get("deadline_end").is_some() ||
        payload.get("duration_minutes").is_some();

    // exact_start
    if let Some(start_str) = payload.get("exact_start").and_then(|v| v.as_str()) {
        match parse_datetime_any(start_str) {
            Some(dt) => item.exact_start = Some(dt.to_rfc3339()),
            None => error!("[MARTHE] Failed to parse datetime '{}'", start_str),
        }
    }

    // exact_end
    if let Some(end_str) = payload.get("exact_end").and_then(|v| v.as_str()) {
        match parse_datetime_any(end_str) {
            Some(dt) => item.exact_end = Some(dt.to_rfc3339()),
            None => error!("[MARTHE] Failed to parse datetime '{}'", end_str),
        }
    }

    // ==========================================================
    // 3. Defaults invullen (indien niet gezet)
    // ==========================================================
    item.fill_defaults();

    // ==========================================================
    // 4. User input? → GEEN SCHEDULING, GEEN CONFLICT DETECTION
    // ==========================================================
    if user_has_timing {
        info!("[MARTHE] User provided timing → skipping scheduling + conflict detection");

        if let Err(e) = storage::save_task(ctx, &item) {
            error!("[MARTHE] Save error: {}", e);
        } else {
            info!("[MARTHE] Task saved: {}", item.id);
        }
        return;
    }

    // ==========================================================
    // 5. AI-generated → Scheduling nodig
    // ==========================================================
    let date = match item.exact_start.as_ref() {
        Some(s) => s[..10].to_string(),
        None => {
            error!("[MARTHE] Missing exact_start after defaults");
            return;
        }
    };

    // Load tasks for same day
    let tasks_today = match storage::load_tasks_for_day(ctx, &date) {
        Ok(t) => t,
        Err(e) => {
            error!("[MARTHE] Load tasks error: {}", e);
            return;
        }
    };

    // Scheduling (only for AI tasks)
    scheduler::schedule_task(&mut item, &tasks_today, &date);

    // Conflict warnings
    for t in &tasks_today {
        if item.overlaps(t) {
            warn!(
                "[MARTHE] CONFLICT: '{}' overlapt '{}'",
                item.name, t.name
            );
        }
    }

    // ==========================================================
    // 6. Save
    // ==========================================================
    if let Err(e) = storage::save_task(ctx, &item) {
        error!("[MARTHE] Save error: {}", e);
    } else {
        info!("[MARTHE] Task saved: {}", item.id);
    }
}

}
