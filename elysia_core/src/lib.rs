// ======================================================================
// 📍 FILE: elysia_core/src/lib.rs
// ======================================================================

pub mod kernel;
pub use kernel::{Kernel, KernelState};

pub mod context;
pub mod db;
pub mod db_init;

pub mod events;

pub mod module;
pub use module::ElysiaModule;

pub mod http;

pub mod auth;

pub mod pairing;

pub mod mdns;

pub use http::build_http_router;
