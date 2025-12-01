// ======================================================================
// 📍 FILE: elysia_core/src/module.rs
// ======================================================================

use crate::{KernelContext, Router};
use crate::events::KernelEvent;
use crate::kernel::KernelState;

// Het basistrait dat ALLE modules moeten implementeren
pub trait ElysiaModule: Send + Sync {
    fn name(&self) -> &'static str;

    fn init(&self, _ctx: &KernelContext) {}
    fn register_routes(&self, _router: &mut Router) {}
    fn register_event_handlers(&self, _bus: &mut crate::events::EventBus) {}

    // ⚡ Belangrijk: KernelState i.p.v. KernelContext
    fn handle_event(&self, _state: &KernelState, _event: KernelEvent) {}

    fn box_clone(&self) -> Box<dyn ElysiaModule>;
}

impl Clone for Box<dyn ElysiaModule> {
    fn clone(&self) -> Box<dyn ElysiaModule> {
        self.box_clone()
    }
}

// ======================================================================
// 📌 Macro voor auto-registratie via inventory
// ======================================================================
pub struct ModuleRegistration {
    pub module: fn() -> Box<dyn ElysiaModule>,
}

inventory::collect!(ModuleRegistration);

#[macro_export]
macro_rules! register_module {
    ($module_type:ty) => {
        inventory::submit! {
            $crate::module::ModuleRegistration {
                module: || Box::new(<$module_type>::default())
            }
        }
    };
}
