// ======================================================================
// 📍 FILE: elysia/elysia_core/src/module.rs
//
// 📝 BESCHRIJVING:
//   Definieert de interface waar ALLE ELYSIA-modules aan moeten voldoen.
//   (Marthe, Junk, Catnip, Finn, enz.)
//
// 🔧 TAKEN:
//   - Uniforme lifecycle API voor modules
//   - Kernel kan modules generiek behandelen
//   - Modules kunnen routes, events en background tasks registreren
// ======================================================================

use crate::{KernelContext, Router, EventBus};

pub trait ElysiaModule: Send + Sync {
    fn name(&self) -> &'static str;

    fn init(&self, _ctx: &mut KernelContext) {}
    fn register_routes(&self, _router: &mut Router) {}
    fn register_event_handlers(&self, _bus: &mut EventBus) {}
    fn start_background_tasks(&self, _ctx: &KernelContext) {}
}
