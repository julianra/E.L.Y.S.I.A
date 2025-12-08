// ======================================================================
// 📍 FILE: elysia_core/src/pairing/ipc.rs
// 📝 Internal IPC structs for pairing state.
// ======================================================================

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PairingState {
    pub active: bool,
    pub expires_in: Option<u64>,
    pub code: Option<String>,
}
