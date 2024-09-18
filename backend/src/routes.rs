//! Minden végpont ebbe a modulba kerül, almodulokra felbontva külön-külön
//! URL alapján. Egy-egy adott végponton belül az adott HTTP method nevével
//! található függvény a tényleges kontroller.

use axum::{routing, Router};
use sqlx::MySqlPool;
use std::sync::Arc;

pub mod health_check;
pub mod saved;
pub mod users;

pub fn router() -> Router<Arc<MySqlPool>> {
    Router::new()
        .route("/health_check", routing::get(health_check::get))
        .route(
            "/saved",
            routing::get(saved::get)
                .put(saved::put)
                .delete(saved::delete),
        )
}
