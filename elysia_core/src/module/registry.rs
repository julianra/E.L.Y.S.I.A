// ======================================================================
// 📍 FILE: elysia_core/src/module/registry.rs
// ======================================================================

use super::ElysiaModule;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ModuleRegistry {
    modules: HashMap<String, Arc<dyn ElysiaModule>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, module: Box<dyn ElysiaModule>) {
        let name = module.name().to_string();
        println!("[CORE][MODULE] Registered: {}", name);

        self.modules.insert(name, Arc::from(module));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn ElysiaModule>> {
        self.modules.get(name).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = Arc<dyn ElysiaModule>> + '_ {
        self.modules.values().cloned()
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }
}
