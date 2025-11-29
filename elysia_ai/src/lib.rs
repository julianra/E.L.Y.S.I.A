// ======================================================================
// 📍 FILE: elysia_ai/src/lib.rs
//
// 📝 BESCHRIJVING:
//   Publieke interface van de AI-kernel.
//   Modules gebruiken AiKernel::send(intent, payload)
//
// ======================================================================

pub mod intents;
pub mod backend;
pub mod router;
pub mod backends;
pub mod errors;

use intents::AiIntent;
use backend::AiBackend;
use errors::AiResult;
use router::AiRouter;

pub struct AiKernel {
    backend: Box<dyn AiBackend>,
}

impl AiKernel {
    pub fn new(backend: Box<dyn AiBackend>) -> Self {
        Self { backend }
    }

    pub async fn send(&self, intent: AiIntent, payload: &str) -> AiResult<String> {
        AiRouter::handle_intent(self.backend.as_ref(), intent, payload).await
    }
}
