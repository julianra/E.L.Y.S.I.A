// ======================================================================
// 📍 FILE: kernel/src/main.rs
//
// 📝 BESCHRIJVING:
//   Dit is de executable die ELYSIA Core start.
//   Enkel de zuivere Kernel op poort 2022.
//
// ======================================================================

use elysia_core::Kernel;
use tokio::runtime::Runtime;
use log::info;

fn main() {
    env_logger::init();

    info!("[CORE] Starting runtime…");

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        if let Err(e) = Kernel::boot().await {
            eprintln!("[CORE] Fatal error: {:?}", e);
            std::process::exit(1);
        }

        // Alive-loop (Kernel HTTP draait async)
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
}
