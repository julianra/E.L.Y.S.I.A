// ===============================================
// FILE: src/modules/marthe/marthe_core.rs
// ROLE: MARTHE Module Core
// ===============================================

use tokio::sync::mpsc::Receiver;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use rusqlite::params;
use chrono::{Duration, NaiveDate, NaiveTime};
use crate::kernel::events::Event;
use crate::kernel::agenda_point::AgendaPoint;
fn find_calendar_day(conn: &Connection, date: NaiveDate) -> Option<String> {
    let sql = "SELECT id FROM calendar_day WHERE date = ?1 LIMIT 1";

    conn.query_row(sql, params![date.to_string()], |row| row.get(0)).ok()
}

fn create_calendar_day(conn: &Connection, date: NaiveDate) -> String {
    let id = uuid::Uuid::new_v4().to_string();

    let sql = "
        INSERT INTO calendar_day (id, date, total_tasks, total_duration, energy_load)
        VALUES (?1, ?2, 0, 0, 0)
    ";

    conn.execute(sql, params![&id, date.to_string()])
        .expect("[MARTHE] Kon calendar_day niet aanmaken");

    id
}

fn get_or_create_day(conn: &Connection, date: NaiveDate) -> String {
    if let Some(existing_id) = find_calendar_day(conn, date) {
        return existing_id;
    }
    create_calendar_day(conn, date)
}

pub struct MartheCore {
    receiver: Receiver<Event>,
    db: Arc<Mutex<Connection>>,
}

impl MartheCore {
    pub fn new(receiver: Receiver<Event>, db: Arc<Mutex<Connection>>) -> Self {
        println!("[MARTHE] Module wordt opgestart…");
        Self { receiver, db }
    }

    pub async fn run(&mut self) {
        println!("[MARTHE] Module gestart.");

        while let Some(event) = self.receiver.recv().await {
            match event {
                Event::ExternalAgendaAdd(req) => {
                    println!("[MARTHE] Nieuw extern agendapunt: {}", req.name);

                    // Stap 1: datum parsen
                    let parsed_date = req.parsed_date();

                    // Stap 1B: dag-id bepalen
                    let day_id = {
                        let conn = self.db.lock().unwrap();

                        match parsed_date {
                            Some(date) => get_or_create_day(&conn, date),
                            None => "undated".into(),
                        }
                    };

                    // Taak maken
                    let mut point = AgendaPoint::from_external(req);
                    point.calendar_day_id = Some(day_id.clone());

                    // ⭐ EINDELIJK: PLANNEN
                    if day_id != "undated" {
                        self.schedule_task_for_day(&mut point);
                    }

                    // Opslaan
                    self.save_to_db(&point);


                    println!("[MARTHE] Opgeslagen {} op dag {}", point.id, day_id);


                    // 🔍 NIEUW: debug – hoeveel taken zitten nu in deze dag?
                    if day_id != "undated" {
                        let tasks = self.load_tasks_for_day(&day_id);
                        println!("[MARTHE] Dag {} bevat nu {} taken", day_id, tasks.len());
                    }
                }

                _ => {
                    println!("[MARTHE] Event ontvangen.");
                }
            }
        }
    }

fn save_to_db(&self, point: &AgendaPoint) {
    let conn = self.db.lock().unwrap();

    conn.execute(
        "INSERT INTO agenda (
            id,
            name,
            duration_minutes,
            created_at,
            calendar_day_id,
            start_time,
            end_time
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &point.id,
            &point.name,
            point.duration_minutes,
            point.created_at.to_rfc3339(),
            &point.calendar_day_id,
            &point.start_time,
            &point.end_time,
        ),
    )
    .expect("[MARTHE] Kon agendapunt niet opslaan");
}

    /// Geef alle taken voor een bepaalde dag terug, gesorteerd op start_time (later)
    fn load_tasks_for_day(&self, day_id: &str) -> Vec<(String, String)> {
        let conn = self.db.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT id, name
                 FROM agenda
                 WHERE calendar_day_id = ?1
                 ORDER BY created_at ASC",
            )
            .expect("[MARTHE] Kon query voor dag-taken niet voorbereiden");

        let rows = stmt
            .query_map([day_id], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                Ok((id, name))
            })
            .expect("[MARTHE] Kon dag-taken niet mappen");

        let mut result = Vec::new();
        for r in rows {
            if let Ok(entry) = r {
                result.push(entry);
            }
        }

        result
    }

// ---------------------------
// PLAN TAak op die dag
// ---------------------------
fn schedule_task_for_day(&self, point: &mut AgendaPoint) {
    let day_id = match &point.calendar_day_id {
        Some(id) => id.clone(),
        None => {
            println!("[MARTHE] Geen dag-id, skipping scheduling.");
            return;
        }
    };

    let conn = self.db.lock().unwrap();

    // 1. Haal alle taken voor deze dag op, met start_time
    let mut stmt = conn.prepare(
        "SELECT start_time, end_time, duration_minutes 
         FROM agenda 
         WHERE calendar_day_id = ?1 
         ORDER BY start_time ASC"
    ).expect("[MARTHE] Kon dagtaken niet ophalen");

    let rows = stmt.query_map([&day_id], |row| {
        let start: Option<String> = row.get(0)?;
        let end: Option<String> = row.get(1)?;
        let duration: i64 = row.get(2)?;
        Ok((start, end, duration))
    }).unwrap();

    let mut last_end: Option<NaiveTime> = None;

    for r in rows {
        if let Ok((start, end, _dur)) = r {
            if let Some(end_str) = end {
                if let Ok(t) = NaiveTime::parse_from_str(&end_str, "%H:%M:%S") {
                    last_end = Some(t);
                }
            }
        }
    }

    // 2. Als er nog geen taken gepland zijn → start om 09:00
    let start_time = if let Some(last) = last_end {
        last
    } else {
        NaiveTime::from_hms_opt(9, 0, 0).unwrap()
    };

    // 3. Einde berekenen
    let end_time = start_time + Duration::minutes(point.duration_minutes as i64);

    point.start_time = Some(start_time.format("%H:%M:%S").to_string());
    point.end_time   = Some(end_time.format("%H:%M:%S").to_string());


    println!("[MARTHE] Gepland: {} → {}", 
        start_time.format("%H:%M"),
        end_time.format("%H:%M")
    );
}
}
