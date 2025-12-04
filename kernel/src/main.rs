// ======================================================================
// 📍 FILE: kernel/src/main.rs
// ======================================================================

use elysia_core::Kernel;
use marthe as _;
use portal as _;
use portal::node_start;

use log::info;
use tokio::runtime::Runtime;

fn main() {
    env_logger::init();

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        // Kernel starten
        tokio::spawn(async {
            if let Err(e) = Kernel::boot_and_run().await {
                eprintln!("[CORE] Fatal error: {e}");
                std::process::exit(1);
            }
        });

        // Portal starten
        tokio::spawn(async {
            info!("[CORE] Spawning PORTAL node on port 7070…");

            loop {
                match node_start().await {
                    Ok(_) => {
                        info!("[PORTAL] Node exited normally");
                        break;
                    }
                    Err(e) => {
                        eprintln!("[PORTAL] Crashed: {e} — restarting in 1s");
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        });

        // Alive-loop
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
}
