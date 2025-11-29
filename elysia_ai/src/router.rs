// ======================================================================
// 📍 FILE: elysia_ai/src/router.rs
//
// 📝 BESCHRIJVING:
//   Router: AiIntent → prompt → backend.generate → output.
//   Fix: Trait-object ondersteuning via ?Sized.
//
// ======================================================================

use crate::backend::AiBackend;
use crate::errors::{AiError, AiResult};
use crate::intents::AiIntent;

pub struct AiRouter;

impl AiRouter {
    pub async fn handle_intent<B>(
        backend: &B,
        intent: AiIntent,
        payload: &str,
    ) -> AiResult<String>
    where
        B: AiBackend + ?Sized,
    {
        let prompt = match intent {
            AiIntent::MartheParseTask => format!(
                "Analyseer deze zin en geef JSON terug met: \
                 name, intent, person, topic, raw_time, time_normalized, \
                 urgency.\n\nZin: \"{}\"",
                payload
            ),

            AiIntent::MartheExtractKeywords => format!(
                "Extraheer kernwoorden en entities uit deze zin. \
                 Geef JSON terug met keywords[], persons[], topics[].\n\n\"{}\"",
                payload
            ),

            AiIntent::MarthePlanWithContext => format!(
                "Maak een planningsvoorstel op basis van deze data. \
                 Input is JSON.\n\n{}",
                payload
            ),

            _ => return Err(AiError::UnknownIntent),
        };

        backend.generate(&prompt).await
    }
}
