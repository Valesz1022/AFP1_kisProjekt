//! Útvonalak végpontokhoz rendelése.

use std::sync::Arc;

use axum::{routing, Router};

use crate::AppState;

pub mod health_check;
pub mod jokes;
pub mod login;
pub mod logout;
pub mod register;
pub mod saved;
pub mod votes;

/// A vendég felhasználók számára elérhető végpontok.
pub fn guest_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health_check", routing::get(health_check::get))
        .route("/register", routing::post(register::post))
        .route("/jokes", routing::get(jokes::get))
        .route("/login", routing::post(login::post))
}

/// A bejelentkezett felhasználók számára elérhető végpontok.
pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/jokes", routing::post(jokes::post))
        .route(
            "/votes",
            routing::get(votes::get)
                .post(votes::post)
                .put(votes::put)
                .delete(votes::delete),
        )
        .route(
            "/saved",
            routing::get(saved::get)
                .post(saved::post)
                .delete(saved::delete),
        )
        .route("/logout", routing::get(logout::get))
}

/// Az adminisztrátorok számára elérhető végpontok.
pub fn admin_router() -> Router<Arc<AppState>> {
    Router::new().route("/jokes", routing::delete(jokes::delete))
}
