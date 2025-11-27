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

use crate::{KernelContext, Router, EventBus, ElysiaModule};
use thiserror::Error;
use crate::register_module;
use mdns_sd::ServiceDaemon; // alleen nodig voor het veld in Kernel
use tokio::runtime::Runtime;

pub struct Kernel {
    ctx: KernelContext,
    router: Router,
    bus: EventBus,
    modules: Vec<Box<dyn ElysiaModule>>,
    // We bewaren de mDNS-daemon zodat hij niet gedropt wordt.
    // Onderstreept om "unused field" warnings te vermijden voorlopig.
    _mdns: Option<ServiceDaemon>,
}

impl Default for CoreModule {
    fn default() -> Self {
        CoreModule
    }
}

// Automatische registratie
register_module!(CoreModule);

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("initialization error: {0}")]
    InitError(String),
}

impl Kernel {
    pub fn boot_and_run() -> Result<(), KernelError> {
        env_logger::init();
        log::info!("[CORE] Elysia Kernel booting...");

        // 1. DB initialiseren
        let (conn, path) = crate::db_init::init_database()
            .map_err(|e| KernelError::InitError(format!("DB init failed: {}", e)))?;
        log::info!("[CORE] Database initialized at {}", path);

        // 2. Migraties uitvoeren
        crate::db::run_migrations(&conn)
            .map_err(|e| KernelError::InitError(format!("Migration failed: {}", e)))?;
        log::info!("[CORE] Database migrations completed.");

        // 3. mDNS starten (fout = warning, geen hard fail)
        let mdns_handle = match crate::mdns::start_mdns(3000) {
            Ok(handle) => Some(handle),
            Err(e) => {
                log::warn!("[CORE] Failed to start mDNS: {}", e);
                None
            }
        };

        // 4. Kernel-object maken
        let mut kernel = Kernel {
            ctx: KernelContext::new(),
            router: Router::new(),
            bus: EventBus::new(),
            modules: vec![],
            _mdns: mdns_handle,
        };

        // 5. DB in context stoppen
        kernel.ctx.set_db(conn);

        // 6. Modules verzamelen
        for reg in inventory::iter::<crate::module::ModuleRegistration> {
            kernel.modules.push((reg.module)());
        }

        // 7. Modules init/eladen
        kernel.init_modules()?;
        kernel.start_modules();
// Start HTTP server in new thread
std::thread::spawn(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let app = crate::http::build_router();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        log::info!("[CORE] HTTP server running on port 3000");

        axum::serve(listener, app).await.unwrap();
    });
});

        // 8. Dummy runtime
        kernel.run_main_loop();

        Ok(())
    }

    fn init_modules(&mut self) -> Result<(), KernelError> {
        for module in &self.modules {
            log::info!("[CORE] Initializing {}", module.name());
            module.init(&mut self.ctx);
            module.register_routes(&mut self.router);
            module.register_event_handlers(&mut self.bus);
        }
        Ok(())
    }

    fn start_modules(&self) {
        for module in &self.modules {
            module.start_background_tasks(&self.ctx);
        }
    }

    fn run_main_loop(&self) {
    log::info!("[CORE] Kernel is running. Press CTRL+C to stop.");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

}

struct CoreModule;

impl ElysiaModule for CoreModule {
    fn name(&self) -> &'static str { "core" }

    fn init(&self, ctx: &mut KernelContext) {
        ctx.set_meta("version", "0.1.0");
        log::info!("[CORE-MODULE] CoreModule init done.");
    }

    fn register_routes(&self, router: &mut Router) {
        router.add_route("GET", "/health");
    }

    fn register_event_handlers(&self, bus: &mut EventBus) {
        bus.register_handler("kernel.started");
    }
}
