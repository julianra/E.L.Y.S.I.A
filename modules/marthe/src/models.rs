// ======================================================================
// 📍 FILE: modules/marthe/src/models.rs
//
// 📝 Beschrijving:
//   Datamodel voor een agenda-item in MARTHE.
//   - bevat optionele start, end, duration, deadline
//   - bevat helperfuncties voor tijdslogica
// ======================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local, Duration, Timelike, TimeZone};
use log::{error};

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

    /// Deadline waarop deze taak ten laatste klaar moet zijn.
    /// ISO8601 / RFC3339 string (bv. "2025-11-29T18:00:00Z")
    #[serde(default)]
    pub deadline_end: Option<String>,
}

impl AgendaItem {
    pub fn default_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Probeert verschillende datetime-formaten te parsen naar DateTime<Local>.
 pub fn parse_datetime(s: &str) -> Option<DateTime<Local>> {
    // 1) Volledig correct RFC3339 (+01:00, Z, offsets, alles)
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Local));
    }

    // (OPTIONEEL) Hier kun je custom patterns zetten, maar is eigenlijk NIET nodig.
    // AI retourneert ALTIJD valide RFC3339.

    log::error!("[MARTHE] Failed to parse datetime '{}'", s);
    None
}

    /// Vult ontbrekende velden aan met defaults:
    /// - duration_minutes → 15 min
    /// - exact_start → nu, of morgen 09:00 als het na 22u is
    /// - exact_end → exact_start + duration
    pub fn fill_defaults(&mut self) {
        let now: DateTime<Local> = Local::now();

        if self.duration_minutes.is_none() {
            self.duration_minutes = Some(15);
        }

        if self.exact_start.is_none() {
            if now.hour() >= 22 {
                let tomorrow = now.date_naive().succ_opt().unwrap();
                let dt = tomorrow.and_hms_opt(9, 0, 0).unwrap();
                let local_dt = DateTime::<Local>::from_naive_utc_and_offset(
                    dt,
                    *Local::now().offset()
                );
                self.exact_start = Some(local_dt.to_rfc3339());
            } else {
                self.exact_start = Some(now.to_rfc3339());
            }
        }

        if self.exact_end.is_none() {
            if let Some(dur) = self.duration_minutes {
                if let Some(start_str) = &self.exact_start {
                    if let Some(start) = Self::parse_datetime(start_str) {
                        self.exact_end = Some((start + Duration::minutes(dur)).to_rfc3339());
                    }
                }
            }
        }
    }

    /// Checkt of twee taken elkaar overlappen.
    pub fn overlaps(&self, other: &AgendaItem) -> bool {
        let Some(a_start_str) = &self.exact_start else { return false };
        let Some(a_end_str)   = &self.exact_end   else { return false };
        let Some(b_start_str) = &other.exact_start else { return false };
        let Some(b_end_str)   = &other.exact_end   else { return false };

        let a_start = match Self::parse_datetime(a_start_str) {
            Some(v) => v,
            None => return false,
        };
        let a_end = match Self::parse_datetime(a_end_str) {
            Some(v) => v,
            None => return false,
        };
        let b_start = match Self::parse_datetime(b_start_str) {
            Some(v) => v,
            None => return false,
        };
        let b_end = match Self::parse_datetime(b_end_str) {
            Some(v) => v,
            None => return false,
        };

        !(a_end <= b_start || b_end <= a_start)
    }
}
