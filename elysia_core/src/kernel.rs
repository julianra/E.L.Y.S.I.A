// ======================================================================
// 📍 FILE: elysia_core/src/kernel.rs
//
// 📝 Kernel boot sequence + pairing-state initialisatie.
//     - Database + migrations
//     - KernelContext (incl. pairing state + meta)
//     - EventBus
//     - Module loader
//     - mDNS discovery
//     - AXUM server (Axum 0.7 style)
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

#[derive(Clone)]
pub struct KernelState {
    pub ctx: Arc<KernelContext>,
    pub modules: Arc<Vec<Box<dyn crate::module::ElysiaModule>>>,
    pub bus: Arc<EventBus>,
}

pub struct Kernel;

impl Kernel {
    pub async fn boot() -> anyhow::Result<()> {
        info!("[CORE] Booting ELYSIA Kernel 2.0…");

        // ----------------------------------------------------
        // DATABASE INIT
        // ----------------------------------------------------
        let (pool, db_path) = init_database()?;
        info!("[CORE] Database ready at {}", db_path);

        // ----------------------------------------------------
        // MIGRATIONS
        // ----------------------------------------------------
        {
            let conn = pool.get()?;
            crate::db::run_migrations(&conn)?;
            info!("[CORE] Migrations applied");
        }

        // ----------------------------------------------------
        // CONTEXT (DB + PAIRING STATE + META)
        // ----------------------------------------------------
        let ctx = Arc::new(KernelContext::new(pool));

        // ----------------------------------------------------
        // EVENT BUS
        // ----------------------------------------------------
        let bus = Arc::new(EventBus::new());

        // ----------------------------------------------------
        // MODULES LADEN
        // ----------------------------------------------------
        let modules = Arc::new(load_modules());
        info!("[CORE] Loaded {} modules", modules.len());

        // ----------------------------------------------------
        // RUNTIME STATE
        // ----------------------------------------------------
        let state = KernelState {
            ctx,
            modules,
            bus,
        };

        // ----------------------------------------------------
        // mDNS DISCOVERY
        // ----------------------------------------------------
        start_mdns(2022, &state)?;

        // ----------------------------------------------------
        // HTTP SERVER (Axum 0.7)
// ----------------------------------------------------
        let app = build_http_router(state.clone());

        task::spawn(async move {
            let listener = TcpListener::bind("0.0.0.0:2022")
                .await
                .expect("Port bind failed");

            info!("[CORE] HTTP server running on port 2022");

            axum::serve(
    listener,
    app.into_make_service_with_connect_info::<std::net::SocketAddr>()
)
.await
.expect("HTTP crashed");

        });

        info!("[CORE] Kernel boot sequence complete.");
        Ok(())
    }
}
