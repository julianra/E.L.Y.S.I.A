// ===============================================
// FILE: src/modules/marthe/marthe_core.rs
// ROLE: MARTHE Module Core
// ===============================================

use tokio::sync::mpsc::Receiver;

use crate::kernel::events::Event;
use crate::kernel::agenda_point::AgendaPoint;

use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};

pub struct MartheCore {
    receiver: Receiver<Event>,
    agenda: Vec<AgendaPoint>,
    db: Arc<Mutex<Connection>>,
}

impl MartheCore {
    pub fn new(receiver: Receiver<Event>, db: Arc<Mutex<Connection>>) -> Self {
        println!("[MARTHE] Module wordt opgestart…");

        Self {
            receiver,
            agenda: Vec::new(),
            db,
        }
    }

    pub async fn run(&mut self) {
        println!("[MARTHE] Module gestart.");

        while let Some(event) = self.receiver.recv().await {
            match event {

                Event::ExternalAgendaAdd(req) => {
                    println!("[MARTHE] Extern agendaverzoek: {}", req.name);

                    let point = AgendaPoint::from_external(req);

                    self.agenda.push(point.clone());
                    self.save_to_db(&point);

                    println!("[MARTHE] Opgeslagen: {}", point.id);
                }

                Event::AgendaPointCreated(point) => {
                    println!("[MARTHE] Confirm event: {}", point.name);
                }

                _ => {
                    println!("[MARTHE] Event genegeerd.");
                }
            }
        }
    }

    fn save_to_db(&self, point: &AgendaPoint) {
        let conn = self.db.lock().unwrap();

        conn.execute(
            "INSERT INTO agenda 
            (id, name, duration_minutes, created_at, start_time, end_time, priority, task_type,
             project, location, deadline, energy_cost, category, recurrence, importance_score,
             predicted_duration, confidence_score, emotional_load)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            params![
                point.id,
                point.name,
                point.duration_minutes,
                point.created_at.to_rfc3339(),

                point.start_time.map(|t| t.to_rfc3339()),
                point.end_time.map(|t| t.to_rfc3339()),
                point.priority,
                point.task_type,
                point.project,
                point.location,
                point.deadline.map(|t| t.to_rfc3339()),

                point.energy_cost,
                point.category,
                point.recurrence,
                point.importance_score,
                point.predicted_duration,
                point.confidence_score,
                point.emotional_load,
            ],
        )
        .expect("[MARTHE] ✘ Database insert fout");
    }
}
