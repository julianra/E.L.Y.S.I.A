// ===============================================
// FILE: src/modules/marthe/marthe_core.rs
// ROLE: MARTHE Module Core
// ===============================================

use tokio::sync::mpsc::Receiver;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::kernel::events::Event;
use crate::kernel::agenda_point::AgendaPoint;

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

                    let point = AgendaPoint::from_external(req);
                    self.save_to_db(&point);

                    println!("[MARTHE] Opgeslagen {}", point.id);
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
            "INSERT INTO agenda (id, name, duration_minutes, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            (
                &point.id,
                &point.name,
                point.duration_minutes,
                point.created_at.to_rfc3339(),
            ),
        )
        .expect("[MARTHE] Kon agendapunt niet opslaan");
    }
}
