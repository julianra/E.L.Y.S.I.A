// ===============================================
// FILE: src/api/external.rs
// ROLE: External API endpoints
// PART OF: API Layer
// PURPOSE:
// - Endpoints voor externe apps (web/Flutter)
// - Externe agenda-aanvragen ontvangen en doorgeven
// ===============================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum::{Json};
use axum::http::StatusCode;

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

// Default values
fn default_type() -> String { "normal".into() }
fn default_priority() -> String { "normal".into() }
fn default_location() -> String { "home".into() }

// -------------------------------
// Response object
// -------------------------------
pub struct ExternalAgendaResponse {
    pub id: Uuid,
    pub message: String,
}

// -------------------------------
// Handler voor business logic
// -------------------------------
pub fn handle_external_agenda_add(
    req: ExternalAgendaRequest,
) -> Result<ExternalAgendaResponse, String> {

    let id = Uuid::new_v4();

    Ok(ExternalAgendaResponse {
        id,
        message: format!("Agenda request '{}' ontvangen", req.name),
    })
}

// -------------------------------
// HTTP endpoint
// -------------------------------
pub async fn add_agenda_http(
    Json(payload): Json<ExternalAgendaRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {

    match handle_external_agenda_add(payload) {
        Ok(res) => Ok(Json(serde_json::json!({
            "success": true,
            "id": res.id.to_string(),
            "message": res.message,
        }))),

        Err(err) => {
            eprintln!("[EXTERNAL] Fout: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
