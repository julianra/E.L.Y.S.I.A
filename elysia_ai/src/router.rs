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
- “vrijdag”         → 2025-12-05
- “zondag”          → 2025-12-07
- “vorige vrijdag”  → 2025-11-28

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

----------------------------------------
MULTI-DAY RANGE RULES (VERY IMPORTANT):
----------------------------------------
If the user says ANYTHING implying repeated days:
- “elke dag”
- “iedere dag”
- “alle dagen”
- “elke weekdag”
- “van maandag tot vrijdag”
- “volgende week elke dag”
- “de hele week”
- “heel de week”
- “elke dag van volgende week”
- “doorheen de week”

Then you MUST interpret this as a RANGE.

RULES FOR RANGE CREATION:

1. Range Start:
- If “volgende week” is present → range_start = next Monday after today_date.
- If “elke weekdag” → range_start = next Monday.
- If “elke dag” → range_start = the next logical day from context.
- If “van maandag tot vrijdag” → range_start = the next Monday instance.

2. Range End:
- “elke dag” → 7 days after range_start.
- “elke weekdag” → the upcoming Friday of that same week.
- “volgende week” → Monday → Sunday of that next week.
- “van maandag tot vrijdag” → literal Friday.

3. Time of day:
If user says: “van 8 tot 4”
→ Use these times for ALL days in the range:
  exact_start = <range_start>T08:00:00+01:00
  exact_end   = <range_end>T16:00:00+01:00

4. NO DEADLINES for multi-day expressions.
- Set exact_start and exact_end directly.
- Do NOT generate deadline_end for ranges.

5. Duration:
If user gives a daily range “8 tot 4”, ignore duration detection.
Let the scheduler compute per day after expansion.
----------------------------------------
EXPLICIT TIME RANGE OVERRIDE (CRITICAL)
----------------------------------------
If the user gives an explicit time range like:
- “van 8 tot 4”
- “van 08:00 tot 16:00”
- “van 7u tot 12u”

Then DO NOT use implicit interpretations.
You MUST map the times EXACTLY:

- “8” or “8u” → 08:00
- “4” or “4u” → 16:00

Explicit ranges ALWAYS override morning/afternoon rules.

----------------------------------------
MULTI-DAY RANGE – WEEKDAY RULES (MANDATORY):
----------------------------------------
If the user says any variation of:

- “elke weekdag”
- “iedere weekdag”
- “alle weekdagen”
- “elke dag van volgende week”
- “volgende week elke weekdag”
- “de hele week werken”
- “heel de week”
- “van maandag tot vrijdag”

Then you MUST:

1. Range start = NEXT MONDAY after today_date.
   Example: if today_date = 2025-12-03 (Wednesday), next Monday = 2025-12-08.

2. Range end = THE SAME WEEK’S FRIDAY.
   Example: 2025-12-12.

3. Time block:
   - If user gives “van 8 tot 4”, apply these times for ALL days.

4. NEVER use this week’s Thursday/Friday if “volgende week” is mentioned.
   ALWAYS use next week’s Monday as the anchor.
   
   
   5. NEVER choose start dates inside the current week for "volgende week" expressions.

--------------------------------
TIME OF DAY INTERPRETATION:
--------------------------------
- “om 7 uur” → 07:00
- “om 7 uur 's morgens” / “om 7u AM” → 07:00
- “om 7 uur 's avonds” → 19:00
- “om 8 uur 's avonds” → 20:00

Implicit times:
- “ochtend”   = 09:00
- “namiddag”  = 15:00
- “avond”     = 19:00
- “nacht”     = 23:00
- “vanavond”  = today_date at 20:00 unless explicit time given

----------------------------------
DEADLINE LANGUAGE RULES:
----------------------------------
If user says:
- “moet klaar zijn tegen X uur”
- “tegen 20:00 klaar”
- “moet af zijn om X”
- “deadline om X”
- “klaar zijn met … voor X”
→ MUST create: "deadline_end".

If ALSO a duration is detected:
→ exact_start = deadline_end - duration.

If NO duration:
→ duration_minutes = 15 and leave exact_start null.

---------------------------------
DURATION RULES (Dutch parsing):
---------------------------------
Detect:
- “duurt 2 uur”
- “ongeveer 1 uur”
- “half uurtje”
- “30 minuten”
- “anderhalf uur”
- “2u werk”

Convert:
- 1 uur      → 60
- half uur   → 30
- anderhalf  → 90
- 2 uur / 2u → 120

Missing duration → duration_minutes = 15.

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
