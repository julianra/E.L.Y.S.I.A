// ======================================================================
// 📍 FILE: elysia_core/src/http/ai.rs
// 📝 ROLE:
//   AI HTTP endpoint (FASE 1 — forward stub)
//
//   - Dit bestand vormt de ENIGE AI-gerelateerde HTTP-boundary
//     binnen de Kernel.
//   - Bevat GEEN AI-logica
//   - Bevat GEEN module-logica
//   - Bevat GEEN beslissingen
//
//   FASE 1:
//     - Strict request/response contract
//     - Forward-only stub naar externe AI-service
//     - Volledige validatie van AI-response
//
//   FASE 2+:
//     - Capability checks
//     - CARE / ORBIT routing
//     - Multi-node AI selectie
// ======================================================================

use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::KernelState;

// ======================================================================
// REQUEST / RESPONSE CONTRACTS
// ======================================================================

#[derive(Deserialize)]
pub struct AiExecuteRequest {
    pub question: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct AiExecuteResponse {
    pub reply: Option<String>,
    pub error: Option<String>,
}

// ======================================================================
// AI EXECUTE — STRICT FORWARD STUB
// ======================================================================

pub async fn ai_execute(
    _state: Arc<KernelState>,
    Json(_req): Json<AiExecuteRequest>,
) -> Json<AiExecuteResponse> {
    // ------------------------------------------------------------------
    // NOTE:
    // - `_req` is bewust ongebruikt in Fase 1
    // - Dit endpoint valideert STRUCTUUR, niet INHOUD
    // - Geen aannames, geen shortcuts
    // ------------------------------------------------------------------

    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "input": _req.question
    });

    log::info!(
        "[CORE][AI] Forward request | question=\"{}\" mode=\"{}\"",
        _req.question,
        _req.mode
    );

    let response = client
        .post("http://127.0.0.1:8123/generate")
        .json(&payload)
        .send()
        .await;

    let resp = match response {
        Ok(r) => r,
        Err(e) => {
            log::error!("[CORE][AI] AI unreachable: {}", e);
            return Json(AiExecuteResponse {
                reply: None,
                error: Some("AI unreachable".into()),
            });
        }
    };

    let json: serde_json::Value = match resp.json().await {
        Ok(j) => j,
        Err(_) => {
            log::error!("[CORE][AI] Invalid JSON from AI");
            return Json(AiExecuteResponse {
                reply: None,
                error: Some("Invalid AI JSON".into()),
            });
        }
    };

    log::info!("[CORE][AI] Raw AI response: {}", json);

    // ------------------------------------------------------------------
    // STRICT CONTRACT VALIDATION
    // ------------------------------------------------------------------

    let success = json
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let reply = json.get("reply").and_then(|v| v.as_str());

    let error = json.get("error").and_then(|v| v.as_str());

    if !success {
        log::error!("[CORE][AI] AI reported failure: {:?}", error);

        return Json(AiExecuteResponse {
            reply: None,
            error: error.map(|e| e.to_string()),
        });
    }

    let reply = match reply {
        Some(r) => r.to_string(),
        None => {
            log::error!("[CORE][AI] Contract violation: success=true but reply missing");
            return Json(AiExecuteResponse {
                reply: None,
                error: Some("Invalid AI response contract".into()),
            });
        }
    };

    log::info!(
        "[CORE][AI] Forwarding reply to caller | reply=\"{}\"",
        reply
    );

    Json(AiExecuteResponse {
        reply: Some(reply),
        error: None,
    })
}
