// ======================================================================
// 📍 FILE: elysia_core/src/kernel_api/meta.rs
// 📝 Kernel metadata accessor.
//     Metadata lives inside KernelContext and can be modified internally.
// ======================================================================

use crate::kernel::KernelState;
use serde_json::Value;

pub fn get_kernel_meta(state: &KernelState) -> Value {
    state.ctx.meta.read().unwrap().clone()
}
