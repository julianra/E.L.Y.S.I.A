// ======================================================================
// 📍 FILE: elysia_core/src/kernel_api/mod.rs
// 📝 Central entrypoint for internal Kernel API.
//     Provides pure data-access for Portal, Orbit, and modules via IPC.
// ======================================================================

pub mod health;
pub mod meta;
pub mod nodes;
pub mod status;

pub use health::*;
pub use meta::*;
pub use nodes::*;
pub use status::*;
