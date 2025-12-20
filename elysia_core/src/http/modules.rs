// ======================================================================
// 📍 FILE: elysia_core/src/http/modules.rs
// ======================================================================

use axum::response::IntoResponse;
use axum::Json;
use axum::http::StatusCode;
use std::sync::Arc;
use serde::Serialize;

use crate::KernelState;
use crate::plugins::loader::PluginLoader;
use crate::security::{get_module_state, set_module_paired};

#[derive(Serialize)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub installed: bool,
    pub loaded: bool,
    pub paired: bool,
}

pub async fn list_modules(state: Arc<KernelState>) -> Json<Vec<ModuleInfo>> {
    let loader = PluginLoader::new("plugins");
    let found = loader.scan();
    let registry = state.modules.read().await;
    let conn = state.ctx.db();

    let modules = found.into_iter().map(|pl| {
        let name = pl.manifest.name.clone();
        let id = name.to_lowercase();

        let loaded = registry.iter().any(|m| m.name().to_lowercase() == id);
        let paired = get_module_state(&conn, &id).ok().flatten().map(|m| m.paired).unwrap_or(false);

        ModuleInfo {
            id,
            name,
            kind: "module".into(),
            installed: true,
            loaded,
            paired,
        }
    }).collect();

    Json(modules)
}

pub async fn pair_module(
    state: Arc<KernelState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let id = id.to_lowercase();

    let exists = PluginLoader::new("plugins")
        .scan()
        .iter()
        .any(|p| p.manifest.name.to_lowercase() == id);

    if !exists {
        return StatusCode::NOT_FOUND;
    }

    if set_module_paired(&state.ctx.db(), &id, true).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

pub async fn unpair_module(
    state: Arc<KernelState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let id = id.to_lowercase();

    if set_module_paired(&state.ctx.db(), &id, false).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
