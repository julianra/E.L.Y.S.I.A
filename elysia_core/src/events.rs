// ======================================================================
// 📍 FILE: elysia_core/src/events.rs
// ======================================================================

use serde_json::Value;
use tokio::task;
use crate::kernel::KernelState;

#[derive(Clone, Debug)]
pub struct KernelEvent {
    pub name: String,
    pub payload: Value,
}

#[derive(Clone, Default)]
pub struct EventBus;

impl EventBus {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn emit(&self, name: &str, payload: Value, state: KernelState) {
        let event = KernelEvent {
            name: name.to_string(),
            payload,
        };

        // FIX: Modules are ARC clones → 'static safe
        let modules: Vec<_> = state.modules.iter().collect();

        for module in modules {
            let module_clone = module.clone();
            let state_clone = state.clone();
            let event_clone = event.clone();

            task::spawn(async move {
                module_clone.handle_event(&state_clone, &event_clone);

                log::debug!(
                    "[EVENTBUS] '{}' delivered to '{}'",
                    event_clone.name,
                    module_clone.name()
                );
            });
        }
    }
}
