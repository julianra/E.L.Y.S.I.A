// ======================================================================
// 📍 FILE: elysia_core/src/plugins/loader.rs
// ======================================================================

use libloading::{Library, Symbol};
use std::{fs, path::PathBuf};

use super::manifest::PluginManifest;
use crate::module::registry::ModuleRegistry;

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub manifest: PluginManifest,
    pub lib_path: PathBuf,

    // Fase 1 status
    pub installed: bool,
    pub loaded: bool,
}

/// Dynamic Plugin Loader
pub struct PluginLoader {
    plugins_dir: PathBuf,
}

impl PluginLoader {
    pub fn new(base: impl Into<PathBuf>) -> Self {
        Self {
            plugins_dir: base.into(),
        }
    }

    /// --------------------------------------------------
    /// Scan plugins folder
    /// - Elke folder met plugin.toml = INSTALLED
    /// - DLL is hier NIET vereist
    /// --------------------------------------------------
    pub fn scan(&self) -> Vec<PluginInfo> {
        let mut found = vec![];

        if !self.plugins_dir.exists() {
            println!("[PLUGIN] Plugins directory bestaat niet");
            return found;
        }

        for entry in fs::read_dir(&self.plugins_dir).unwrap() {
            let dir = entry.unwrap().path();
            if !dir.is_dir() {
                continue;
            }

            let manifest_path = dir.join("plugin.toml");
            if !manifest_path.exists() {
                continue;
            }

            let text = match fs::read_to_string(&manifest_path) {
                Ok(t) => t,
                Err(e) => {
                    println!(
                        "[PLUGIN] Kan manifest niet lezen {:?}: {}",
                        manifest_path, e
                    );
                    continue;
                }
            };

            let manifest: PluginManifest = match toml::from_str(&text) {
                Ok(m) => m,
                Err(e) => {
                    println!("[PLUGIN] Fout in manifest {:?}: {}", manifest_path, e);
                    continue;
                }
            };

            let lib_path = Self::resolve_library_path(&dir, &manifest.entrypoint);

            println!(
                "[PLUGIN] Gevonden plugin: {} ({}), installed=true",
                manifest.name, manifest.version
            );

            found.push(PluginInfo {
                manifest,
                lib_path,
                installed: true,
                loaded: false,
            });
        }

        found
    }

    /// --------------------------------------------------
    /// Resolve platform-specific library path
    /// --------------------------------------------------
    fn resolve_library_path(dir: &PathBuf, entry: &str) -> PathBuf {
        #[cfg(target_os = "windows")]
        let filename = format!("{}.dll", entry);

        #[cfg(target_os = "linux")]
        let filename = format!("{}.so", entry);

        #[cfg(target_os = "macos")]
        let filename = format!("{}.dylib", entry);

        dir.join(filename)
    }

    /// --------------------------------------------------
    /// Load plugins
    /// - Probeert alleen DLL-lading
    /// - Plugins zonder DLL blijven INSTALLED
    /// --------------------------------------------------
    pub unsafe fn load_all(
        &self,
        registry: &mut ModuleRegistry,
        plugins: &mut Vec<PluginInfo>,
    ) -> anyhow::Result<()> {
        for pl in plugins.iter_mut() {
            if !pl.lib_path.exists() {
                println!(
                    "[PLUGIN] {} geïnstalleerd maar niet geladen (geen library)",
                    pl.manifest.name
                );
                continue;
            }

            println!(
                "[PLUGIN] Laden: {} ({})",
                pl.manifest.name, pl.manifest.version
            );

            // 1️⃣ Load DLL
            let lib = match unsafe { Library::new(&pl.lib_path) } {
                Ok(l) => l,
                Err(e) => {
                    println!(
                        "[PLUGIN] Fout bij laden library {}: {:?}",
                        pl.manifest.name, e
                    );
                    continue;
                }
            };

            // 2️⃣ Store DLL (prevent unload)
            let stored_lib: &Library = registry.store_lib(lib);

            // 3️⃣ Resolve entrypoint
            type InitFn = extern "C" fn(&mut ModuleRegistry);

            let func: Symbol<InitFn> = match unsafe { stored_lib.get(b"elysia_register") } {
                Ok(f) => f,
                Err(e) => {
                    println!(
                        "[PLUGIN] Entrypoint ontbreekt voor {}: {:?}",
                        pl.manifest.name, e
                    );
                    continue;
                }
            };

            // 4️⃣ Register module
            func(registry);

            pl.loaded = true;

            println!("[PLUGIN] {} succesvol geladen", pl.manifest.name);
        }

        Ok(())
    }
}
