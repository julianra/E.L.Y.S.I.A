// ===============================================
// FILE: src/api/external.rs
// ===============================================

use axum::{Json, extract::State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::kernel::event_bus::EventBus;
use crate::kernel::events::Event;

// -------------------------------
// Request van externe apps
// -------------------------------
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExternalAgendaRequest {
    pub name: String,

    #[serde(default = "default_type")]
    pub r#type: String,

    #[serde(default)]
    pub date: Option<String>,

    #[serde(default)]
    pub duration_minutes: Option<u32>,

    #[serde(default = "default_priority")]
    pub priority: String,

    #[serde(default = "default_location")]
    pub location: String,
}

fn default_type() -> String { "normal".into() }
fn default_priority() -> String { "normal".into() }
fn default_location() -> String { "home".into() }

// -------------------------------
// HTTP endpoint
// -------------------------------
pub async fn add_agenda_http(
    State(bus): State<EventBus>,
    Json(payload): Json<ExternalAgendaRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    
    // 🔥 Event publiceren
    bus.publish(Event::ExternalAgendaAdd(payload.clone())).await;

    Ok(Json(json!({
        "success": true,
        "message": "Agenda request received",
        "name": payload.name,
    })))
}
