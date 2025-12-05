// ======================================================================
// 📍 FILE: elysia_core/src/lib.rs
//
// 📝 BESCHRIJVING:
//   Het publieke toegangspunt van de ELYSIA Core library.
//   Exporteert enkel de noodzakelijke structen, traits en functies.
//
//   Dit is de OS API van ELYSIA Core:
//      - Kernel
//      - KernelState
//      - Module traits en registry
//      - EventBus
//      - Database initialisatie en migraties
//      - HTTP router
//      - mDNS discovery
//
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

// Re-exporten voor eenvoud
pub use kernel::Kernel;
pub use kernel::KernelState;
pub use module::ElysiaModule;
