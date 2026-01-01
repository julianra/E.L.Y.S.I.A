// ======================================================================
// 📍 FILE: elysia_core/src/kernel_api/nodes.rs
// 📝 Multi-node information (placeholder for future Fase 3/4).
//     Kernel never exposes this via HTTP.
// ======================================================================

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub ip: String,
    pub port: u16,
    pub version: String,
    pub roles: Vec<String>,
}

pub fn list_known_nodes() -> Vec<NodeInfo> {
    // Kernel does not store nodes yet (until Fase 3 multi-node networking)
    // Placeholder internal API for future expansion.
    vec![]
}
