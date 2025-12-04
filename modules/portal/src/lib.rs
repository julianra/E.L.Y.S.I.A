// ======================================================================
// 📍 FILE: modules/portal/src/lib.rs
//
//  ⭐ PORTAL v1.0 — Remote Access Node for ELYSIA
//
//  ✔ Kernel module (health endpoint, pairing later)
//  ✔ register_module! macro
//  ✔ node_start() = async Axum server for remote access
// ======================================================================

use elysia_core::{ElysiaModule, Router, EventBus};
use elysia_core::KernelContext;
use elysia_core::events::KernelEvent;
use elysia_core::kernel::KernelState;
use elysia_core::register_module;


use log::info;

// ======================================================================
// ⭐ MODULE STRUCT
// ======================================================================

pub struct PortalModule;

impl Default for PortalModule {
    fn default() -> Self {
        PortalModule
    }
}

// Automatische registratie in ELYSIA via inventory
register_module!(PortalModule);

// ======================================================================
// ⭐ MODULE IMPLEMENTATIE
// ======================================================================

impl ElysiaModule for PortalModule {
    fn name(&self) -> &'static str {
        "portal"
    }

    fn init(&self, _ctx: &KernelContext) {
        info!("[PORTAL] Initialised");
    }

    fn register_routes(&self, router: &mut Router) {
        router.add_route("GET", "/portal/health");
        info!("[PORTAL] Registered route /portal/health");
    }

    fn register_event_handlers(&self, _bus: &mut EventBus) {
        // (Later) Pairing, key exchange, heartbeat events…
    }

    fn handle_event(&self, _state: &KernelState, _event: KernelEvent) {
        // future use
    }

    fn box_clone(&self) -> Box<dyn ElysiaModule> {
        Box::new(Self)
    }
}

// ======================================================================
// ⭐ PORTAL NODE SERVER (wordt gestart door Kernel)
// ======================================================================
//
// kernel/main.rs gebruikt:
//
//     portal::node_start().await
//
// ======================================================================

pub async fn node_start() -> Result<(), String> {
    use axum::{Router, routing::get};
    use tokio::net::TcpListener;

    let addr = "0.0.0.0:7070";

    info!("[PORTAL] Node starting on {}", addr);

    let app = Router::new()
        .route("/health", get(|| async { "PORTAL OK" }));

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| e.to_string())?;

    axum::serve(listener, app)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
