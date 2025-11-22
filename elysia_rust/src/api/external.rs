// ===============================================
// FILE: src/api/external.rs
// ===============================================

use axum::{Json, extract::State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDate;

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

// 👉 **ZET DIT BUITEN DE STRUCT**
impl ExternalAgendaRequest {
    pub fn parsed_date(&self) -> Option<NaiveDate> {
        match &self.date {
            Some(d) => {
                // 1) Eerst proberen exact "YYYY-MM-DD"
                if let Ok(date) = NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    return Some(date);
                }

                // 2) Dan proberen ISO-datetime → split at 'T'
                if let Some((date_part, _)) = d.split_once('T') {
                    if let Ok(date) = NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                        return Some(date);
                    }
                }

                None
            }
            None => None,
        }
    }
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
    
    bus.publish(Event::ExternalAgendaAdd(payload.clone())).await;

    Ok(Json(json!({
        "success": true,
        "message": "Agenda request received",
        "name": payload.name,
    })))
}
