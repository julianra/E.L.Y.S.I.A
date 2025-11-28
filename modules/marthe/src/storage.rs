// ======================================================================
// 📍 FILE: modules/marthe/src/storage.rs
//
// 📝 Beschrijving:
//   Opslaglaag voor MARTHE-taken in SQLite.
// ======================================================================

use crate::models::AgendaItem;
use elysia_core::KernelContext;
use rusqlite::Row;

// -------------------------------------------------------------
// SAVE TASK
// -------------------------------------------------------------
pub fn save_task(ctx: &KernelContext, item: &AgendaItem) -> Result<(), String> {
    let conn = ctx.db();

    conn.execute(
        "INSERT INTO marthe_tasks (
            id,
            name,
            duration_minutes,
            exact_start,
            exact_end,
            priority,
            location,
            energy_cost,
            deadline_end
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &item.id,
            &item.name,
            &item.duration_minutes,
            &item.exact_start,
            &item.exact_end,
            &item.priority,
            &item.location,
            &item.energy_cost,
            &item.deadline_end,
        ),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// -------------------------------------------------------------
// HELPER: Row → AgendaItem
// -------------------------------------------------------------
fn row_to_item(row: &Row) -> Result<AgendaItem, rusqlite::Error> {
    Ok(AgendaItem {
        id: row.get(0)?,
        name: row.get(1)?,
        duration_minutes: row.get(2)?,
        exact_start: row.get(3)?,
        exact_end: row.get(4)?,
        priority: row.get(5)?,
        location: row.get(6)?,
        energy_cost: row.get(7)?,
        deadline_end: row.get(8)?,
    })
}

// -------------------------------------------------------------
// LOAD TASKS FOR DAY
// -------------------------------------------------------------
pub fn load_tasks_for_day(ctx: &KernelContext, date: &str) -> Result<Vec<AgendaItem>, String> {
    let conn = ctx.db();

    let mut stmt = conn
        .prepare(
            "SELECT
                id,
                name,
                duration_minutes,
                exact_start,
                exact_end,
                priority,
                location,
                energy_cost,
                deadline_end
             FROM marthe_tasks
             WHERE date(exact_start) = date(?1)
             ORDER BY exact_start ASC, created_at ASC"
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([date], |row| row_to_item(row))
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();

    for row_result in rows {
        match row_result {
            Ok(item) => items.push(item),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(items)
}
