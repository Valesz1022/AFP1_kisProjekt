use axum::{routing, Router};
use std::env::args;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::fmt;

use vicc_explorer::routes::health_check;

const DEFAULT_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    fmt::init();

    let app = Router::new()
        .route("/health_check", routing::get(health_check::get))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        args().nth(1).unwrap_or(DEFAULT_PORT.to_string())
    ))
    .await
    .expect("Failed to bind to address.");
    println!(
        "Listening on {:?}",
        listener.local_addr().expect("Address resolution failed.")
    );

    axum::serve(listener, app)
        .await
        .expect("Server stopped unexpectedly.");
}
