// ======================================================================
// 📍 FILE: elysia_core/src/kernel.rs
//
// 📝 DEEL 1 – SECURITY BASELINE
//     - Enable ConnectInfo<SocketAddr> so /auth/create_admin
//       can check client IP.
//
// ======================================================================

use std::sync::Arc;
use std::net::SocketAddr;

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
// KernelState – runtime state
// --------------------------------------------------
#[derive(Clone)]
pub struct KernelState {
    pub ctx: Arc<KernelContext>,
    pub modules: Arc<Vec<Box<dyn crate::module::ElysiaModule>>>,
    pub bus: Arc<EventBus>,
}

// --------------------------------------------------
// Kernel
// --------------------------------------------------
pub struct Kernel;

impl Kernel {
    pub async fn boot() -> anyhow::Result<()> {
        info!("[CORE] Booting ELYSIA Kernel 2.0…");

        // --- DB init -------------------------------------------------------
        let (pool, db_path) = init_database()?;
        info!("[CORE] Database ready at {}", db_path);

        {
            let conn = pool.get()?;
            crate::db::run_migrations(&conn)?;
            info!("[CORE] Migrations applied");
        }

        // --- Context -------------------------------------------------------
        let ctx = Arc::new(KernelContext::new(pool));

        // --- EventBus ------------------------------------------------------
        let bus = Arc::new(EventBus::new());

        // --- Modules -------------------------------------------------------
        let modules = Arc::new(load_modules());
        info!("[CORE] Loaded {} modules", modules.len());

        // --- Runtime state -------------------------------------------------
        let state = KernelState { ctx, modules, bus };

        // --- mDNS ----------------------------------------------------------
        start_mdns(2022, &state)?;

        // --- HTTP server ---------------------------------------------------
        let app = build_http_router(state.clone());

        task::spawn(async move {
            let listener = TcpListener::bind("0.0.0.0:2022")
                .await
                .expect("Failed to bind port 2022");

            info!("[CORE] HTTP server running on port 2022");

            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .expect("Kernel HTTP crashed");
        });

        info!("[CORE] Kernel boot sequence complete.");

        Ok(())
    }
}
