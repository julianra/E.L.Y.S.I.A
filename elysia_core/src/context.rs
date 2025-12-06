// ======================================================================
// 📍 FILE: elysia_core/src/context.rs
//
// 📝 KernelContext met database + pairing-state + meta
// ======================================================================

use std::sync::{Arc, RwLock};
use std::time::Instant;

use r2d2::Pool;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct KernelContext {
    pub pool: Pool<SqliteConnectionManager>,

    // Oude metadata van core (kan later vervangen worden door DB)
    pub meta: Arc<RwLock<serde_json::Value>>,

    // Pairing-state
    pub pairing_enabled: Arc<RwLock<bool>>,
    pub pairing_code: Arc<RwLock<Option<String>>>,
    pub pairing_expires_at: Arc<RwLock<Option<Instant>>>,
}

impl KernelContext {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self {
            pool,

            meta: Arc::new(RwLock::new(serde_json::json!({}))),

            pairing_enabled: Arc::new(RwLock::new(false)),
            pairing_code: Arc::new(RwLock::new(None)),
            pairing_expires_at: Arc::new(RwLock::new(None)),
        }
    }

    // ------------------------------------------------------------------
    // 🗃 Database connection
    // ------------------------------------------------------------------
    pub fn db(&self) -> PooledConnection<SqliteConnectionManager> {
        self.pool.get().expect("DB pool error")
    }

    // ------------------------------------------------------------------
    // 🔐 Pairing actief JA/NEE
    //
    // Controle:
    //   - Is pairing_enabled true?
    //   - Is er een vervaldatum?
    //   - Als pairing verlopen is → auto-disable
    // ------------------------------------------------------------------
    pub fn pairing_is_active(&self) -> bool {
        let enabled = *self.pairing_enabled.read().unwrap();

        if !enabled {
            return false;
        }

        if let Some(expires) = *self.pairing_expires_at.read().unwrap() {
            if Instant::now() > expires {
                // Auto-disable pairing
                *self.pairing_enabled.write().unwrap() = false;
                *self.pairing_code.write().unwrap() = None;
                *self.pairing_expires_at.write().unwrap() = None;
                return false;
            }
            return true;
        }

        false
    }
}
