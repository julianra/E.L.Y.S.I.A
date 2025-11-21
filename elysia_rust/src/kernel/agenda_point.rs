use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaPoint {
    pub id: String,                     // UUID
    pub name: String,                   // Titel van de taak
    pub duration_minutes: u32,          // Duur in minuten
    pub created_at: DateTime<Utc>,      // Aanmaakdatum

    // Planning data
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub priority: Option<String>,
    pub task_type: Option<String>,
    pub project: Option<String>,
    pub location: Option<String>,
    pub deadline: Option<DateTime<Utc>>,

    // AI / behavior
    pub energy_cost: Option<u8>,        // 1 - 10
    pub category: Option<String>,
    pub recurrence: Option<String>,
    pub importance_score: Option<f32>,
    pub predicted_duration: Option<u32>,
    pub confidence_score: Option<f32>,
    pub emotional_load: Option<u8>,

    // Metadata
    pub required_tools: Option<Vec<String>>,
    pub blocking_rules: Option<Vec<String>>,
    pub context_tags: Option<Vec<String>>,
    pub linked_tasks: Option<Vec<String>>,
}
