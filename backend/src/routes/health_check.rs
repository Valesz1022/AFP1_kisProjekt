//! API állapotának ellenőrzésére szolgáló végpont.

use axum::Json;
use serde_json::{json, Value};
use tracing::instrument;

#[instrument(name = "Health check")]
pub async fn get() -> Json<Value> {
    Json(json!({
        "status": "active"
    }))
}
