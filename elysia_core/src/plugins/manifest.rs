// ======================================================================
// 📍 FILE: elysia_core/src/plugins/manifest.rs
//
// 📝 BESCHRIJVING:
//   PluginManifest beschrijft een plug-in module:
//       - naam
//       - versie
//       - beschrijving
//       - entrypoint (fn in de DLL)
//       - permissies
//
//   Dit manifest zit in: /plugins/<module>/plugin.toml
// ======================================================================

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub entrypoint: String,
    pub permissions: Option<Vec<String>>,
}
