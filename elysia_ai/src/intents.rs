// ======================================================================
// 📍 FILE: elysia_ai/src/intents.rs
//
// 📝 BESCHRIJVING:
//   AI-intents voor MARTHE.  
//   Later uitbreidbaar.
//
// ======================================================================

#[derive(Debug, Clone)]
pub enum AiIntent {

    /// Natuurlijke taal → taakconcept
    MartheParseTask,

    /// Kernwoorden / entities extraheren
    MartheExtractKeywords,

    /// Planningsadvies genereren met (mogelijke) JUNK-context
    MarthePlanWithContext,
}
