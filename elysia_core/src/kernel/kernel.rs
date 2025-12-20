// ======================================================================
// 📍 FILE: elysia_core/src/kernel/kernel.rs
// 📝 ELYSIA Kernel boot sequence — no HTTP, pure core runtime.
// ======================================================================

use std::sync::Arc;
use log::info;
use tokio::task;

use crate::{
    context::KernelContext,
    db_init::init_database,
    db,
    events::EventBus,
    module::registry::ModuleRegistry,
    plugins::loader::PluginLoader,
    kernel_api::get_kernel_status,
    mdns::start_mdns,
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
        info!("[CORE] Booting ELYSIA Kernel 2.1…");

        // -------------------------
        // DATABASE INIT
        // -------------------------
        let (pool, db_path) = init_database()?;
        info!("[CORE] Database ready at {}", db_path);

        // -------------------------
        // MIGRATIONS
        // -------------------------
        {
            let conn = pool.get()?;
            db::run_migrations(&conn)?;
            info!("[CORE] Migrations applied");
        }

        // -------------------------
        // CONTEXT
        // -------------------------
        let ctx = Arc::new(KernelContext::new(pool));

        // -------------------------
        // EVENT BUS
        // -------------------------
        let bus = Arc::new(EventBus::new());

        // -------------------------
        // MODULE REGISTRY
        // -------------------------
        let mut registry = ModuleRegistry::new();

        // -------------------------
        // LOAD PLUGINS
        // -------------------------
        info!("[CORE] Scanning plugins/");
        let loader = PluginLoader::new("plugins");
        let mut found = loader.scan();

        unsafe {
            loader.load_all(&mut registry, &mut found)?;
        }

        info!("[CORE] Loaded {} modules", registry.len());

        // -------------------------
        // COMPOSE STATE
        // -------------------------
        let state = Arc::new(KernelState {
    ctx,
    bus,
    modules: Arc::new(registry),
});


        // -------------------------
        // MDNS DISCOVERY
        // -------------------------
        start_mdns(2022, &state)?;

        // -------------------------
        // KERNEL READY
        // -------------------------
        let status = get_kernel_status(&state);
        info!("[CORE] Kernel online with {} modules", status.modules);
        // -------------------------
// -------------------------
// LOCAL HTTP SERVER (Admin IPC)
// -------------------------
{
    let state_http = state.clone();

    task::spawn(async move {
        use axum::serve;
        use tokio::net::TcpListener;
        use std::net::SocketAddr;
        use crate::http::build_router;

        let addr = SocketAddr::from(([127, 0, 0, 1], 2022));

        let app = build_router(state_http.clone());

        let listener = TcpListener::bind(addr).await.unwrap();

        info!("[CORE] Local admin API on http://127.0.0.1:2022");

        serve(listener, app)
            .await
            .unwrap();
    });
}

        Ok(())
    }
}
