//! Minden végpont ebbe a modulba kerül, almodulokra felbontva külön-külön
//! URL alapján. Egy-egy adott végponton belül az adott HTTP method nevével
//! található függvény a tényleges kontroller.

use std::sync::Arc;

use axum::{routing, Router};

use crate::AppState;

pub mod health_check;
pub mod users;

/// A végpontok csoportosítása, és egy útvonalvezetőbe tétele.
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/health_check", routing::get(health_check::get))
}
