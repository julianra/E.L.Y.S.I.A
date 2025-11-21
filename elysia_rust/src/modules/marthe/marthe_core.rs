use tokio::sync::mpsc::Receiver;

use crate::kernel::events::Event;
use crate::kernel::agenda_point::AgendaPoint;

use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct MartheCore {
    receiver: Receiver<Event>,
    agenda: Vec<AgendaPoint>,
    db: Arc<Mutex<Connection>>,
}

impl MartheCore {
    pub fn new(receiver: Receiver<Event>, db: Arc<Mutex<Connection>>) -> Self {
        println!("[MARTHE] Module wordt opgestart…");

        let agenda = Vec::new();   // later: load from DB

        Self {
            receiver,
            agenda,
            db,
        }
    }

    pub async fn run(&mut self) {
        println!("[MARTHE] Module gestart.");

        while let Some(event) = self.receiver.recv().await {
            match event {
                Event::KernelHeartbeat => {
                    println!("[MARTHE] Kernel leeft ✔");
                }

                Event::AgendaAdded(point) => {
                    println!(
                        "[MARTHE] Nieuw agendapunt ontvangen: {} ({} min)",
                        point.name, point.duration_minutes
                    );

                    // In RAM opslaan
                    self.agenda.push(point.clone());

                    // In DB opslaan
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
                    .expect("Kon agendapunt niet opslaan in database");

                    println!("[MARTHE] Agenda totaal: {}", self.agenda.len());
                }

                Event::MartheHeartbeat => {
                    // Niet gebruikt, maar verplicht om match compleet te maken.
                }
            }
        }
    }
}
