// ======================================================================
// 📍 FILE: elysia/kernel/src/main.rs
//
// 📝 BESCHRIJVING:
//   De MAIN executable van ELYSIA.  
//   Dit is het enige bestand dat wordt uitgevoerd met `cargo run -p kernel`.
//
// 🔧 TAKEN:
//   - Start de Elysia Kernel via elysia_core::Kernel
//   - Geeft fouten duidelijk weer
//   - Wordt quasi nooit gewijzigd (blijft altijd klein)
// ======================================================================

use elysia_core::Kernel;

fn main() {
    if let Err(e) = Kernel::boot_and_run() {
        eprintln!("[CORE] Fatal error: {e}");
        std::process::exit(1);
    }
}
