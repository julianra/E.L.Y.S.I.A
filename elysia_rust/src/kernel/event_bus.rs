use tokio::sync::mpsc;
use super::events::Event;

#[derive(Clone)]
pub struct EventBus {
    pub sender: mpsc::Sender<Event>,
}

impl EventBus {
    pub fn new(sender: mpsc::Sender<Event>) -> Self {
        EventBus { sender }
    }

    pub async fn send(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
}
