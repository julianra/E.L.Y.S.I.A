use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::{Receiver};

use crate::kernel::events::Event;

pub struct MartheCore {
    pub receiver: Receiver<Event>,
}

impl MartheCore {
    pub fn new(receiver: Receiver<Event>) -> Self {
        Self { receiver }
    }

    pub async fn run(&mut self) {
        println!("[MARTHE] Module gestart.");

        while let Some(event) = self.receiver.recv().await {
            match event {
                Event::TaskAdded(task) => {
                    println!(
                        "[MARTHE] Nieuwe taak ontvangen: {} ({} min)",
                        task.name,
                        task.duration_minutes
                    );
                }
                Event::KernelHeartbeat => {
                    println!("[MARTHE] Kernel leeft nog ✔");
                }
                Event::MartheHeartbeat => {}
            }
        }
    }
}
