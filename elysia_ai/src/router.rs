// ======================================================================
// 📍 FILE: elysia_ai/src/router.rs
//
// 📝 AiIntent → Prompt Router (STRICT JSON + TODAY injection)
// ======================================================================

use crate::backend::AiBackend;
use crate::errors::{AiError, AiResult};
use crate::intents::AiIntent;
use serde_json::json;

pub struct AiRouter;

impl AiRouter {
    pub async fn handle_intent(
        backend: &dyn AiBackend,
        intent: AiIntent,
        payload: &str,
    ) -> AiResult<String> {

        // ============================================================
        // 1. TODAY dynamisch bepalen via RUST
        // ============================================================
        let today_date = chrono::Local::now().format("%Y-%m-%d").to_string();

        // ============================================================
        // 2. JSON payload die we naar AI sturen
        // ============================================================
        let wrapped_payload = json!({
            "input": payload,
            "today_date": today_date
        })
        .to_string();

        // ============================================================
        // 3. Intent → Prompt Template
        // ============================================================
        let prompt = match intent {

            // --------------------------------------------------------
            // MARTHE task parsing (STRICT JSON)
            // --------------------------------------------------------
            AiIntent::MartheParseTask => format!(
r#"
You are a deterministic Dutch → structured task parser.

GOAL:
Return a fully resolved task object ready for scheduling by the ELYSIA MARTHE planner.

STRICT OUTPUT RULES:
- Output ONLY VALID JSON.
- NO markdown, NO commentary, NO backticks.
- All fields MUST exist.
- All datetimes MUST be RFC3339 with timezone (e.g. "2025-12-07T20:00:00+01:00").
- Use 24h time format.
- Name MUST be a short English action summary: "<Verb> <Target>".
- description may be null.
- Keywords must be lowercase without duplicates.

-----------------------------
DATE INTERPRETATION (DUTCH):
-----------------------------
Use “today_date” as reference.

Basic keywords:
- vandaag = today_date
- morgen = +1 day
- overmorgen = +2 days
- gisteren = -1 day

Weekdays:
- maandag, dinsdag, woensdag, donderdag, vrijdag, zaterdag, zondag

Interpret rule:
- A bare weekday (“vrijdag”) = the FIRST upcoming instance of that weekday AFTER today_date.
- NEVER choose a past weekday for this week.

Examples:
If today_date = 2025-12-01 (Monday):
- “vrijdag”       → 2025-12-05
- “zondag”        → 2025-12-07
- “vorige vrijdag” → 2025-11-28

----------------------------------------------
“VOLGENDE WEEK <weekday>” RULES (IMPORTANT):
----------------------------------------------
“volgende week X” ALWAYS means:
- the FIRST upcoming weekday X that is AT LEAST 7 days after today_date,
- NEVER jump to next year unless 7+ days naturally moves into next year.
- DO NOT interpret “volgende week” as “next calendar week of next year”.

Examples (today_date = 2025-12-01):
- “volgende week zondag” → 2025-12-07
- “volgende week maandag” → 2025-12-08
- “volgende week vrijdag” → 2025-12-12

--------------------------------
TIME OF DAY INTERPRETATION:
--------------------------------
- “om 7 uur” → 07:00
- “om 7 uur 's morgens” / “om 7u AM” → 07:00
- “om 7 uur 's avonds” → 19:00
- “om 8 uur 's avonds” → 20:00

Implicit times:
- “ochtend” = 09:00
- “namiddag” = 15:00
- “avond” = 19:00
- “nacht” = 23:00
- “vanavond” = today_date at 20:00 (unless time stated)

----------------------------------
DEADLINE LANGUAGE RULES:
----------------------------------
If user says:
- “moet klaar zijn tegen X uur”
- “tegen 20:00 klaar”
- “moet af zijn om X”
- “deadline om X”
- “klaar zijn met … voor X”
→ This MUST create: "deadline_end"

If ALSO a duration is detected:
→ exact_start = deadline_end - duration

If NO duration detected:
→ duration_minutes = 15 (default)
→ exact_start may be null (planner will place it)

---------------------------------
DURATION RULES (Dutch parsing):
---------------------------------
Detect statements like:
- “duurt 2 uur”
- “ongeveer 1 uur”
- “half uurtje”
- “30 minuten”
- “anderhalf uur”
- “nog 2 uur werken”
- “2u werk”

Convert:
- 1 uur → 60 min
- half uur → 30 min
- anderhalf uur → 90 min
- 2 uur → 120 min
- 2u → 120 min

If duration missing → duration_minutes = 15

----------------------------------
OUTPUT JSON FORMAT (MANDATORY):
----------------------------------
{{
  "name": string,
  "task_type": "todo" | "call" | "meeting" | "reminder",
  "description": string | null,
  "exact_start": "YYYY-MM-DDTHH:MM:SS+TZ" | null,
  "exact_end": "YYYY-MM-DDTHH:MM:SS+TZ" | null,
  "duration_minutes": number | null,
  "deadline_end": "YYYY-MM-DDTHH:MM:SS+TZ" | null,
  "keywords": [string],
  "urgency": "low" | "normal" | "high" | "critical",
  "flexible": boolean
}}

------------------------------------
INPUT YOU MUST PARSE:
------------------------------------
{wrapped_payload}

Return ONLY the JSON object.
"#
),

            // --------------------------------------------------------
            // KEYWORD extraction
            // --------------------------------------------------------
            AiIntent::MartheExtractKeywords => format!(
r#"
Extract keywords. Output ONLY valid JSON.

{{
  "keywords": [string]
}}

Input:
{payload}
"#
            ),

            // --------------------------------------------------------
            // Planning with context
            // --------------------------------------------------------
            AiIntent::MarthePlanWithContext => format!(
r#"
You are given planning JSON.
Return ONLY updated planning as valid JSON.

Input:
{payload}
"#
            )
        };

        backend.generate(&prompt).await
    }
}
