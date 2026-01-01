// ======================================================================
// 📍 FILE: elysia_core/src/kernel/kernel.rs
// 📝 ELYSIA Kernel boot sequence
//
// - Database init + migrations
// - Event bus
// - Plugin loading (hot-reloadable)
// - mDNS broadcast
// - Local admin HTTP API
// ======================================================================

use std::sync::Arc;

use log::info;
use tokio::sync::RwLock;
use tokio::task;

use crate::{
    context::KernelContext, db, db_init::init_database, events::EventBus,
    kernel_api::get_kernel_status, mdns::start_mdns, module::registry::ModuleRegistry,
    plugins::reloader::reload_plugins,
};

// ======================================================================
// Kernel State (shared across HTTP / mDNS / background tasks)
// ======================================================================

#[derive(Clone)]
pub struct KernelState {
    pub ctx: Arc<KernelContext>,
    pub bus: Arc<EventBus>,
    pub modules: Arc<RwLock<ModuleRegistry>>,
}

// ======================================================================
// Kernel
// ======================================================================

pub struct Kernel;

impl Kernel {
    pub async fn boot() -> anyhow::Result<()> {
        info!("[CORE] Booting ELYSIA Kernel 2.1…");

        // --------------------------------------------------
        // DATABASE INIT
        // --------------------------------------------------
        let (pool, db_path) = init_database()?;
        info!("[CORE] Database ready at {}", db_path);

        // --------------------------------------------------
        // MIGRATIONS
        // --------------------------------------------------
        {
            let conn = pool.get()?;
            db::run_migrations(&conn)?;
            info!("[CORE] Migrations applied");
        }

        // --------------------------------------------------
        // CONTEXT
        // --------------------------------------------------
        let ctx = Arc::new(KernelContext::new(pool));

        // --------------------------------------------------
        // EVENT BUS
        // --------------------------------------------------
        let bus = Arc::new(EventBus::new());

        // --------------------------------------------------
        // MODULE REGISTRY (HOT-RELOADABLE)
        // --------------------------------------------------
        let modules = Arc::new(RwLock::new(ModuleRegistry::new()));

        // --------------------------------------------------
        // INITIAL PLUGIN LOAD
        // --------------------------------------------------
        info!("[CORE] Loading plugins (initial scan)");
        reload_plugins(modules.clone(), "plugins").await?;

        // --------------------------------------------------
        // COMPOSE KERNEL STATE
        // --------------------------------------------------
        let state = Arc::new(KernelState {
            ctx,
            bus,
            modules: modules.clone(),
        });

        // --------------------------------------------------
        // MDNS DISCOVERY
        // --------------------------------------------------
        start_mdns(2022, &state)?;

        // --------------------------------------------------
        // KERNEL READY LOG
        // --------------------------------------------------
        let status = get_kernel_status(&state).await;
        info!("[CORE] Kernel online with {} modules", status.modules);

        // --------------------------------------------------
        // LOCAL HTTP ADMIN API
        // --------------------------------------------------
        {
            let state_http = state.clone();

            task::spawn(async move {
                use crate::http::build_router;
                use axum::serve;
                use std::net::SocketAddr;
                use tokio::net::TcpListener;

                let addr = SocketAddr::from(([127, 0, 0, 1], 2022));
                let app = build_router(state_http.clone());

                let listener = TcpListener::bind(addr)
                    .await
                    .expect("Failed to bind admin API");

                info!("[CORE] Local admin API on http://127.0.0.1:2022");

                serve(listener, app).await.expect("Admin API crashed");
            });
        }

        Ok(())
    }
}
