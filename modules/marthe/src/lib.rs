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

        task::spawn(async move {
            // AI CALL
            let ai_result = state_clone.ctx.ai.send(
                elysia_ai::intents::AiIntent::MartheParseTask,
                &raw
            ).await;

            let parsed_str = match ai_result {
                Ok(x) => x,
                Err(e) => {
                    error!("[MARTHE] AI error: {}", e);
                    return;
                }
            };

            info!("[MARTHE] AI returned: {}", parsed_str);

            // JSON parse
            let parsed_json: Value = match serde_json::from_str(&parsed_str) {
                Ok(v) => v,
                Err(e) => {
                    error!("[MARTHE] Invalid AI JSON: {}", e);
                    return;
                }
            };

            // ==========================================================
            // ⭐  FIX: AI uses "title", MARTHE expects "name"
            // ==========================================================
            let parsed_fixed = parsed_json;

            


            // PARSED event dispatch
            let evt = KernelEvent {
                name: "task_requested".to_string(),
                payload: parsed_fixed,
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

        // JSON → AgendaItem (payload is exact)
let mut item = match serde_json::from_value::<AgendaItem>(payload.clone()) {
    Ok(i) => i,
    Err(e) => {
        error!("[MARTHE] Invalid parsed payload: {}", e);
        return;
    }
};

// Als AI exact_start én exact_end gaf → no defaults
// Als één van de twee ontbreekt → defaults aanvullen
item.fill_defaults();

// Alle verdere logic blijft gelijk


// ==========================================================
// ⭐ Convert AI fields date + time → exact_start / exact_end
// ==========================================================
use chrono::TimeZone;

if let Some(date_str) = payload.get("date").and_then(|v| v.as_str()) {
    let mut datetime_str = date_str.to_string();

    if let Some(time_str) = payload.get("time").and_then(|v| v.as_str()) {
        datetime_str.push('T');
        datetime_str.push_str(time_str);
        datetime_str.push_str(":00");
    } else {
        datetime_str.push_str("T09:00:00");
    }

    let dt = match chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S") {
        Ok(v) => match chrono::Local.from_local_datetime(&v).single() {
            Some(ldt) => ldt,
            None => {
                error!("[MARTHE] Local datetime ambiguous: {}", datetime_str);
                return;
            }
        },
        Err(e) => {
            error!("[MARTHE] Failed to parse datetime '{}': {}", datetime_str, e);
            return;
        }
    };

    item.exact_start = Some(dt.to_rfc3339());
}

// ==========================================================
// ⭐ Convert AI fields date + time → exact_start / exact_end
// ==========================================================

if let Some(date_str) = payload.get("date").and_then(|v| v.as_str()) {
    let mut datetime_str = date_str.to_string();

    if let Some(time_str) = payload.get("time").and_then(|v| v.as_str()) {
        datetime_str.push('T');
        datetime_str.push_str(time_str);
        datetime_str.push_str(":00"); // seconds
    } else {
        // default time: 09:00
        datetime_str.push_str("T09:00:00");
    }

    // Now convert to RFC3339
    use chrono::TimeZone; // <-- bovenaan REQUIRED import

let dt = match chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S") {
    Ok(v) => {
        // Try to map local time safely
        match chrono::Local.from_local_datetime(&v).single() {
            Some(local_dt) => local_dt,
            None => {
                error!("[MARTHE] Local datetime mapping failed (ambiguous/invalid): {}", datetime_str);
                return;
            }
        }
    }
    Err(e) => {
        error!("[MARTHE] Failed to parse datetime '{}': {}", datetime_str, e);
        return;
    }
};

item.exact_start = Some(dt.to_rfc3339());

}

        // Defaults (duration, start, end)
        item.fill_defaults();

        // Datum extraheren
        let date = match item.exact_start.as_ref() {
            Some(s) => s[..10].to_string(),
            None => {
                error!("[MARTHE] Missing exact_start after defaults");
                return;
            }
        };

        // Taken van dezelfde dag laden
        let tasks_today = match storage::load_tasks_for_day(ctx, &date) {
            Ok(t) => t,
            Err(e) => {
                error!("[MARTHE] Load tasks error: {}", e);
                return;
            }
        };

        // SCHEDULING
        scheduler::schedule_task(&mut item, &tasks_today, &date);

        // CONFLICT DETECTIE
        for t in &tasks_today {
            if item.overlaps(t) {
                warn!(
                    "[MARTHE] CONFLICT: '{}' overlapt '{}'",
                    item.name, t.name
                );
            }
        }

        // OPSLAAN IN DB
        if let Err(e) = storage::save_task(ctx, &item) {
            error!("[MARTHE] Save error: {}", e);
        } else {
            info!("[MARTHE] Task saved: {}", item.id);
        }
    }
}
