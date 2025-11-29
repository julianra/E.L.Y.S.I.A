// ======================================================================
// 📍 FILE: elysia_ai/src/errors.rs
//
// 📝 BESCHRIJVING:
//   Centrale fouttypes voor de AI-kernel.  
//   Alle AI-gerelateerde onderdelen gebruiken deze enums.
//
// 🔧 TAKEN:
//   - AiError enum
//   - AiResult<T>
//   - Wordt gebruikt in backend.rs, router.rs, lib.rs
//
// ======================================================================

use thiserror::Error;

pub type AiResult<T> = Result<T, AiError>;

#[derive(Error, Debug)]
pub enum AiError {

    // --------------------------------------------------------------
    // Backend fouten
    // --------------------------------------------------------------

    #[error("AI backend unreachable or failed: {0}")]
    Backend(String),

    #[error("AI backend returned non-success status code: {0}")]
    BackendStatus(String),

    // --------------------------------------------------------------
    // JSON/parsing fouten
    // --------------------------------------------------------------

    #[error("AI returned invalid or unparseable JSON: {0}")]
    InvalidResponse(String),

    #[error("AI returned an incomplete response")]
    IncompleteResponse,

    // --------------------------------------------------------------
    // Intent routing fouten
    // --------------------------------------------------------------

    #[error("Unknown AI intent was requested")]
    UnknownIntent,

    #[error("Missing prompt template for intent")]
    MissingPrompt,

    // --------------------------------------------------------------
    // Kernel errors
    // --------------------------------------------------------------

    #[error("Internal AI kernel error: {0}")]
    Internal(String),
}
