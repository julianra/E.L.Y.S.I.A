// ======================================================================
// 📍 FILE: elysia_core/src/context.rs
//
// 📝 BESCHRIJVING:
//   Houdt metadata en database-verbinding bij voor ALLE modules.
// ======================================================================

use std::collections::HashMap;
use rusqlite::Connection;

pub struct KernelContext {
    meta: HashMap<String, String>,
    db: Option<Connection>,
}

impl KernelContext {
    pub fn new() -> Self {
        Self {
            meta: HashMap::new(),
            db: None,
        }
    }

    // Metadata
    pub fn set_meta(&mut self, key: &str, value: &str) {
        self.meta.insert(key.to_string(), value.to_string());
    }

    pub fn get_meta(&self, key: &str) -> Option<&String> {
        self.meta.get(key)
    }

    // Database
    pub fn set_db(&mut self, conn: Connection) {
        self.db = Some(conn);
    }

    pub fn db(&self) -> &Connection {
        self.db.as_ref().expect("Database not initialized")
    }
}
