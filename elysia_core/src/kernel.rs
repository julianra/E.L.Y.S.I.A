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



pub struct Kernel {
    ctx: KernelContext,
    router: Router,
    bus: EventBus,
    modules: Vec<Box<dyn ElysiaModule>>,
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


    // 3. Kernel-object maken
    let mut kernel = Kernel {
        ctx: KernelContext::new(),
        router: Router::new(),
        bus: EventBus::new(),
        modules: vec![],
    };

    // 4. DB in context stoppen
    kernel.ctx.set_db(conn);


    // 5. Modules verzamelen
    for reg in inventory::iter::<crate::module::ModuleRegistration> {
        kernel.modules.push((reg.module)());
    }

    // 6. Modules init/eladen
    kernel.init_modules()?;
    kernel.start_modules();

    // 7. Dummy runtime
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
        log::info!("[CORE] Kernel is running.");
        log::info!("[CORE] Routes: {:?}", self.router.routes);
        log::info!("[CORE] Event handlers: {:?}", self.bus.handlers);
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
