// ======================================================================
// 📍 FILE: elysia_core/src/kernel.rs
//
// 📝 Kernel boot sequence voor ELYSIA 2.1 (Dynamic Plugin Edition)
//     - Database + migrations
//     - KernelContext
//     - EventBus
//     - Dynamic Plugin Loader (.dll/.so/.dylib)
//     - ModuleRegistry
//     - mDNS discovery
//     - Axum HTTP server
// ======================================================================

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task;
use log::info;

use crate::{
    context::KernelContext,
    db_init::init_database,
    db,
    events::EventBus,
    http::build_http_router,
    mdns::start_mdns,
    module::registry::ModuleRegistry,
    plugins::loader::PluginLoader,
};

#[derive(Clone)]
pub struct KernelState {
    pub ctx: Arc<KernelContext>,
    pub bus: Arc<EventBus>,
    pub modules: Arc<ModuleRegistry>,
}

pub struct Kernel;

impl Kernel {
    pub async fn boot() -> anyhow::Result<()> {
        info!("[CORE] Booting ELYSIA Kernel 2.1 – Dynamic Plugin System…");

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
            db::run_migrations(&conn)?;
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
        // MODULE REGISTRY (dynamic)
        // ----------------------------------------------------
        let mut registry = ModuleRegistry::new();

        // ----------------------------------------------------
        // LOAD PLUGINS (.dll/.so/.dylib)
        // ----------------------------------------------------
        info!("[CORE] Scanning plugins/");
        let loader = PluginLoader::new("plugins");
        let found_plugins = loader.scan();

        info!("[CORE] Found {} plugins", found_plugins.len());

        unsafe {
            loader.load_all(&mut registry, found_plugins)?;
        }

        info!("[CORE] Loaded {} modules", registry.len());

        let modules = Arc::new(registry);

        // ----------------------------------------------------
        // RUNTIME STATE
        // ----------------------------------------------------
        let state = KernelState {
            ctx,
            bus,
            modules,
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
                app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
            )
            .await
            .expect("HTTP crashed");
        });

        info!("[CORE] Kernel boot sequence complete.");
        Ok(())
    }
}
