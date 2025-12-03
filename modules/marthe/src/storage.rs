// ======================================================================
// 📍 FILE: modules/marthe/src/storage.rs
//
// 📝 Beschrijving:
//   Opslaglaag voor MARTHE-taken in SQLite.
// ======================================================================

use crate::models::AgendaItem;
use elysia_core::KernelContext;
use rusqlite::Row;
use crate::scheduler::expand_range_if_needed;


// -------------------------------------------------------------
// SAVE TASK
// -------------------------------------------------------------
pub fn save_task(ctx: &KernelContext, item: &AgendaItem) -> Result<(), String> {
    use crate::scheduler::expand_range_if_needed;

    let conn = ctx.db();

    // ⭐ BELANGRIJK: eerst range-expansion uitvoeren
    let expanded = expand_range_if_needed(item);

    // daarna in loop opslaan:
    for mut task in expanded {
        // 2) Normaliseer timestamps (offset fix)
        if let Some(start) = &task.exact_start {
            if let Some(dt) = AgendaItem::parse_datetime(start) {
                task.exact_start = Some(dt.to_rfc3339());
            }
        }

        if let Some(end) = &task.exact_end {
            if let Some(dt) = AgendaItem::parse_datetime(end) {
                task.exact_end = Some(dt.to_rfc3339());
            }
        }

        if let Some(deadline) = &task.deadline_end {
            if let Some(dt) = AgendaItem::parse_datetime(deadline) {
                task.deadline_end = Some(dt.to_rfc3339());
            }
        }

        // 3) Opslaan
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
                &task.id,
                &task.name,
                &task.duration_minutes,
                &task.exact_start,
                &task.exact_end,
                &task.priority,
                &task.location,
                &task.energy_cost,
                &task.deadline_end,
            ),
        )
        .map_err(|e| e.to_string())?;
    }

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
            Ok(mut item) => {
                // -----------------------------------------------------
                // NORMALIZE LOADED TIMESTAMPS (important for Orbit)
                // -----------------------------------------------------
                if let Some(start) = &item.exact_start {
                    if let Some(dt) = AgendaItem::parse_datetime(start) {
                        item.exact_start = Some(dt.to_rfc3339());
                    }
                }

                if let Some(end) = &item.exact_end {
                    if let Some(dt) = AgendaItem::parse_datetime(end) {
                        item.exact_end = Some(dt.to_rfc3339());
                    }
                }

                if let Some(deadline) = &item.deadline_end {
                    if let Some(dt) = AgendaItem::parse_datetime(deadline) {
                        item.deadline_end = Some(dt.to_rfc3339());
                    }
                }

                items.push(item)
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(items)
}
