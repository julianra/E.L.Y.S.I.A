// ======================================================================
// 📍 FILE: elysia_core/src/module/mod.rs
//
// 📝 BESCHRIJVING:
//   Dit bestand definieert het basis ElysiaModule trait dat ALLE modules
//   moeten implementeren. Deze versie is future-proof, clean en eenvoudig.
//   Modules leveren:
//      - hun naam
//      - hun API-routes (via Advanced Modular Router)
//      - hun event handlers (optioneel)
//
//   Dit bestand re-exporteert ook de registry en router API.
// ======================================================================

pub mod registry;
pub mod router;

use crate::events::{KernelEvent};
use crate::kernel::KernelState;
use axum::Router;

// Basistype dat elke module moet implementeren.
pub trait ElysiaModule: Send + Sync {
    /// Unieke module-naam (bv: "marthe", "junk", "shield")
    fn name(&self) -> &'static str;

    /// Registreer alle API endpoints van deze module
    fn routes(&self) -> Router {
        Router::new()
    }

    /// Event handler — optioneel
    fn handle_event(&self, _state: &KernelState, _event: &KernelEvent) {}
}

impl Clone for Box<dyn ElysiaModule> {
    fn clone(&self) -> Box<dyn ElysiaModule> {
        panic!("Modules moeten niet gekloond worden; Kernel houdt ownership.")
    }
}
