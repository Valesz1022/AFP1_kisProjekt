use crate::AppState;
use axum::{routing, Router};
use std::sync::Arc;

pub mod health_check;
pub mod saved;
pub mod users;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health_check", routing::get(health_check::get))
        .route(
            "/users",
            routing::get(users::get)
                .put(users::put)
                .delete(users::delete),
        )
        .route(
            "/saved",
            routing::get(saved::get)
                .put(saved::put)
                .delete(saved::delete),
        )
}
