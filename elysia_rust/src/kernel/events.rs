// ===============================================
// FILE: src/kernel/events.rs
// ROLE: Global Event Definitions
// PART OF: Kernel Layer
// PURPOSE:
// - Alle event types voor interne communicatie
// - API -> Kernel -> Modules
// ===============================================

use serde::{Serialize, Deserialize};

use crate::api::external::ExternalAgendaRequest;
use crate::kernel::agenda_point::AgendaPoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Externe app wil een agenda-item toevoegen
    ExternalAgendaAdd(ExternalAgendaRequest),

    /// MARTHE heeft een AgendaPoint aangemaakt en eventueel in de database gezet
    AgendaPointCreated(AgendaPoint),
}
