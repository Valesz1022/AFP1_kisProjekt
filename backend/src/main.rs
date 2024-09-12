use axum::{routing, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::fmt;

use vicc_explorer::routes::health_check;

#[tokio::main]
async fn main() {
    fmt::init();

    let app = Router::new()
        .route("/health_check", routing::get(health_check::get))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address.");

    axum::serve(listener, app)
        .await
        .expect("Server stopped unexpectedly.");
}
