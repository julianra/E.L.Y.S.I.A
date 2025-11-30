// ======================================================================
// 📍 FILE: elysia_ai/src/lib.rs
//
// 📝 AI-kernel met:
//     ✔ Health-check
//     ✔ Auto-start Ollama (alleen indien offline)
//     ✔ Bescherming tegen dubbele Ollama-services
// ======================================================================

pub mod intents;
pub mod backend;
pub mod router;
pub mod backends;
pub mod errors;

use std::sync::Arc;
use std::thread;
use std::process::{Command, Stdio};

use backend::AiBackend;
use backends::ollama::OllamaBackend;
use errors::AiResult;
use intents::AiIntent;
use router::AiRouter;

pub struct AiKernel {
    backend: Arc<dyn AiBackend>,
}

impl AiKernel {
    pub fn new(backend: Arc<dyn AiBackend>) -> Self {
        log::info!("[AI] Initialising AI-kernel…");

        let backend_clone = backend.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new()
                .expect("[AI] Failed to create Tokio runtime");

            rt.block_on(async move {
                if let Some(ollama) = backend_clone
                    .as_any()
                    .downcast_ref::<OllamaBackend>()
                {
                    // ------------------------------------------------------------
                    // 1) TEST OF OLLAMA AL DRAAIT
                    // ------------------------------------------------------------
                    let mut online = false;

                    for _ in 0..2 {
                        if ollama.list_models().await.is_ok() {
                            online = true;
                            break;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                    }

                    if online {
                        log::info!("[AI] Ollama is already running ✓");
                    } else {
                        // --------------------------------------------------------
                        // 2) OLLAMA IS OFFLINE → probeer te starten
                        //    Bescherming tegen dubbele processen:
                        //      - Start alleen als models() unreachable is
                        // --------------------------------------------------------
                        log::warn!("[AI] Ollama offline → starting 'ollama serve'…");

                        let start_res = Command::new("ollama")
                            .arg("serve")
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .spawn();

                        match start_res {
                            Ok(_) => log::info!("[AI] Ollama started ✓"),
                            Err(e) => {
                                log::error!("[AI] Failed to start Ollama: {}", e);
                            }
                        }

                        // --------------------------------------------------------
                        // 3) WACHT TOT OLLAMA HELAAS ECHT OPSTART (MAX 20s)
                        // --------------------------------------------------------
                        let mut attempts = 0;
                        loop {
                            if ollama.list_models().await.is_ok() {
                                log::info!("[AI] Ollama reachable ✓");
                                break;
                            }

                            attempts += 1;
                            if attempts >= 40 {
                                log::error!("[AI] Ollama failed to start (timeout)");
                                break;
                            }

                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        }
                    }

                    // ------------------------------------------------------------
                    // 4) MODEL-LIJST LOGGEN (indien mogelijk)
                    // ------------------------------------------------------------
                    match ollama.list_models().await {
                        Ok(models) => {
                            log::info!("[AI] AI backend ONLINE ✓");
                            log::info!("[AI] Beschikbare modellen:");
                            for m in models {
                                log::info!("       - {}", m);
                            }
                        }
                        Err(e) => {
                            log::warn!("[AI] Backend OFFLINE ({})", e);
                        }
                    }
                }
            });
        });

        Self { backend }
    }

    pub async fn send(
        &self,
        intent: AiIntent,
        payload: &str
    ) -> AiResult<String> {
        AiRouter::handle_intent(self.backend.as_ref(), intent, payload).await
    }
}
