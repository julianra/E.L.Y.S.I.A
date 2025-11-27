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

#[derive(Default)]
pub struct EventBus {
    pub handlers: Vec<String>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn register_handler(&mut self, event: &str) {
        self.handlers.push(event.to_string());
    }
}
