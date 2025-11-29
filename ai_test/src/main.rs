// ======================================================================
// 📍 FILE: ai_test/src/main.rs
// ======================================================================

use elysia_ai::{
    AiKernel,
    intents::AiIntent,
    backends::ollama::OllamaBackend,
};

#[tokio::main]
async fn main() {
    println!("=== AI KERNEL TEST ===");

    // Backend met defaults
    let backend = OllamaBackend::new();
    let ai = AiKernel::new(Box::new(backend));

    let text = "ik moet morgen jari bellen over zijn eindwerk";

    match ai.send(AiIntent::MartheParseTask, text).await {
        Ok(res) => {
            println!("\n[AI RESPONSE]");
            println!("{res}");
        }
        Err(e) => {
            println!("\n[AI ERROR]");
            println!("{e}");
        }
    }

    println!("\n=== EINDE TEST ===");
}
