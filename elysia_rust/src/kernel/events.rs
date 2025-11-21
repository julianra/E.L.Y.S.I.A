use serde::{Serialize, Deserialize};
use crate::kernel::tasks::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    KernelHeartbeat,
    MartheHeartbeat,
    TaskAdded(Task),
}
