// ======================================================================
// 📍 FILE: elysia_core/src/kernel.rs
// ======================================================================

use crate::{KernelContext, Router, EventBus, ElysiaModule};
use thiserror::Error;
use mdns_sd::ServiceDaemon;
use tokio::runtime::Runtime;
use std::sync::Arc;

use elysia_ai::AiKernel;
use elysia_ai::backends::ollama::OllamaBackend;

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
    pub async fn boot_and_run() -> Result<(), KernelError> {
        log::info!("[CORE] Booting ELYSIA Kernel...");

        let (pool, db_path) = crate::db_init::init_database()
            .map_err(|e|
                KernelError::InitError(format!("DB init failed: {}", e))
            )?;

        log::info!("[CORE] Database initialized at {}", db_path);

        let ai_backend = Arc::new(OllamaBackend::new());
        let ai_kernel = Arc::new(AiKernel::new(ai_backend));

        let mut kernel = Kernel {
            ctx: KernelContext::new(pool, ai_kernel),
            router: Router::new(),
            bus: EventBus::new(),
            modules: vec![],
            _mdns: crate::mdns::start_mdns(3000).ok(),
        };

        for reg in inventory::iter::<crate::module::ModuleRegistration> {
            kernel.modules.push((reg.module)());
        }

        for module in &kernel.modules {
            module.init(&kernel.ctx);
            module.register_routes(&mut kernel.router);
            module.register_event_handlers(&mut kernel.bus);
        }

        let state = KernelState {
            ctx: Arc::new(kernel.ctx.clone_for_http()),
            bus: Arc::new(kernel.bus.clone_for_http()),
            modules: Arc::new(kernel.modules.iter().map(|m| m.clone()).collect()),
        };

        tokio::spawn(async move {
            let app = crate::http::build_router(state);
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
                .await
                .expect("Failed to bind kernel HTTP listener");

            log::info!("[CORE] HTTP server running on port 3000");

            axum::serve(listener, app)
                .await
                .expect("Kernel HTTP server crashed");
        });

        // ⭐ BELANGRIJK: Kernel klaar → OK teruggeven
        Ok(())
    }
}
