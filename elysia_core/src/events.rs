// ======================================================================
// 📍 FILE: elysia_core/src/events.rs
// ======================================================================

use serde_json::Value;
use crate::module::ElysiaModule;
use crate::kernel::KernelState;

#[derive(Debug, Clone)]
pub struct KernelEvent {
    pub name: String,
    pub payload: Value,
}

#[derive(Clone, Default)]
pub struct EventBus {
    pub handlers: Vec<String>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }

    pub fn register_handler(&mut self, handler: &str) {
        self.handlers.push(handler.to_string());
    }

    pub fn clone_for_http(&self) -> Self {
        self.clone()
    }

    // ⚡ Belangrijk: ctx → state
    pub fn dispatch(
        &self,
        event: KernelEvent,
        modules: &std::sync::Arc<Vec<Box<dyn ElysiaModule>>>,
        state: &KernelState
    ) {
        let target = event.name.clone();

        for module in modules.iter() {
            let key = format!("{}.{}", module.name(), target);

            if self.handlers.contains(&key) {
                module.handle_event(state, event.clone());
            }
        }
    }
}
