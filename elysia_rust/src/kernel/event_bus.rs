// ===============================================
// FILE: src/kernel/event_bus.rs
// ROLE: EventBus voor interne communicatie
// ===============================================
use tokio::sync::mpsc::Sender;
use crate::kernel::events::Event;

#[derive(Clone)]
pub struct EventBus {
    sender: Sender<Event>,
}

impl EventBus {
    pub fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }

    pub async fn publish(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
}
