//! Útvonalak végpontokhoz rendelése

use crate::AppState;
use axum::{routing, Router};
use std::sync::Arc;

pub mod health_check;
pub mod jokes;
pub mod saved;
pub mod users;
pub mod votes;

pub fn guest_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health_check", routing::get(health_check::get))
        .route("/users", routing::post(users::post))
        .route("/jokes", routing::get(jokes::get))
}

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health_check", routing::get(health_check::get))
        .route("/jokes", routing::get(jokes::get).post(jokes::post))
        .route(
            "/votes",
            routing::post(votes::post)
                .put(votes::put)
                .delete(votes::delete),
        )
        .route(
            "/saved",
            routing::get(saved::get)
                .post(saved::post)
                .delete(saved::delete),
        )
}

pub fn admin_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health_check", routing::get(health_check::get))
        .route(
            "/jokes",
            routing::get(jokes::get)
                .post(jokes::post)
                .delete(jokes::delete),
        )
        .route(
            "/votes",
            routing::post(votes::post)
                .put(votes::put)
                .delete(votes::delete),
        )
        .route(
            "/saved",
            routing::get(saved::get)
                .post(saved::post)
                .delete(saved::delete),
        )
}
