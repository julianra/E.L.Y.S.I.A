// ======================================================================
// 📍 FILE: elysia_core/src/module/registry.rs
//
// 📝 BESCHRIJVING:
//   Dit bestand beheert:
//     - module discovery (inventory crate)
//     - module loading
//     - lijst van actieve modules
//
//   Elke module gebruikt:
//
//       inventory::submit!(ModuleRegistration { module: || Box::new(MyModule {}) });
//
// ======================================================================

use crate::module::ElysiaModule;
use inventory::collect;

pub struct ModuleRegistration {
    pub module: fn() -> Box<dyn ElysiaModule>,
}

inventory::collect!(ModuleRegistration);

pub fn load_modules() -> Vec<Box<dyn ElysiaModule>> {
    let mut modules = vec![];

    for reg in inventory::iter::<ModuleRegistration> {
        modules.push((reg.module)());
    }

    modules
}
