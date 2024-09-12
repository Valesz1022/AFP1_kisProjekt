use axum::{routing, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    fmt::init();

    let app = Router::new()
        .route("/health_check", routing::get(health_check))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address.");

    axum::serve(listener, app)
        .await
        .expect("Server stopped unexpectedly.");
}

async fn health_check() -> &'static str {
    "Hello, world!"
}
