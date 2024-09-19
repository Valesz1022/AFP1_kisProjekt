//! API állapotát ellenőrző végpont

use axum::{http::StatusCode, response::IntoResponse};
use tracing::instrument;

#[instrument(name = "health_check::get")]
pub async fn get() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!").into_response()
}
