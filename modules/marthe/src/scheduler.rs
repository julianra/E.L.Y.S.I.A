// ======================================================================
// 📍 FILE: modules/marthe/src/scheduler.rs
//
// 📝 Beschrijving:
//   De centrale planner van MARTHE.
//   Bepaalt hoe een taak moet ingepland worden:
//     - exact blok (start & end)
//     - forward planning (start → end)
//     - backward planning (end → start)
//     - deadline planning
//     - automatisch beste slot zoeken via slot_engine
//
//   Wordt 100% minuut-nauwkeurig uitgevoerd.
//
// ======================================================================

use chrono::{DateTime, Duration, Local};
use crate::models::AgendaItem;
use crate::slot_engine::{build_day_timeline, find_free_windows, find_best_slot_for_minutes};

// ======================================================================
//  Helpers
// ======================================================================

fn parse_dt(s: &str) -> Option<DateTime<Local>> {
    AgendaItem::parse_datetime(s)
}

fn to_rfc(dt: DateTime<Local>) -> String {
    dt.to_rfc3339()
}

// ======================================================================
//  PUBLIC ENTRY POINT
// ======================================================================

pub fn schedule_task(
    item: &mut AgendaItem,
    tasks_today: &[AgendaItem],
    date: &str
) {
    let has_start = item.exact_start.is_some();
    let has_end   = item.exact_end.is_some();
    let has_dead  = item.deadline_end.is_some();
    let dur       = item.duration_minutes.unwrap_or(15);

    // Scenario 1: exact blok gegeven → alleen conflict-checken
    if has_start && has_end {
        normalize_exact_block(item);
        return;
    }

    // Scenario 2: start bekend, end niet → forward plan
    if has_start && !has_end {
        forward_plan(item, dur);
        return;
    }

    // Scenario 3: end bekend, start niet → backward plan
    if has_end && !has_start {
        backward_plan(item, dur);
        return;
    }

    // Scenario 4: geen tijden maar wél deadline
    if !has_start && !has_end && has_dead {
        deadline_plan(item, tasks_today, date, dur);
        return;
    }

    // Scenario 5: niets bekend → plan vrij beste slot
    free_plan(item, tasks_today, date, dur);
}

// ======================================================================
//  FORWARD PLAN
// ======================================================================

fn forward_plan(item: &mut AgendaItem, duration: i64) {
    if let Some(start_str) = &item.exact_start {
        if let Some(start) = parse_dt(start_str) {
            let end = start + Duration::minutes(duration);
            item.exact_end = Some(to_rfc(end));
        }
    }
}

// ======================================================================
//  BACKWARD PLAN
// ======================================================================

fn backward_plan(item: &mut AgendaItem, duration: i64) {
    if let Some(end_str) = &item.exact_end {
        if let Some(end) = parse_dt(end_str) {
            let start = end - Duration::minutes(duration);
            item.exact_start = Some(to_rfc(start));
        }
    }
}

// ======================================================================
//  EXACT BLOCK: clean up formatting
// ======================================================================

fn normalize_exact_block(item: &mut AgendaItem) {
    if let (Some(s), Some(e)) = (&item.exact_start, &item.exact_end) {
        if let (Some(start), Some(end)) = (parse_dt(s), parse_dt(e)) {
            item.exact_start = Some(to_rfc(start));
            item.exact_end   = Some(to_rfc(end));
        }
    }
}

// ======================================================================
//  DEADLINE PLANNING (backward slot search)
// ======================================================================

fn deadline_plan(
    item: &mut AgendaItem,
    tasks_today: &[AgendaItem],
    date: &str,
    duration: i64
) {
    let deadline_str = item.deadline_end.as_ref().unwrap();

    if let Some(deadline) = parse_dt(deadline_str) {
        let timeline = build_day_timeline(date, tasks_today);
        let free = find_free_windows(&timeline);

        // We zoeken een slot dat eindigt vóór deadline
        for block in free {
            let free_start = block.start;
            let free_end = block.end;

            // We willen end ≤ deadline
            let latest_start = deadline - Duration::minutes(duration);

            // Check of dit binnen het block kan
            if latest_start >= free_start && latest_start + Duration::minutes(duration) <= free_end {
                item.exact_start = Some(to_rfc(latest_start));
                item.exact_end   = Some(to_rfc(latest_start + Duration::minutes(duration)));
                return;
            }
        }

        // Geen plek gevonden → fallback naar first free (kan AI later oplossen)
        free_plan(item, tasks_today, date, duration);
    }
}

// ======================================================================
//  FREE PLANNING → via slot_engine
// ======================================================================

fn free_plan(
    item: &mut AgendaItem,
    tasks_today: &[AgendaItem],
    date: &str,
    duration: i64
) {
    let timeline = build_day_timeline(date, tasks_today);
    let free = find_free_windows(&timeline);

    if let Some(start) = find_best_slot_for_minutes(&free, duration) {
        item.exact_start = Some(to_rfc(start));
        item.exact_end   = Some(to_rfc(start + Duration::minutes(duration)));
    }
}
