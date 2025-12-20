// ======================================================================
// 📍 FILE: elysia_core/src/plugins/reloader.rs
// 📝 ROLE:
//   Hot-reload van plugins zonder kernel restart
//
//   - Scant plugins folder
//   - Bouwt NIEUWE ModuleRegistry
//   - Laadt DLLs
//   - Vervangt atomair de actieve registry
//
//   Fase 1:
//     - Volledige reload (geen partial unload)
// ======================================================================

use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;

use crate::module::registry::ModuleRegistry;
use crate::plugins::loader::PluginLoader;

pub async fn reload_plugins(
    modules: Arc<RwLock<ModuleRegistry>>,
    plugins_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    info!("[CORE][PLUGINS] Reload requested");

    let loader = PluginLoader::new(plugins_dir.as_ref());
    let mut found = loader.scan();

    let mut new_registry = ModuleRegistry::new();

    unsafe {
        loader.load_all(&mut new_registry, &mut found)?;
    }

    let count = new_registry.len();

    {
        let mut guard = modules.write().await;
        *guard = new_registry;
    }

    info!("[CORE][PLUGINS] Reload complete ({} modules)", count);
    Ok(())
}
