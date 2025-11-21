use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

use chrono::Utc;
use uuid::Uuid;

pub mod events;
pub mod event_bus;
pub mod agenda_point;
pub mod database;

use events::Event;
use event_bus::EventBus;
use agenda_point::AgendaPoint;
use database::Database;

use crate::modules::marthe::MartheCore;

pub struct Kernel;

impl Kernel {
    pub fn new() -> Self {
        Kernel
    }

    pub async fn run(&self) {
        println!("[KERNEL] Kernel is running...");

        // === 1) DATABASE OPENEN ===
        let db = Database::new().expect("Kon database niet openen");

        // === 2) TABELLEN MAKEN VOOR GEBRUIK ===
        db.init().expect("Kon database structuur niet initialiseren");
        println!("[KERNEL] Database structuur klaar.");

        // === 3) EVENTBUS AANMAKEN ===
        let (bus_tx, mut bus_rx) = mpsc::channel::<Event>(64);
        let event_bus = EventBus::new(bus_tx.clone());

        // === 4) MAILBOX VOOR MARTHE ===
        let (marthe_tx, marthe_rx) = mpsc::channel::<Event>(32);

        // === 5) START MARTHE MODULE ===
        tokio::spawn({
            let db_conn = db.conn.clone();
            async move {
                let mut marthe = MartheCore::new(marthe_rx, db_conn);
                marthe.run().await;
            }
        });

        // === 6) KERNEL MAIN LOOP ===
        loop {
            // Heartbeat
            event_bus.send(Event::KernelHeartbeat).await;

            // DEMO-AGENDAITEM
            let agenda = AgendaPoint {
                id: Uuid::new_v4().to_string(),
                name: "Koken om 16:30".to_string(),
                duration_minutes: 45,
                created_at: Utc::now(),

                start_time: None,
                end_time: None,
                priority: None,
                task_type: None,
                project: None,
                location: None,
                deadline: None,

                energy_cost: None,
                category: None,
                recurrence: None,
                importance_score: None,
                predicted_duration: None,
                confidence_score: None,
                emotional_load: None,

                required_tools: None,
                blocking_rules: None,
                context_tags: None,
                linked_tasks: None,
            };

            event_bus.send(Event::AgendaAdded(agenda.clone())).await;

            // Ontvang events van EventBus
            if let Some(event) = bus_rx.recv().await {
                println!("[KERNEL] Received event: {:?}", event);

                // Stuur ALLE events door naar Marthe
                let _ = marthe_tx.send(event.clone()).await;
            }

            sleep(Duration::from_secs(3)).await;
        }
    }
}
