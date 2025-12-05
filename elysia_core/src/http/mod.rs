// ======================================================================
// 📍 FILE: elysia_core/src/http/mod.rs
//
// 📝 BESCHRIJVING:
//   Dit is de hoofdrouter van ELYSIA Core.  
//   Deze module combineert ALLE HTTP-routes in één uniforme API.
//
//   Inbegrepen:
//     • Core routes        (/health, /status, /nodes, meta)
//     • Auth routes        (/auth/* voor onboarding & login)
//     • Pairing routes     (/pairing/* voor device onboarding)
//     • Module routes      (/api/<module>/* dynamisch per module)
//
//   Belangrijk:
//     - Iedere .merge() gebruikt state.clone()  
//     - mount_module_routes gebruikt &KernelState (borrow)  
//     - Dit bestand exporteert `build_http_router` zodat kernel.rs 
//       deze functie kan gebruiken.
//
// ======================================================================

use axum::Router;
use crate::kernel::KernelState;

// Submodules
mod core_routes;
mod auth_routes;
mod pairing_routes;
mod module_router;

// Publieke exports
pub use core_routes::*;
pub use auth_routes::auth_routes;
pub use pairing_routes::pairing_routes;
pub use module_router::mount_module_routes;

// ----------------------------------------------------------------------
// 🏗️ build_http_router
//
// Combineert alle routers in één axum::Router.
// Deze functie wordt aangeroepen door Kernel::boot().
// ----------------------------------------------------------------------
pub fn build_http_router(state: KernelState) -> Router {
    let mut app = Router::new();

    // --------------------------------------------------
    // CORE ROUTES (health, status, meta, nodes)
    // --------------------------------------------------
    app = app.merge(core_routes::routes(state.clone()));

    // --------------------------------------------------
    // AUTH ROUTES (admin onboarding + login)
    // --------------------------------------------------
    app = app.merge(auth_routes(state.clone()));

    // --------------------------------------------------
    // PAIRING ROUTES (device onboarding)
    // --------------------------------------------------
    app = app.merge(pairing_routes(state.clone()));

    // --------------------------------------------------
    // MODULE ROUTES (/api/<module>/...) 
    // Belangrijk: &state → borrowed! (geen move)
    // --------------------------------------------------
    app = app.merge(mount_module_routes(&state));

    app
}
