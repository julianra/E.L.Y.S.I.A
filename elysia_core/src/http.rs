// elysia/elysia_core/src/http.rs

use axum::{Router, routing::{get, post}, Json};
use crate::kernel::KernelState;
use crate::events::KernelEvent;
use serde_json::Value;

pub fn build_router(state: KernelState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/agenda/add", post({
    let state = state.clone();
    move |Json(payload): Json<Value>| {
        let state = state.clone();
        async move {
            let event = KernelEvent {
                name: "task_requested_raw".to_string(),
                payload,
            };

            state.bus.dispatch(
                event,
                &state.modules,
                &state
            );

            "OK"
        }
    }

    
}))

        .route("/marthe/tasks", get({
    let state = state.clone();
    move || {
        let state = state.clone();
        async move {
            let conn = state.ctx.db();

            let mut stmt = conn.prepare(
                "SELECT id, name, duration_minutes, exact_start, exact_end,
                        priority, location, energy_cost
                 FROM marthe_tasks"
            ).unwrap();

            let rows = stmt
                .query_map([], |row| {
                    Ok(serde_json::json!({
                        "id": row.get::<_, String>(0)?,
                        "name": row.get::<_, String>(1)?,
                        "duration_minutes": row.get::<_, Option<i64>>(2)?,
                        "exact_start": row.get::<_, Option<String>>(3)?,
                        "exact_end": row.get::<_, Option<String>>(4)?,
                        "priority": row.get::<_, Option<String>>(5)?,
                        "location": row.get::<_, Option<String>>(6)?,
                        "energy_cost": row.get::<_, Option<i64>>(7)?
                    }))
                })
                .unwrap()
                .map(|r| r.unwrap())
                .collect::<Vec<_>>();

            Json(rows)
        }
    }
}))
        // ============================================================
        // DELETE TASK (MARTHE)
        // ============================================================
        .route("/marthe/task/:id", axum::routing::delete({
            let state = state.clone();
            move |axum::extract::Path(id): axum::extract::Path<String>| {
                let state = state.clone();
                async move {
                    let conn = state.ctx.db();
                    let res = conn.execute(
                        "DELETE FROM marthe_tasks WHERE id = ?",
                        &[&id]
                    );

                    match res {
                        Ok(rows) => {
                            Json(serde_json::json!({
                                "status": "ok",
                                "deleted": rows
                            }))
                        }
                        Err(e) => {
                            Json(serde_json::json!({
                                "status": "error",
                                "message": e.to_string()
                            }))
                        }
                    }
                }
            }
        }))

       
    }
