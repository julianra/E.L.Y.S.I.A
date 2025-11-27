// ======================================================================
// 📍 FILE: modules/marthe/src/storage.rs
// ======================================================================

use crate::models::AgendaItem;
use elysia_core::KernelContext;

pub fn save_task(ctx: &KernelContext, item: &AgendaItem) -> Result<(), String> {
    let conn = ctx.db();

    conn.execute(
        "INSERT INTO marthe_tasks (
            id, name, duration_minutes, exact_start, exact_end,
            priority, location, energy_cost
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            &item.id,
            &item.name,
            &item.duration_minutes,
            &item.exact_start,
            &item.exact_end,
            &item.priority,
            &item.location,
            &item.energy_cost,
        ),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
