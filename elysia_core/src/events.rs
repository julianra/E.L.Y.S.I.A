// ======================================================================
// 📍 FILE: elysia_core/src/events.rs
//
// 📝 BESCHRIJVING:
//   De ELYSIA EventBus — een async, multi-handler event systeem dat
//   alle modules onafhankelijk events laat verwerken.
//
//   BELANGRIJK:
//     - Modules worden opgeslagen als Arc<dyn ElysiaModule>
//     - Elke module krijgt zijn eigen Tokio task
//     - Geen borrow-issues meer
//     - Geen blocking in kernel
//
//   Modules ontvangen events via:
//
//       fn handle_event(&self, state: &KernelState, event: &KernelEvent)
//
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

    /// Verstuur een event naar ALLE modules (elk in eigen async task).
    pub async fn emit(&self, name: &str, payload: Value, state: KernelState) {
        let event = KernelEvent {
            name: name.to_string(),
            payload,
        };

        // Clone module list zodat closure 'static wordt
        let modules = state.modules.clone();

        for module in modules.iter() {
            let module = module.clone();     // Arc clone → 'static
            let state = state.clone();       // KernelState clone → Arc inside
            let event = event.clone();       // deep clone → safe

            task::spawn(async move {
                module.handle_event(&state, &event);

                log::debug!(
                    "[EVENTBUS] '{}' delivered to module '{}'",
                    event.name,
                    module.name()
                );
            });
        }
    }
}
