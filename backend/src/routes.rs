//! Útvonalak végpontokhoz rendelése

use crate::AppState;
use axum::{routing, Router};
use std::sync::Arc;

pub mod health_check;
pub mod jokes;
pub mod saved;
pub mod users;
pub mod votes;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health_check", routing::get(health_check::get))
        .route("/users", routing::get(users::get).post(users::post))
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
