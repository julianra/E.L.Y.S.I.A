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
You are a deterministic task-parser for a scheduling system.

OUTPUT RULES:
- Output ONLY STRICT VALID JSON.
- NO markdown, NO backticks, NO explanations.
- All fields MUST be present.
- title MUST NEVER be null or empty.
- title MUST be a short English action summary: "<Verb> <Target>".
- title MUST be derived from the meaning of the sentence, NOT copied from the whole input.
- Good examples:
    "Call Jari"
    "Discuss project"
    "Meet client"
    "Review report"

INPUT FORMAT:
You receive JSON:
{{
  "input": "<sentence>",
  "today_date": "YYYY-MM-DD"
}}

DATE RULES:
- Use "today_date" as absolute reference.
- "morgen"/"tomorrow" = today_date + 1 day.
- ALWAYS output an actual date in YYYY-MM-DD.

TIME & FLEXIBILITY RULES:
- If NO explicit time → "time": null, "flexible": true, "flex_window_days": 1
- If explicit time exists → "flexible": false, "flex_window_days": 0

KEYWORDS RULES:
- Lowercase
- Useful keywords only
- No duplicates

URGENCY RULES:
- If calling/meeting someone tomorrow → "high"
- Else → "normal"

STRICT JSON SCHEMA:
{{
  "title": string,                 // NEVER null
  "task_type": "call" | "todo" | "meeting" | "reminder",
  "description": string,
  "date": "YYYY-MM-DD",
  "time": "HH:MM" | null,
  "keywords": [string],
  "urgency": "low" | "normal" | "high" | "critical",
  "flexible": bool,
  "flex_window_days": number
}}

Return ONLY the JSON object.

INPUT JSON:
{wrapped_payload}
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
