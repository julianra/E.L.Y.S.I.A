// ======================================================================
// 📍 FILE: elysia_core/src/kernel_api/mod.rs
// 📝 Central entrypoint for internal Kernel API.
//     Provides pure data-access for Portal, Orbit, and modules via IPC.
// ======================================================================

pub mod status;
pub mod health;
pub mod meta;
pub mod nodes;

pub use status::*;
pub use health::*;
pub use meta::*;
pub use nodes::*;
