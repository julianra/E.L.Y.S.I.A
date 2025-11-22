// ===============================================
// FILE: src/api/external.rs
// ===============================================

use axum::{Json, extract::State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use chrono::{NaiveDate, NaiveTime};

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
    pub exact_start: Option<String>,

    #[serde(default)]
    pub exact_end: Option<String>,

    #[serde(default)]
    pub duration_minutes: Option<u32>,

    #[serde(default = "default_priority")]
    pub priority: String,

    #[serde(default = "default_location")]
    pub location: String,
}

// -------------------------------
// Datum + tijd parser
// -------------------------------
impl ExternalAgendaRequest {
    pub fn parsed_date_and_time(&self) -> Option<(NaiveDate, NaiveTime)> {
        if let Some(d) = &self.date {
            // 1. ISO 8601 + Z → Flutter default
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(d) {
                return Some((dt.date_naive(), dt.time()));
            }

            // 2. "YYYY-MM-DDTHH:MM:SS.sss"
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(
                d,
                "%Y-%m-%dT%H:%M:%S%.3f"
            ) {
                return Some((dt.date(), dt.time()));
            }

            // 3. "YYYY-MM-DDTHH:MM:SS"
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(
                d,
                "%Y-%m-%dT%H:%M:%S"
            ) {
                return Some((dt.date(), dt.time()));
            }

            // 4. Alleen datum → tijd = 00:00
            if let Ok(nd) = NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                return Some((nd, NaiveTime::from_hms_opt(0, 0, 0).unwrap()));
            }
        }

        None
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
