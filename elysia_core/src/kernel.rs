// ======================================================================
// 📍 FILE: elysia_core/src/kernel.rs
//
// 📝 BESCHRIJVING:
//   De centrale ELYSIA Kernel. Deze file start:
//      - Database
//      - EventBus
//      - Module loader
//      - HTTP router (modulair)
//      - mDNS (met node capabilities)
//      - Runtime state voor alle modules
//
//   Deze kernel is volledig future-proof en vormt het OS-hart van ELYSIA.
// ======================================================================

use std::sync::Arc;

use crate::{
    context::KernelContext,
    db_init::init_database,
    events::EventBus,
    http::build_http_router,
    mdns::start_mdns,
    module::registry::load_modules,
};

use tokio::net::TcpListener;
use tokio::task;
use log::info;

// --------------------------------------------------
// KernelState – runtime state voor alle modules
// --------------------------------------------------
#[derive(Clone)]
pub struct KernelState {
    pub ctx: Arc<KernelContext>,
    pub modules: Arc<Vec<Box<dyn crate::module::ElysiaModule>>>,
    pub bus: Arc<EventBus>,
}

// --------------------------------------------------
// Kernel struct
// --------------------------------------------------
pub struct Kernel;

impl Kernel {
    pub async fn boot() -> anyhow::Result<()> {
        info!("[CORE] Booting ELYSIA Kernel 2.0…");

        // --- Database initialisatie ---------------------------------------
        let (pool, db_path) = init_database()?;
        info!("[CORE] Database ready at {}", db_path);

        // --- Context (DB + AI placeholder + meta) --------------------------
        let ctx = Arc::new(KernelContext::new(pool));

        // --- Event bus -----------------------------------------------------
        let bus = Arc::new(EventBus::new());

        // --- Modules laden -------------------------------------------------
        let modules = Arc::new(load_modules());
        info!("[CORE] Loaded {} modules", modules.len());

        // --- Kernel runtime state ------------------------------------------
        let state = KernelState {
            ctx,
            modules,
            bus,
        };

        // --- mDNS service (node discovery) --------------------------------
        start_mdns(2022, &state)?;

        // --- HTTP server start ---------------------------------------------
        let app = build_http_router(state.clone());

        task::spawn(async move {
            let listener = TcpListener::bind("0.0.0.0:2022")
                .await
                .expect("Failed to bind port 2022");

            info!("[CORE] HTTP server running on port 2022");

            axum::serve(listener, app)
                .await
                .expect("Kernel HTTP crashed");
        });

        info!("[CORE] Kernel boot sequence complete.");

        Ok(())
    }
}
