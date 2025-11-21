mod kernel;
mod api;
mod modules;

use kernel::Kernel;
use api::api_router;
use tokio::task;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("[CORE] Elysia Rust booting...");

    // Start de Kernel in een aparte async task
    task::spawn(async {
        let kernel = Kernel::new();
        kernel.run().await;
    });

    // API router
    let app: Router = api_router();

    // Adres
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("[API] HTTP server luistert op http://{}", addr);

    // Axum 0.7 server starten
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
