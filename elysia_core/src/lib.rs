// ======================================================================
// 📍 FILE: elysia_core/src/lib.rs
// 📝 Main library entrypoint for ELYSIA Core.
// ======================================================================

pub mod kernel;
pub use kernel::{Kernel, KernelState};

pub mod context;
pub mod db;
pub mod db_init;

pub mod events;

pub mod module;

pub mod pairing;

pub mod mdns;

pub mod kernel_api;

pub mod security;

pub mod plugins;
