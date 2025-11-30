// ======================================================================
// 📍 FILE: ai_test/src/main.rs
//
// 📝 Zelfstandig testprogramma om de AI-kernel uit te proberen
// ======================================================================

use std::sync::Arc;

use elysia_ai::{
    AiKernel,
    intents::AiIntent,
    backends::ollama::OllamaBackend,
};

#[tokio::main]
async fn main() {
    println!("=== AI KERNEL TEST ===");

    // Backend aanmaken (Ollama)
    let backend = Arc::new(OllamaBackend::new());

    // AI-kernel initialiseren (start health-check + autostart Ollama)
    let ai = AiKernel::new(backend);

    // Testzin
    let text = "ik moet morgen jari bellen over zijn eindwerk";

    // Intent uitvoeren
    match ai.send(AiIntent::MartheParseTask, text).await {
        Ok(response) => {
            println!("\n[AI RESPONSE]");
            println!("{}", response);
        }
        Err(e) => {
            println!("\n[AI ERROR]");
            println!("{e}");
        }
    }

    println!("\n=== EINDE TEST ===");
}
