// ======================================================================
// 📍 FILE: elysia_core/src/context.rs
//
// 📝 BESCHRIJVING:
//   Houdt metadata en database-verbinding bij voor ALLE modules.
// ======================================================================

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct KernelContext {
    pub db_pool: Arc<Pool<SqliteConnectionManager>>,
    pub meta: Arc<RwLock<HashMap<String, String>>>,
}

impl KernelContext {
    pub fn new(db_pool: Pool<SqliteConnectionManager>) -> Self {
        KernelContext {
            db_pool: Arc::new(db_pool),
            meta: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn clone_for_http(&self) -> Self {
        self.clone()
    }

    pub fn db(&self) -> r2d2::PooledConnection<SqliteConnectionManager> {
        self.db_pool.get().expect("Failed to get DB connection")
    }

    pub fn set_meta(&self, key: &str, value: &str) {
        self.meta.write().unwrap().insert(key.to_string(), value.to_string());
    }
}
