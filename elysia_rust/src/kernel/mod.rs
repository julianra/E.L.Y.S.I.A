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

    pub async fn run(&self) {
    println!("[KERNEL] Kernel is running...");

    // === 1) DATABASE OPENEN ===
    let db = Database::new().expect("Kon database niet openen");

    // === 2) STRUCTUUR AANMAKEN ===
    db.init().expect("Kon database structuur niet initialiseren");
    println!("[KERNEL] Database structuur klaar.");

    // === 3) EVENTBUS (tx/rx) ===
    let (bus_tx, mut bus_rx) = mpsc::channel::<Event>(64);
    let event_bus = EventBus::new(bus_tx.clone());

    // === 4) MARTHE MAILBOX ===
    let (marthe_tx, marthe_rx) = mpsc::channel::<Event>(32);

    // === 5) MARTHE STARTEN ===
    tokio::spawn({
        let db_conn = db.conn.clone();
        async move {
            let mut marthe = MartheCore::new(marthe_rx, db_conn);
            marthe.run().await;
        }
    });

    // === 6) MAIN EVENT LOOP ===
    loop {
        // Ontvang events van EventBus
        if let Some(event) = bus_rx.recv().await {
            println!("[KERNEL] Received event: {:?}", event);

            // ALLES naar Marthe sturen
            let _ = marthe_tx.send(event).await;
        }
    }
}

}
