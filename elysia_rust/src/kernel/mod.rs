pub mod event_bus;
pub mod events;
pub mod tasks;

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use crate::kernel::event_bus::EventBus;
use crate::kernel::events::Event;
use crate::kernel::tasks::Task;

use crate::modules::marthe::MartheCore;

pub struct Kernel;

impl Kernel {
    pub fn new() -> Self {
        Kernel
    }

    pub async fn run(&self) {
        println!("[KERNEL] Kernel is running...");

        // === EventBus kanaal voor Kernel zelf ===
        let (kernel_tx, mut kernel_rx) = mpsc::channel(64);
        let bus = EventBus::new(kernel_tx.clone());

        // === Marthe krijgt haar eigen mailbox (receiver) ===
        let (marthe_tx, marthe_rx) = mpsc::channel(32);

        // === Start Marthe module ===
        tokio::spawn(async move {
            let mut marthe = MartheCore::new(marthe_rx);
            marthe.run().await;
        });

        // === KERNEL MAIN LOOP ===
        loop {
            // 1) Kernel heartbeat uitsturen
            bus.send(Event::KernelHeartbeat).await;

            // 2) Test: stuur taak naar iedereen
            bus.send(Event::TaskAdded(Task {
                name: "Koken om 16:30".to_string(),
                duration_minutes: 45,
            })).await;

            // 3) Kernel ontvangt ALLE events van modules
            tokio::select! {
                Some(event) = kernel_rx.recv() => {
                    println!("[KERNEL] Received event: {:?}", event);

                    // 4) Stuur ALLE events door naar Marthe
                    let _ = marthe_tx.send(event.clone()).await;
                }
            }

            sleep(Duration::from_secs(3)).await;
        }
    }
}
