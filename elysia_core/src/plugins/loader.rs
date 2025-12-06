// ======================================================================
// 📍 FILE: elysia_core/src/plugins/loader.rs
// ======================================================================

use std::{fs, path::PathBuf};
use libloading::{Library, Symbol};

use crate::module::registry::ModuleRegistry;

use super::manifest::PluginManifest;

pub struct PluginInfo {
    pub manifest: PluginManifest,
    pub lib_path: PathBuf,
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

    /// Zoek plugin.toml bestanden in de plugins folder
    pub fn scan(&self) -> Vec<PluginInfo> {
        let mut found = vec![];

        if !self.plugins_dir.exists() {
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
                Err(_) => continue,
            };

            let manifest: PluginManifest = match toml::from_str(&text) {
                Ok(m) => m,
                Err(e) => {
                    println!("[PLUGIN] Fout in manifest {:?}: {}", manifest_path, e);
                    continue;
                }
            };

            let lib_path = Self::resolve_library_path(&dir, &manifest.entrypoint);

            found.push(PluginInfo {
                manifest,
                lib_path,
            });
        }

        found
    }

    /// Vind .dll / .so / .dylib
    fn resolve_library_path(dir: &PathBuf, entry: &str) -> PathBuf {
        #[cfg(target_os = "windows")]
        let filename = format!("{}.dll", entry);

        #[cfg(target_os = "linux")]
        let filename = format!("{}.so", entry);

        #[cfg(target_os = "macos")]
        let filename = format!("{}.dylib", entry);

        dir.join(filename)
    }

    /// Laad ALLE plugins
    pub unsafe fn load_all(
        &self,
        registry: &mut ModuleRegistry,
        plugins: Vec<PluginInfo>,
    ) -> anyhow::Result<()> {
        for pl in plugins {
            if !pl.lib_path.exists() {
                println!("[PLUGIN] Library ontbreekt: {:?}", pl.lib_path);
                continue;
            }

            println!(
                "[PLUGIN] Laden: {} ({})",
                pl.manifest.name, pl.manifest.version
            );

            let lib = match Library::new(&pl.lib_path) {
                Ok(l) => l,
                Err(e) => {
                    println!("[PLUGIN] Fout bij laden library: {:?}", e);
                    continue;
                }
            };

            // Definieer type van entrypoint
            type InitFn = extern "C" fn(&mut ModuleRegistry);

            let func: Symbol<InitFn> = match lib.get(b"elysia_register") {
                Ok(f) => f,
                Err(e) => {
                    println!("[PLUGIN] Entrypoint niet gevonden: {:?}", e);
                    continue;
                }
            };

            // Roep plugin init aan
            func(registry);
        }

        Ok(())
    }
}
