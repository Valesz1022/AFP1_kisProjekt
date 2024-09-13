use std::{io, sync::Arc};

use anyhow::Context;
use axum::{routing, serve::Serve, Router};
use configuration::Settings;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::debug;

use crate::routes::health_check;

pub mod configuration;
pub(crate) mod routes;

pub(crate) struct AppState {
    pub connection_pool: MySqlPool,
}

impl AppState {
    #[must_use]
    pub const fn new(connection_pool: MySqlPool) -> Self {
        Self { connection_pool }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Application {
    server: Serve<Router, Router>,
}

impl Application {
    #[inline]
    pub async fn build(configuration: Settings) -> anyhow::Result<Self> {
        let connection_pool = MySqlPoolOptions::new()
            .connect_lazy_with(configuration.database.connect_options());

        let listener = {
            let address = format!(
                "{}:{}",
                configuration.application.host, configuration.application.port,
            );

            TcpListener::bind(address)
                .await
                .context("Failed to bind to address.")?
        };

        debug!(
            "listening on {}",
            listener
                .local_addr()
                .context("Failed to get local address from TCP listener.")?
        );

        let app_state = Arc::new(AppState::new(connection_pool));

        let app = Router::new()
            .route("/health_check", routing::get(health_check::get))
            .layer(TraceLayer::new_for_http())
            .with_state(app_state);

        Ok(Self {
            server: axum::serve(listener, app),
        })
    }

    #[inline]
    pub async fn run_until_stopped(self) -> io::Result<()> {
        self.server.await
    }
}
