mod kernel;
mod modules;

use kernel::Kernel;

#[tokio::main] // <-- NODIG om async fn main toe te laten
async fn main() {
    println!("[CORE] Elysia Rust booting...");

    let kernel = Kernel::new();
    kernel.run().await; // <-- BELANGRIJK: kernel run is async

    // Deze regel zal voorlopig nooit bereikt worden,
    // omdat de kernel blijft loopen.
}
