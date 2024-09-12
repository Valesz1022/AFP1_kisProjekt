use axum::{routing, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_subscriber::fmt;
use vicc_explorer::{configuration::Settings, routes::health_check};

#[tokio::main]
async fn main() {
    fmt::init();

    let config = Settings::parse().unwrap();

    let app = Router::new()
        .route("/health_check", routing::get(health_check::get))
        .layer(TraceLayer::new_for_http());

    let listener = {
        let address =
            format!("{}:{}", config.application.host, config.application.port,);

        TcpListener::bind(address)
            .await
            .expect("Failed to bind to address.")
    };

    debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Server stopped unexpectedly.");
}
