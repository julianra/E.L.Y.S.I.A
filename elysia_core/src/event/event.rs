// ======================================================================
// 📍 FILE: elysia/elysia_core/src/event/event.rs
//
// 📝 BESCHRIJVING:
//   Definieert alle event types binnen ELYSIA.
//   Gebruikt een enum met payload structs voor typed events.
//
// 🔧 TAKEN:
//   - Typed events definiëren
//   - Payload data structuren beheren
//   - Event-name helpers voorzien
// ======================================================================

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingMailPayload {
    pub from: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAddedPayload {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    MailIncoming(IncomingMailPayload),
    MartheTaskAdded(TaskAddedPayload),
    CoreTickMinute,
    Custom(String), // fallback
}

impl EventType {
    pub fn name(&self) -> &'static str {
        match self {
            EventType::MailIncoming(_) => "mail.incoming",
            EventType::MartheTaskAdded(_) => "marthe.task_added",
            EventType::CoreTickMinute => "core.tick.minute",
            EventType::Custom(_) => "custom",
        }
    }
}
