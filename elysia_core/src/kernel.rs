// ======================================================================
// 📍 FILE: elysia/elysia_core/src/kernel.rs
//
// 📝 BESCHRIJVING:
//   De kern van het ELYSIA-platform. Beheert de volledige lifecycle:
//   modules registreren, init, routes, events, tasks, en de runtime.
//
// 🔧 TAKEN:
//   - Boot sequence (logging, context)
//   - Modules registreren
//   - Lifecycle-functies aanroepen op modules
//   - Router & EventBus configureren
//   - Module background-taken starten
//   - Kernel runtime draaien (voor nu: dummy loop)
// ======================================================================
// 📍 FILE: elysia_core/src/kernel.rs
// ======================================================================

use crate::{KernelContext, Router, EventBus, ElysiaModule};
use thiserror::Error;
use mdns_sd::ServiceDaemon;
use tokio::runtime::Runtime;
use std::sync::Arc;

#[derive(Clone)]
pub struct KernelState {
    pub ctx: Arc<KernelContext>,
    pub bus: Arc<EventBus>,
    pub modules: Arc<Vec<Box<dyn ElysiaModule>>>,
}

pub struct Kernel {
    pub ctx: KernelContext,
    pub router: Router,
    pub bus: EventBus,
    pub modules: Vec<Box<dyn ElysiaModule>>,
    pub _mdns: Option<ServiceDaemon>,
}

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("initialization error: {0}")]
    InitError(String),
}

impl Kernel {
    pub fn boot_and_run() -> Result<(), KernelError> {
        env_logger::init();
        log::info!("[CORE] Booting ELYSIA Kernel...");

        let (pool, db_path) = crate::db_init::init_database()
            .map_err(|e| KernelError::InitError(format!("DB init failed: {}", e)))?;
        log::info!("[CORE] Database initialized at {}", db_path);

        let mdns_handle = match crate::mdns::start_mdns(3000) {
            Ok(h) => Some(h),
            Err(e) => {
                log::warn!("[CORE] mDNS failed: {}", e);
                None
            }
        };

        let mut kernel = Kernel {
            ctx: KernelContext::new(pool),
            router: Router::new(),
            bus: EventBus::new(),
            modules: vec![],
            _mdns: mdns_handle,
        };

        // ----------------------------
        // MODULES LADEN
        // ----------------------------
        for reg in inventory::iter::<crate::module::ModuleRegistration> {
            kernel.modules.push((reg.module)());
        }

        for module in &kernel.modules {
            module.init(&kernel.ctx);
            module.register_routes(&mut kernel.router);
            module.register_event_handlers(&mut kernel.bus);
        }

        // ----------------------------
        // SHARED STATE VOOR AXUM
        // ----------------------------
        let state = KernelState {
            ctx: Arc::new(kernel.ctx.clone_for_http()),
            bus: Arc::new(kernel.bus.clone_for_http()),
            modules: Arc::new(kernel.modules.iter().map(|m| m.clone()).collect()),
        };

        // ----------------------------
        // START HTTP SERVER
        // ----------------------------
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let app = crate::http::build_router(state);
                let listener =
                    tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
                log::info!("[CORE] HTTP server running on port 3000");
                axum::serve(listener, app).await.unwrap();
            });
        });

        // ----------------------------
        // MAIN LOOP
        // ----------------------------
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
