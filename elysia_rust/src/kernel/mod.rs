// Elysia Rust - kernel/mod.rs
// ===============================================
// FILE: src/kernel/mod.rs
// ROLE: Kernel Core
// PART OF: Kernel Layer
// PURPOSE:
// - Beheer van de kernfunctionaliteit
// - Initialisatie en event loop
// ===============================================
use tokio::sync::mpsc;

pub mod events;
pub mod event_bus;
pub mod agenda_point;
pub mod database;

use events::Event;
use event_bus::EventBus;
use database::Database;
use crate::modules::marthe::MartheCore;

pub struct Kernel;

impl Kernel {

    pub fn new() -> Self {
        Kernel
    }

    // Maak een eventbus die API en Kernel delen
    pub fn build_eventbus() -> (EventBus, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel::<Event>(64);
        (EventBus::new(tx), rx)
    }

    pub async fn run_with_bus(&self, mut bus_rx: mpsc::Receiver<Event>) {
        println!("[KERNEL] Kernel is running...");

        // Database openen
        let db = Database::new().expect("Kon database niet openen");
        db.init().expect("Kon database structuur niet initialiseren");

        // MARTHE mailbox
        let (marthe_tx, marthe_rx) = mpsc::channel::<Event>(32);

        // MARTHE starten
        tokio::spawn({
            let db_conn = db.conn.clone();
            async move {
                let mut marthe = MartheCore::new(marthe_rx, db_conn);
                marthe.run().await;
            }
        });

        // Kernel event-loop
        while let Some(event) = bus_rx.recv().await {
            println!("[KERNEL] Received event: {:?}", event);

            let _ = marthe_tx.send(event.clone()).await;
        }
    }
}
