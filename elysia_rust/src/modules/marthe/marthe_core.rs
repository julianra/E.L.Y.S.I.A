use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::Sender;

use crate::kernel::events::Event;

pub struct MartheCore {
    sender: Sender<Event>,
}

impl MartheCore {
    pub fn new(sender: Sender<Event>) -> Self {
        MartheCore { sender }
    }

    pub async fn run(&self) {
        println!("[MARTHE] Module gestart.");

        loop {
            println!("[MARTHE] Wachten op nieuwe taken...");

            // Stuur heartbeat naar kernel
            let _ = self.sender.send(Event::MartheHeartbeat).await;

            sleep(Duration::from_secs(4)).await;
        }
    }
}
