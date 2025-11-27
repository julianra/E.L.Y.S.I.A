// ======================================================================
// 📍 FILE: elysia/elysia_core/src/event/bus.rs
//
// 📝 BESCHRIJVING:
//   De EventBus van ELYSIA. Laat modules events publiceren en
//   async handlers registreren.
//
// 🔧 TAKEN:
//   - Event handlers registreren
//   - Typed events dispatchen
//   - Async uitvoering van handlers
//   - Events toelaten zonder listeners
// ======================================================================

use std::collections::HashMap;
use std::sync::Arc;

use super::event::EventType;

pub type EventHandler = Arc<dyn Fn(EventType) -> EventHandlerFuture + Send + Sync>;
pub type EventHandlerFuture = std::pin::Pin<Box<dyn std::future::Future<Output=()> + Send>>;

#[derive(Default)]
pub struct EventBus {
    handlers: HashMap<String, Vec<EventHandler>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe<F, Fut>(&mut self, event_name: &str, handler: F)
    where
        F: Fn(EventType) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.handlers
            .entry(event_name.to_string())
            .or_default()
            .push(Arc::new(move |ev| Box::pin(handler(ev))));
    }

    pub async fn publish(&self, event: EventType) {
        let name = event.name();

        if let Some(listeners) = self.handlers.get(name) {
            for handler in listeners {
                handler(event.clone()).await;
            }
        }
    }

    /// Handige helper om alle geregistreerde event-namen op te halen (voor logging).
    pub fn list(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}
