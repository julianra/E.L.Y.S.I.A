pub mod event_bus;
pub mod events;

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use crate::modules::marthe::MartheCore;
use crate::kernel::event_bus::EventBus;
use crate::kernel::events::Event;




pub struct Kernel;

impl Kernel {
    pub fn new() -> Self {
        Kernel
    }

    pub async fn run(&self) {
        println!("[KERNEL] Kernel is running...");

        // Maak kanaal (bus)
        let (tx, mut rx) = mpsc::channel(32);
        let bus = EventBus::new(tx.clone());

        // Start Marthe
        tokio::spawn(async move {
            let marthe = MartheCore::new(tx.clone()); // TX naar Marthe
            marthe.run().await;
        });

        // KERNEL MAIN LOOP
        loop {
            // Kernel stuurt heartbeat
            bus.send(Event::KernelHeartbeat).await;

            // Kernel ontvangt ALLE events hier
            tokio::select! {
                Some(event) = rx.recv() => {
                    println!("[KERNEL] Received event: {:?}", event);
                }
            }

            sleep(Duration::from_secs(3)).await;
        }
    }
}
