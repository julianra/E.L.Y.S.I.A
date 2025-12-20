// ======================================================================
// 📍 FILE: elysia_core/src/plugins/mod.rs
//
// 📝 BESCHRIJVING:
//   Plugin subsystem voor ELYSIA. Laadt dynamic libraries (.dll/.so/.dylib)
//   en registreert modules op runtime. Bestaat uit:
//       - manifest parser
//       - plugin loader (libloading)
//       - plugin registry types
//
// ======================================================================

pub mod manifest;
pub mod loader;
pub mod reloader;
