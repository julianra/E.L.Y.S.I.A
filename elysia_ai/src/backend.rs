// ======================================================================
// 📍 FILE: elysia_ai/src/backend.rs
//
// 📝 BESCHRIJVING:
//   Backend abstractie van de AI-engine.
//   Maakt downcasting mogelijk via as_any().
// ======================================================================

use async_trait::async_trait;
use crate::errors::AiResult;
use std::any::Any;

#[async_trait]
pub trait AiBackend: Send + Sync {
    /// AI prompt uitvoeren
    async fn generate(&self, prompt: &str) -> AiResult<String>;

    /// Voor downcasting (om te detecteren of backend Ollama is)
    fn as_any(&self) -> &dyn Any;
}