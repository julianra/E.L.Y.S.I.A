// ======================================================================
// 📍 FILE: elysia_ai/src/backends/ollama.rs
//
// 📝 BESCHRIJVING:
//   Ollama backend voor de AI-kernel.
//   - Probeert eerst /api/chat
//   - Valt terug op /api/generate
//   - Ondersteunt NDJSON streaming
//
// ======================================================================

use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use futures_util::stream::StreamExt;


use crate::backend::AiBackend;
use crate::errors::{AiError, AiResult};

const CHAT_URL: &str = "http://127.0.0.1:11434/api/chat";
const GEN_URL:  &str = "http://127.0.0.1:11434/api/generate";

pub struct OllamaBackend {
    client: Client,
}

impl OllamaBackend {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Probeert eerst /api/chat, dan /api/generate
    async fn try_generate(&self, prompt: &str) -> AiResult<String> {
        // 1) Probeer nieuw endpoint
        if let Ok(out) = self.call_chat(prompt).await {
            return Ok(out);
        }

        // 2) Fallback naar oude endpoint
        self.call_generate(prompt).await
    }

    // ------------------------------------------------------------
    // /api/chat
    // ------------------------------------------------------------
    async fn call_chat(&self, prompt: &str) -> AiResult<String> {
        let payload = serde_json::json!({
            "model": "phi3.5",
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        let resp = self.client
            .post(CHAT_URL)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AiError::Backend(format!("HTTP fout: {e}")))?;

        if !resp.status().is_success() {
            return Err(AiError::BackendStatus(format!(
                "HTTP {} bij /api/chat",
                resp.status()
            )));
        }

        let mut out = String::new();
        let mut stream = resp.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result
                .map_err(|e| AiError::Backend(format!("Stream-fout: {e}")))?;

            if let Ok(s) = std::str::from_utf8(&chunk) {
                for line in s.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if let Some(msg) = json.get("message") {
                            if let Some(content) = msg.get("content") {
                                out.push_str(content.as_str().unwrap_or(""));
                            }
                        }
                    }
                }
            }
        }

        if out.is_empty() {
            return Err(AiError::IncompleteResponse);
        }

        Ok(out)
    }

    // ------------------------------------------------------------
    // /api/generate
    // ------------------------------------------------------------
    async fn call_generate(&self, prompt: &str) -> AiResult<String> {
        let payload = serde_json::json!({
            "model": "phi3.5",
            "prompt": prompt
        });

        let resp = self.client
            .post(GEN_URL)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AiError::Backend(format!("HTTP fout: {e}")))?;

        if !resp.status().is_success() {
            return Err(AiError::BackendStatus(format!(
                "HTTP {} bij /api/generate",
                resp.status()
            )));
        }

        let mut out = String::new();
        let mut stream = resp.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result
                .map_err(|e| AiError::Backend(format!("Stream-fout: {e}")))?;

            if let Ok(s) = std::str::from_utf8(&chunk) {
                for line in s.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if let Some(content) = json.get("response") {
                            out.push_str(content.as_str().unwrap_or(""));
                        }
                    }
                }
            }
        }

        if out.is_empty() {
            return Err(AiError::IncompleteResponse);
        }

        Ok(out)
    }
}

#[async_trait]
impl AiBackend for OllamaBackend {
    async fn generate(&self, prompt: &str) -> AiResult<String> {
        // 3 retries zoals Python
        for attempt in 1..=3 {
            match self.try_generate(prompt).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == 3 {
                        return Err(AiError::Backend(format!(
                            "Alle pogingen gefaald: {e}"
                        )));
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        }

        Err(AiError::Internal(
            "Onbereikbare code in generate()".into(),
        ))
    }
}
