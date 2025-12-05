// ======================================================================
// 📍 FILE: elysia_core/src/context.rs
//
// 📝 KernelContext zonder AI (AI komt later als module)
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

    pub fn db(&self) -> r2d2::PooledConnection<SqliteConnectionManager> {
        self.db_pool.get().expect("DB connection error")
    }
}
