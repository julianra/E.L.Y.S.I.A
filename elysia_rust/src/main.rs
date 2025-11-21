// Elysia Rust - main.rs
// ===============================================
// FILE: src/main.rs
// ROLE: Application Entry Point
// PART OF: Core Layer
// ===============================================

mod kernel;
mod api;
mod modules;

use kernel::Kernel;

#[tokio::main]
async fn main() {
    println!("[CORE] Elysia Rust booting...");

    let kernel = Kernel::new();
    kernel.run().await;
}
