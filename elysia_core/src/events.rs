// ======================================================================
// 📍 FILE: elysia/elysia_core/src/events.rs
//
// 📝 BESCHRIJVING:
//   Een eenvoudige EventBus voor het registreren van event handlers.
//   Wordt later uitgebreid naar async message queue.
//
// 🔧 TAKEN:
//   - Registreren van event handlers
//   - Modules kunnen zich abonneren op events
//   - Kernel toont geregistreerde handlers bij opstart
// ======================================================================
use serde_json::Value;
use crate::KernelContext;
use crate::module::ElysiaModule;

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

    pub fn dispatch(
        &self,
        event: KernelEvent,
        modules: &[Box<dyn ElysiaModule>],
        ctx: &KernelContext
    ) {
        let target = event.name.clone();

        for module in modules {
            let key = format!("{}.{}", module.name(), target);

            if self.handlers.contains(&key) {
                module.handle_event(ctx, event.clone());
            }
        }
    }
}
