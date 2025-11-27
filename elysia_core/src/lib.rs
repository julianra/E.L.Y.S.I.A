// ======================================================================
// 📍 FILE: elysia/elysia_core/src/lib.rs
//
// 📝 BESCHRIJVING:
//   Het centrale toegangspunt van de ELYSIA Core library.
//   Dit bestand exporteert alle belangrijke structen en modules.
//
// 🔧 TAKEN:
//   - Herexporteert Kernel, ElysiaModule, Router, EventBus, Context
//   - Verbindt de interne modules van elysia_core
//   - Zorgt dat andere crates enkel `elysia_core` hoeven te importeren
// ======================================================================

pub mod kernel;
pub mod module;
pub mod context;
pub mod router;
pub mod events;
pub mod db_init;
pub mod db;


pub use crate::kernel::Kernel;
pub use crate::module::ElysiaModule;
pub use crate::context::KernelContext;
pub use crate::router::Router;
pub use crate::events::EventBus;
