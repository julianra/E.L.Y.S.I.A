// ======================================================================
// 📍 FILE: elysia_core/src/kernel_api/status.rs
// ======================================================================

use serde::Serialize;
use std::sync::Arc;
use crate::kernel::KernelState;

#[derive(Serialize)]
pub struct KernelStatus {
    pub status: String,
    pub version: String,
    pub db: String,
    pub modules: usize,
}

pub async fn get_kernel_status(state: &Arc<KernelState>) -> KernelStatus {
    let count = state.modules.read().await.len(); // 🔒 single source of truth

    KernelStatus {
        status: "online".into(),
        version: "2.1".into(),
        db: "ok".into(),
        modules: count,
    }
}
