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

use chrono::{DateTime, Duration, Local,Datelike, Timelike, TimeZone};
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

    let deadline = match parse_dt(deadline_str) {
        Some(d) => d,
        None => return,
    };

    let timeline = build_day_timeline(date, tasks_today);
    let free = find_free_windows(&timeline);

    let latest_start = deadline - Duration::minutes(duration);

    // 1) Zoek ideale slot (zoals origineel)
    for block in free {
        let free_start = block.start;
        let free_end = block.end;

        if latest_start >= free_start
            && latest_start + Duration::minutes(duration) <= free_end
        {
            item.exact_start = Some(to_rfc(latest_start));
            item.exact_end = Some(to_rfc(latest_start + Duration::minutes(duration)));
            return;
        }
    }

    // ⭐ 2) GEEN plek? → start gewoon op latest_start (HARD DEADLINE MODE)
    item.exact_start = Some(to_rfc(latest_start));
    item.exact_end = Some(to_rfc(deadline));

    // conflicts are OK → scheduler will flag them
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

pub fn expand_range_if_needed(item: &AgendaItem) -> Vec<AgendaItem> {
    // Alleen expanden als start en end bestaan
    let Some(start_str) = &item.exact_start else { return vec![item.clone()]; };
    let Some(end_str)   = &item.exact_end   else { return vec![item.clone()]; };

    let Some(start) = AgendaItem::parse_datetime(start_str) else { return vec![item.clone()]; };
    let Some(end)   = AgendaItem::parse_datetime(end_str)   else { return vec![item.clone()]; };

    // Minder dan 12 uren? → geen multi-day
    if end - start < Duration::hours(12) {
        return vec![item.clone()];
    }

    let mut current = start.date_naive();
    let final_date  = end.date_naive();

    let start_t = start.time();
    let end_t   = end.time();

    let mut results = Vec::new();

    while current <= final_date {
        let weekday = current.weekday().number_from_monday(); // ma=1..zo=7

        // Alleen weekdagen
        if weekday <= 5 {
            // Bouw NaiveDateTime voor deze dag
            let start_naive = current
                .and_hms_opt(start_t.hour(), start_t.minute(), start_t.second())
                .unwrap();

            let end_naive = current
                .and_hms_opt(end_t.hour(), end_t.minute(), end_t.second())
                .unwrap();

            // ⭐ ECHTE FIX: NaiveDateTime naar LOCAL ZONDER UTC HACK
            let start_dt = Local.from_local_datetime(&start_naive).single().unwrap();
            let end_dt   = Local.from_local_datetime(&end_naive).single().unwrap();

            let mut sub = item.clone();
            sub.id = AgendaItem::default_id();
            sub.exact_start = Some(start_dt.to_rfc3339());
            sub.exact_end   = Some(end_dt.to_rfc3339());

            results.push(sub);
        }

        current = current.succ_opt().unwrap();
    }

    results
}
