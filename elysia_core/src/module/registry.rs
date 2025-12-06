// ======================================================================
// 📍 FILE: elysia_core/src/module/registry.rs
// ======================================================================

use super::ElysiaModule;
use std::collections::HashMap;
use std::sync::Arc;
use libloading::Library;

pub struct ModuleRegistry {
    modules: HashMap<String, Arc<dyn ElysiaModule>>,
    libs: Vec<Library>, // libraries levend houden
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            libs: Vec::new(),
        }
    }

    pub fn store_lib(&mut self, lib: Library) -> &Library {
    self.libs.push(lib);
    self.libs.last().unwrap()
}


    pub fn register_module(&mut self, module: Box<dyn ElysiaModule>) {
        let name = module.name().to_string();
        println!("[CORE][MODULE] Registered: {}", name);

        self.modules.insert(name, Arc::from(module));
    }

    pub fn iter(&self) -> impl Iterator<Item = Arc<dyn ElysiaModule>> + '_ {
        self.modules.values().cloned()
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }
}
