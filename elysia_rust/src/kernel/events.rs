use serde::{Serialize, Deserialize};
use crate::kernel::agenda_point::AgendaPoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    KernelHeartbeat,
    MartheHeartbeat,
    AgendaAdded(AgendaPoint),
}
