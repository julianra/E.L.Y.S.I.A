// ======================================================================
// 📍 FILE: elysia_core/src/events.rs
// ======================================================================

use serde_json::Value;
use tokio::task;
use crate::kernel::KernelState;
use crate::security::get_module_state;

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

        // 🔒 Lees registry één keer
        let modules = {
            let registry = state.modules.read().await;
            registry.iter().collect::<Vec<_>>()
        };

        let conn = state.ctx.db();

        for module in modules {
            let module_name = module.name().to_lowercase();

            // 🔒 ZERO-TRUST ENFORCEMENT
            let paired = match get_module_state(&conn, &module_name) {
                Ok(Some(m)) => m.paired,
                _ => false,
            };

            if !paired {
                log::warn!(
                    "[EVENTBUS] blocked event '{}' for unpaired module '{}'",
                    event.name,
                    module_name
                );
                continue;
            }

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
