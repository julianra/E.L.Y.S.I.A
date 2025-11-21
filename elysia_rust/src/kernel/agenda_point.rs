// ===============================================
// FILE: src/kernel/agenda_point.rs
// ROLE: Agenda Point Definition
// PART OF: Kernel Layer
// PURPOSE:
// - Definitie van het AgendaPoint struct
// - Omzetten van externe requests naar AgendaPoints
// ===============================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::api::external::ExternalAgendaRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaPoint {
    pub id: String,
    pub name: String,
    pub duration_minutes: u32,
    pub created_at: DateTime<Utc>,

    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub priority: Option<String>,
    pub task_type: Option<String>,
    pub project: Option<String>,
    pub location: Option<String>,
    pub deadline: Option<DateTime<Utc>>,

    pub energy_cost: Option<u8>,
    pub category: Option<String>,
    pub recurrence: Option<String>,
    pub importance_score: Option<f32>,
    pub predicted_duration: Option<u32>,
    pub confidence_score: Option<f32>,
    pub emotional_load: Option<u8>,

    pub required_tools: Option<Vec<String>>,
    pub blocking_rules: Option<Vec<String>>,
    pub context_tags: Option<Vec<String>>,
    pub linked_tasks: Option<Vec<String>>,
}

impl AgendaPoint {
    pub fn from_external(req: ExternalAgendaRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            duration_minutes: req.duration_minutes.unwrap_or(30),
            created_at: Utc::now(),

            start_time: None,
            end_time: None,
            priority: Some(req.priority),
            task_type: Some(req.r#type),
            project: Some("external".into()),
            location: Some(req.location),
            deadline: None,

            energy_cost: None,
            category: None,
            recurrence: None,
            importance_score: None,
            predicted_duration: None,
            confidence_score: None,
            emotional_load: None,

            required_tools: None,
            blocking_rules: None,
            context_tags: None,
            linked_tasks: None,
        }
    }
}
