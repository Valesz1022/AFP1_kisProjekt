use std::{io, sync::Arc};

use axum::{extract::Request, serve::Serve, Router};
use configuration::Settings;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{debug, span, Level};
use uuid::Uuid;

pub mod configuration;
pub(crate) mod models;
pub(crate) mod routes;
pub mod telemetry;

/// A webszerver állapota, végpontokban elérhetővé téve referencia számlált
/// módon.
pub(crate) struct AppState {
    /// Az adatbázishoz való kommunikáláshoz kapcsolatok pool-ja.
    pub connection_pool: MySqlPool,
}

impl AppState {
    #[must_use]
    pub const fn new(connection_pool: MySqlPool) -> Self {
        Self { connection_pool }
    }
}

/// Tartalmazza a webszervert, könnyebbé téve annak futtatását, és felépítését.
#[non_exhaustive]
#[derive(Debug)]
pub struct Application {
    server: Serve<Router, Router>,
}

/// Az alkalmazás felépítése közben felmerülhető hibák.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BuildError {
    /// A megadott host-ra és port-ra való csatlakozás közben felmerülő hiba.
    /// Általában foglalt a port, ezért nem sikerült a csatlakozás.
    #[error("Failed to bind to address.")]
    BindAddress(#[from] io::Error),
}

impl Application {
    /// Az alkalmazás felépítése, konfigurálása.
    /// Csatlakozik az adatbázishoz, illetve a port-hoz.
    /// Konfigurációs beállításokat felhasználja.
    #[inline]
    pub async fn build(configuration: Settings) -> Result<Self, BuildError> {
        let connection_pool = MySqlPoolOptions::new()
            .connect_lazy_with(configuration.database.connect_options());

        let listener = {
            let address = format!(
                "{}:{}",
                configuration.application.host, configuration.application.port,
            );

            TcpListener::bind(address).await?
        };

        debug!("listening on {}", listener.local_addr()?);

        let app_state = Arc::new(AppState::new(connection_pool));

        let routes = Router::new()
            .nest("/", routes::guest_router())
            .nest("/", routes::user_router())
            .nest("/", routes::admin_router());

        let app = routes
            .layer(TraceLayer::new_for_http().make_span_with(
                |request: &Request| {
                    let request_id = Uuid::new_v4().to_string();

                    span! {
                        Level::DEBUG,
                        "request",
                        %request_id,
                        method = ?request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    }
                },
            ))
            .with_state(app_state);

        Ok(Self {
            server: axum::serve(listener, app),
        })
    }

    /// Elindíta a szervert, és futtatja, amíg az egy javíthatatlan hibát nem
    /// kap, ebben az esetben leáll.
    #[inline]
    pub async fn run_until_stopped(self) -> io::Result<()> {
        self.server.await
    }
}
