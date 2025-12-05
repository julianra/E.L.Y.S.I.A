// ======================================================================
// 📍 FILE: elysia_core/src/lib.rs
//
// 📝 BESCHRIJVING:
//   Publiek toegangspunt van de ELYSIA Core library.
//   Exporteert enkel de noodzakelijke structen, traits en functies.
//
//   OS API van ELYSIA Core:
//      - Kernel & KernelState
//      - Module-systeem (ElysiaModule + registry)
//      - EventBus
//      - Database init + migrations
//      - HTTP router
//      - mDNS discovery
//      - Auth (password hashing + login endpoints)
// ======================================================================

// Kernel
pub mod kernel;

// Modulesysteem
pub mod module;

// EventBus
pub mod events;

// DB
pub mod db;
pub mod db_init;

// HTTP
pub mod http;

// Context (metadata + db)
pub mod context;

// mDNS
pub mod mdns;

// Auth
pub mod auth;

// Pairing
pub mod pairing;

// Re-exporten voor eenvoud
pub use kernel::Kernel;
pub use kernel::KernelState;
pub use module::ElysiaModule;
