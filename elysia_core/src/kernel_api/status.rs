// ======================================================================
// 📍 FILE: elysia_core/src/kernel_api/status.rs
// 📝 Internal Kernel Status API (no HTTP, no JSON).
//     Pure data for monitoring via IPC.
// ======================================================================

use crate::kernel::KernelState;

pub struct KernelStatus {
    pub running: bool,
    pub modules: usize,
    pub db_online: bool,
    pub version: &'static str,
}

pub fn get_kernel_status(state: &KernelState) -> KernelStatus {
    KernelStatus {
        running: true,
        modules: state.modules.len(),
        db_online: true,
        version: "2.1",
    }
}
