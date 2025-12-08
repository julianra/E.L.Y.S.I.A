// ======================================================================
// 📍 FILE: elysia_core/src/kernel_api/health.rs
// 📝 Kernel health state, used by Portal for node selection.
// ======================================================================

pub enum KernelHealth {
    Ok,
    Error(String),
}

pub fn get_kernel_health() -> KernelHealth {
    KernelHealth::Ok
}
