//! API állapotának ellenőrzésére szolgáló végpont.

use axum::Json;
use serde_json::{json, Value};

#[inline]
pub async fn get() -> Json<Value> {
    Json(json!({
        "status": "active"
    }))
}
