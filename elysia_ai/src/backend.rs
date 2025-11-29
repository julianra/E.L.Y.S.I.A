// ======================================================================
// 📍 FILE: elysia_ai/src/backend.rs
//
// 📝 BESCHRIJVING:
//   Backend abstractie van de AI-engine.  
//   Vandaag alleen Ollama, maar uitbreidbaar.
//
// ======================================================================

use async_trait::async_trait;
use crate::errors::{ AiResult};

#[async_trait]
pub trait AiBackend: Send + Sync {
    /// Voert een prompt uit via een backend (Ollama, LM Studio, cloud…)
    async fn generate(&self, prompt: &str) -> AiResult<String>;
}
