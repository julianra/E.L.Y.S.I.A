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

        let mut kernel = Kernel {
            ctx: KernelContext::new(),
            router: Router::new(),
            bus: EventBus::new(),
            modules: vec![],
        };

        // Laad ALLE modules die zichzelf hebben geregistreerd
for reg in inventory::iter::<crate::module::ModuleRegistration> {
    kernel.modules.push((reg.module)());
}

        kernel.init_modules()?;
        kernel.start_modules();
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
