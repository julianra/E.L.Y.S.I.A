// ======================================================================
// 📍 FILE: elysia_core/src/module/mod.rs
// 📝 Kernel-side module system: registry + module trait.
// ======================================================================

pub mod registry;

use crate::events::KernelEvent;
use crate::kernel::KernelState;

pub trait ElysiaModule: Send + Sync {
    fn name(&self) -> &'static str;

    fn handle_event(&self, _state: &KernelState, _event: &KernelEvent) {}
}
