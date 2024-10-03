//! API állapotát ellenőrző végpont

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::instrument;

#[instrument(name = "health_check::get")]
pub async fn get() -> impl IntoResponse {
    let payload = json!({
        "status": "active"
    });
    (StatusCode::OK, Json(payload)).into_response()
}
