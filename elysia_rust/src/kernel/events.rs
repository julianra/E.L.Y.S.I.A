use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    KernelHeartbeat,
    MartheHeartbeat,
    TaskAdded { name: String },
}
