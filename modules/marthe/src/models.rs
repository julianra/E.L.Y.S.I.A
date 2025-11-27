// ======================================================================
// 📍 FILE: modules/marthe/src/models.rs
// ======================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local, Duration, Timelike};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgendaItem {
    #[serde(default = "AgendaItem::default_id")]
    pub id: String,

    pub name: String,

    #[serde(default)]
    pub duration_minutes: Option<i64>,

    #[serde(default)]
    pub exact_start: Option<String>,

    #[serde(default)]
    pub exact_end: Option<String>,

    #[serde(default)]
    pub priority: Option<String>,

    #[serde(default)]
    pub location: Option<String>,

    #[serde(default)]
    pub energy_cost: Option<i64>,
}

impl AgendaItem {
    pub fn default_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn fill_defaults(&mut self) {
        let now: DateTime<Local> = Local::now();

        if self.duration_minutes.is_none() {
            self.duration_minutes = Some(15);
        }

        if self.exact_start.is_none() {
            // logica: als het na 22u is → morgen om 9u
            if now.hour() >= 22 {
                let tomorrow = now.date_naive().succ_opt().unwrap();
                let dt = tomorrow.and_hms_opt(9, 0, 0).unwrap();
                self.exact_start = Some(
    DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset()).to_rfc3339()
);

            } else {
                self.exact_start = Some(now.to_rfc3339());

            }
        }

        if self.exact_end.is_none() {
            if let Some(dur) = self.duration_minutes {
                let start = match DateTime::parse_from_rfc3339(self.exact_start.as_ref().unwrap()) {
    Ok(dt) => dt.with_timezone(&Local),
    Err(e) => {
        log::error!("[MARTHE] Failed to parse exact_start: {}", e);
        return;
    }
};


                self.exact_end = Some((start + Duration::minutes(dur)).to_string());
            }
        }
    }
}
