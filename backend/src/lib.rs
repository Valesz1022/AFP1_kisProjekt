//! Webszerver a `vicc_explorer` backendjének.

use std::{io, sync::Arc};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    Json,
};
use axum_login::{
    login_required, permission_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use configuration::Settings;
use serde_json::json;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use thiserror::Error;
use time::Duration;
use tokio::{
    net::TcpListener,
    select, signal,
    task::{self, AbortHandle, JoinError},
};
use tower_http::trace::TraceLayer;
use tower_sessions::{cookie::{Key, SameSite}, session_store};
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

/// A webszerver indítása és futtatása közben felmerülhető problémák.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ServerError {
    /// A szerver nem tudott elindulni, mivel a konfigurációban megadott port
    /// már foglalt.
    #[error("Port is already in use.")]
    Port(io::Error),
    /// A szerver hibásan állt le.
    #[error("Server shut down unexpectedly.")]
    Shutdown(io::Error),
    /// Nem lehetett a hitelesítéshez szükséges adatbázis táblákat létrehozni.
    #[error("Couldn't run database migrations to initialize sessions.")]
    Migrations(#[from] sqlx::Error),
    /// Nem sikerült lefuttatni az adatbázis helyreállításához szükséges
    /// szálat.
    #[error("Couldn't run graceful shutdown task.")]
    Join(#[from] JoinError),
    /// Nem sikerült az adatbázist helyreállítani.
    #[error("Couldn't run cleaning up in the database.")]
    Session(#[from] session_store::Error),
}

impl Application {
    /// Az alkalmazás felépítése, konfigurálása és futtatása.
    ///
    /// Konfigurációs beállításokat felhasználja.
    /// Csatlakozik az adatbázishoz, illetve a port-hoz.
    /// Kezeli a hitelesítést, inicializálja az ahhoz szükséges dolgokat.
    /// Futtatja a webszervert, illetve annak megállása után törli az
    /// adabázisból a fölösleges dolgokat (pl. bejelentkezett felhasználók).
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
            .with_secure(true)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key)
            .with_same_site(SameSite::None);

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
            .layer(middleware::from_fn(add_cors))
            .with_state(app_state)
            .fallback(Self::fallback_handler);

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

    /// Akkor használjuk, ha egy olyan kérés érkezik, amelyet a router nem
    /// tud hozzákötni semmilyen handlerhöz. Ilyenkor a router fallback-ként
    /// (visszaesésként) meghívja ezt a függvényt.
    /// `Axum` már alapvetően csinál egy ilyet, de az nem megfelelő nekünk,
    /// mivel nem JSON formátumban küldi a válasz törzsét, mi pedig abban
    /// szeretnénk.
    async fn fallback_handler() -> (StatusCode, Json<serde_json::Value>) {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Endpoint not found"
            })),
        )
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

/// A CORS miatt engedélyezni kell minden domainről kéréseket, illetve minden
/// headert a kérésekben.
/// Nem a legbiztonságosabb, hiszen így minden domain ténylegesen minden headert
/// felküldhet, de sajnos más megoldás most nincs, mivel a frontend nem fut fix
/// IP címen.
/// Publikus, mivel Debianon hostoljuk, és van pár hiba linkerrel.
///
/// # Panics
/// Használja `unwrap()` metódust, tehát tudna panicelni, viszont ez csak abban
/// az esetben fut hibára, ha a CORS specifikáció megváltozik, és az
/// Access-Control-Allow-Origin vagy az Access-Control-Allow-Headers nem
/// lehet *, vagy a HTTP státuszkódok megváltoznak.
#[doc(hidden)]
pub async fn add_cors(request: Request, next: Next) -> impl IntoResponse {
    let mut response = next.run(request).await;

    response
        .headers_mut()
        .append("Access-Control-Allow-Origin", "*".try_into().unwrap());
    response
        .headers_mut()
        .append("Access-Control-Allow-Headers", "*".try_into().unwrap());

    response.headers_mut().append(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, DELETE, OPTIONS".try_into().unwrap(),
    );

    response
}
