// ======================================================================
// 📍 FILE: elysia_ai/src/backends/ollama.rs
//
// 📝 Ollama backend met:
//     ✔ dynamische modelkeuze
//     ✔ standaardmodel: qwen2.5:7b-instruct
// ======================================================================

use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use futures_util::stream::StreamExt;
use std::any::Any;

use crate::backend::AiBackend;
use crate::errors::{AiError, AiResult};

const CHAT_URL: &str = "http://127.0.0.1:11434/api/chat";
const GEN_URL:  &str = "http://127.0.0.1:11434/api/generate";

// Beste algemene model voor JSON taken
const DEFAULT_MODEL: &str = "qwen2.5:7b-instruct";

pub struct OllamaBackend {
    pub client: Client,
}

impl OllamaBackend {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    /// Health-check
    pub async fn list_models(&self) -> Result<Vec<String>, String> {
        let resp = self.client
            .get("http://127.0.0.1:11434/api/tags")
            .send()
            .await
            .map_err(|e| format!("AI unreachable: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("HTTP {}", resp.status()));
        }

        let json = resp.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Invalid JSON: {e}"))?;

        let mut out = vec![];

        if let Some(arr) = json.get("models").and_then(|v| v.as_array()) {
            for m in arr {
                if let Some(name) = m.get("name").and_then(|v| v.as_str()) {
                    out.push(name.to_string());
                }
            }
        }

        Ok(out)
    }

    async fn try_generate(&self, prompt: &str) -> AiResult<String> {
        if let Ok(out) = self.call_chat(prompt, DEFAULT_MODEL).await {
            return Ok(out);
        }
        self.call_generate(prompt, DEFAULT_MODEL).await
    }

    async fn call_chat(&self, prompt: &str, model: &str) -> AiResult<String> {
        let payload = serde_json::json!({
            "model": model,
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
                "HTTP {} bij /api/chat", resp.status()
            )));
        }

        let mut out = String::new();
        let mut stream = resp.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result
                .map_err(|e| AiError::Backend(format!("Stream-fout: {e}")))?;

            if let Ok(s) = std::str::from_utf8(&chunk) {
                for line in s.lines() {
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

        if out.trim().is_empty() {
            return Err(AiError::IncompleteResponse);
        }

        Ok(out)
    }

    async fn call_generate(&self, prompt: &str, model: &str) -> AiResult<String> {
        let payload = serde_json::json!({
            "model": model,
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
                "HTTP {} bij /api/generate", resp.status()
            )));
        }

        let mut out = String::new();
        let mut stream = resp.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result
                .map_err(|e| AiError::Backend(format!("Stream-fout: {e}")))?;

            if let Ok(s) = std::str::from_utf8(&chunk) {
                for line in s.lines() {
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if let Some(content) = json.get("response") {
                            out.push_str(content.as_str().unwrap_or(""));
                        }
                    }
                }
            }
        }

        if out.trim().is_empty() {
            return Err(AiError::IncompleteResponse);
        }

        Ok(out)
    }
}

#[async_trait]
impl AiBackend for OllamaBackend {
    async fn generate(&self, prompt: &str) -> AiResult<String> {
        for attempt in 1..=3 {
            match self.try_generate(prompt).await {
                Ok(out) => return Ok(out),
                Err(e) => {
                    if attempt == 3 {
                        return Err(AiError::Backend(format!(
                            "Alle pogingen gefaald: {e}"
                        )));
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
        Err(AiError::Internal("Onbereikbare code".into()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
