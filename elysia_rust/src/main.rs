mod api;
mod kernel;
mod modules;

use tokio::{task, net::TcpListener};
use kernel::Kernel;
use api::api_router;

#[tokio::main]
async fn main() {
    println!("[CORE] Elysia Rust booting...");

    // 1) EventBus + Receiver
    let (bus, bus_rx) = Kernel::build_eventbus();

    // 2) API router
    let app = api_router(bus.clone());

    // 3) API server parallel
    task::spawn(async move {
        println!("[API] Listening on http://0.0.0.0:3000");

        let listener = TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("Kan poort 3000 niet openen");

        axum::serve(listener, app)
            .await
            .expect("API server crash");
    });

    // 4) Kernel starten
    let kernel = Kernel::new();
    kernel.run_with_bus(bus_rx).await;
}
