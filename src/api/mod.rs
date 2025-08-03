use axum::routing::Router;
use tokio::net::TcpListener;

pub mod endpoints;
pub use endpoints::Endpoint;

mod subscription;
pub use subscription::subscription;

pub mod prelude;

pub const API_PORT: u32 = 6226;

pub async fn run() {
    let listener = TcpListener::bind(format!("0.0.0.0:{API_PORT}")).await.unwrap();

    let router = Router::new()
        .route("/control/resume", endpoints::control::resume())
        .route("/control/pause", endpoints::control::pause());

    println!("[api/startup] serving on port {API_PORT}");
    if let Err(e) = axum::serve(listener, router).await {
        println!("[api] error: {e:?}");
    }
}

pub async fn listen() {
    let rx = &endpoints::CHANNEL.1;
    loop {
        println!("[api] {:?}", rx.recv().await);
    }
}
