
// ======================================================================
// 📍 FILE: modules/marthe/src/slot_engine.rs
//
// 📝 Beschrijving:
//   Berekent tijdsloten per dag, per minuut nauwkeurig.
//   Output:
//     - FREE blocks
//     - BUSY blocks gelinkt aan bestaande tasks
//
//   Wordt gebruikt door Marthe om:
//     - vrije tijd te vinden
//     - slot in te plannen
//     - conflicten automatisch op te lossen
// ======================================================================

use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone};
use crate::models::AgendaItem;

#[derive(Debug, Clone)]
pub enum BlockType {
    Free,
    Busy(String), // Task ID
}

#[derive(Debug, Clone)]
pub struct TimeBlock {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub kind: BlockType,
}

// Convert task to timeline-friendly data
fn parse_task_times(task: &AgendaItem) -> Option<(DateTime<Local>, DateTime<Local>)> {
    let start = AgendaItem::parse_datetime(task.exact_start.as_ref()?);
    let end = AgendaItem::parse_datetime(task.exact_end.as_ref()?);

    match (start, end) {
        (Some(s), Some(e)) => Some((s, e)),
        _ => None,
    }
}

// ======================================================================
// MAIN FUNCTION: build_day_timeline
// ======================================================================
pub fn build_day_timeline(
    date: &str,
    tasks: &[AgendaItem]
) -> Vec<TimeBlock> {

    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .expect("Invalid date");

    let day_start = Local
        .from_local_datetime(
            &parsed_date.and_hms_opt(0, 0, 0).unwrap()
        )
        .unwrap();

    let day_end = Local
        .from_local_datetime(
            &parsed_date.and_hms_opt(23, 59, 0).unwrap()
        )
        .unwrap();

    // -------------------------------------------------------------
    // 1. Verzamel alle BUSY blocks (tasks)
    // -------------------------------------------------------------
    let mut busy_blocks: Vec<(DateTime<Local>, DateTime<Local>, String)> = vec![];

    for task in tasks {
        if let Some((s, e)) = parse_task_times(task) {
            busy_blocks.push((s, e, task.id.clone()));
        }
    }

    // Sorteer BUSY blocks op begintijd
    busy_blocks.sort_by_key(|(start, _, _)| *start);

    // -------------------------------------------------------------
    // 2. Loop door tasks en maak FREE/BUSY blocks
    // -------------------------------------------------------------
    let mut blocks = Vec::<TimeBlock>::new();
    let mut cursor = day_start;

    for (start, end, id) in busy_blocks {
        // FREE block vóór taak
        if start > cursor {
            blocks.push(TimeBlock {
                start: cursor,
                end: start,
                kind: BlockType::Free,
            });
        }

        // BUSY block
        blocks.push(TimeBlock {
            start,
            end,
            kind: BlockType::Busy(id),
        });

        cursor = end;
    }

    // FREE block NA alle taken tot einde dag
    if cursor < day_end {
        blocks.push(TimeBlock {
            start: cursor,
            end: day_end,
            kind: BlockType::Free,
        });
    }

    blocks
}

// ======================================================================
// FIND FREE WINDOWS (minuut-nauwkeurig)
// ======================================================================
pub fn find_free_windows(blocks: &[TimeBlock]) -> Vec<TimeBlock> {
    blocks.iter()
        .filter(|b| matches!(b.kind, BlockType::Free))
        .cloned()
        .collect()
}

// ======================================================================
// FIND BEST SLOT (first-fit, ultra simpel maar werkt)
// ======================================================================
pub fn find_best_slot_for_minutes(
    blocks: &[TimeBlock],
    required_minutes: i64
) -> Option<DateTime<Local>> {

    for block in blocks {
        if let BlockType::Free = block.kind {
            let free_duration = block.end - block.start;

            if free_duration >= Duration::minutes(required_minutes) {
                return Some(block.start);
            }
        }
    }

    None
}
