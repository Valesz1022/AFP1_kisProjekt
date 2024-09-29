use std::{io, sync::Arc};

use axum::extract::Request;
use axum_login::{
    login_required, permission_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use configuration::Settings;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use thiserror::Error;
use time::Duration;
use tokio::{
    net::TcpListener,
    select, signal,
    task::{self, AbortHandle, JoinError},
};
use tower_http::trace::TraceLayer;
use tower_sessions::{cookie::Key, session_store};
use tower_sessions_sqlx_store::MySqlStore;
use tracing::{debug, span, Level};
use users::Backend;
use uuid::Uuid;

pub mod configuration;
pub(crate) mod models;
pub(crate) mod routes;
pub mod telemetry;
pub(crate) mod users;

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

/// A webszerver könnyebb felépítésére, futtatására szolgáló üres adatszerkezet.
#[non_exhaustive]
#[derive(Debug)]
pub struct Application;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Port is already in use.")]
    Port(io::Error),
    #[error("Server shut down unexpectedly.")]
    Shutdown(io::Error),
    #[error("Couldn't run database migrations to initialize sessions.")]
    Migrations(#[from] sqlx::Error),
    #[error("Couldn't run graceful shutdown task.")]
    Join(#[from] JoinError),
    #[error("Couldn't run cleaning up in the database.")]
    Session(#[from] session_store::Error),
}

impl Application {
    /// Az alkalmazás felépítése, konfigurálása.
    /// Csatlakozik az adatbázishoz, illetve a port-hoz.
    /// Konfigurációs beállításokat felhasználja.
    #[inline]
    pub async fn serve(configuration: Settings) -> Result<(), ServerError> {
        let connection_pool = MySqlPoolOptions::new()
            .connect_lazy_with(configuration.database.connect_options());

        // A bejelentkezett felhasználókat ugyanabba az adatbázisba mentjük
        // el, mint ahova a többi adatot.
        let session_store = MySqlStore::new(connection_pool.clone());

        // Létrehozza az `axum-login` számára szükséges táblákat az
        // adatbázisban.
        session_store.migrate().await?;

        let deletion_task =
            task::spawn(session_store.clone().continuously_delete_expired(
                tokio::time::Duration::from_secs(60),
            ));

        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        let backend = Backend::new(connection_pool.clone());
        let auth_layer =
            AuthManagerLayerBuilder::new(backend, session_layer).build();

        let app_state = Arc::new(AppState::new(connection_pool));

        let app = routes::admin_router()
            // csak admin jogosultsággal elérhető végpontok
            .route_layer(permission_required!(Backend, "admin")) 
            .merge(routes::user_router())
            // csak bejelentkezett felhasználók számára elérhető végpontok
            .route_layer(login_required!(Backend))
            .merge(routes::guest_router())
            .layer(auth_layer)
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

        let listener = {
            let address = format!(
                "{}:{}",
                configuration.application.host, configuration.application.port,
            );

            TcpListener::bind(address)
                .await
                .map_err(ServerError::Port)?
        };

        debug!(
            "listening on {}",
            listener.local_addr().map_err(ServerError::Port)?
        );

        // Amikor leáll a szerver, megvárja, hogy a jelenleg bejelentkezett
        // felhasználók törlésre kerüljenek.
        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal(
                deletion_task.abort_handle(),
            ))
            .await
            .map_err(ServerError::Shutdown)?;

        deletion_task.await??;

        Ok(())
    }

    /// A megkapott szál-leállító értéket meghívja, ha olyan jelet kap, amely
    /// leállítja a program futását. Ezzel ha le is áll a program, az adatbázis
    /// helyes állapotban lesz, sikeres tisztítások után.
    ///
    /// # Panics
    /// Ha olyan operációs rendszeren fut a szerver, ahol a jeleket nem sikerül
    /// kezelni, leáll a szerver azok kezelése nélkül, így az adatbázis 
    /// helytelen állapotban lesz.
    async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        select! {
            _ = ctrl_c => { deletion_task_abort_handle.abort() },
            _ = terminate => { deletion_task_abort_handle.abort() },
        }
    }
}
