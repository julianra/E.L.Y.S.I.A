// ======================================================================
// 📍 FILE: elysia/elysia_core/src/context.rs
//
// 📝 BESCHRIJVING:
//   De globale context waarin alle modules draaien.
//   Dit wordt later uitgebreid met database-connecties,
//   config, caches, pipelines, enz.
//
// 🔧 TAKEN:
//   - Opslag van key-value metadata
//   - Toegang tot globale systeemstatus
//   - Delen van data tussen kernel en modules
// ======================================================================

use std::collections::HashMap;

#[derive(Default)]
pub struct KernelContext {
    pub metadata: HashMap<String, String>,
}

impl KernelContext {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new()
        }
    }

    pub fn set_meta(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn get_meta(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}
