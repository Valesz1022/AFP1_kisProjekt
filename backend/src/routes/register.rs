//! Felhasználók regisztrálását kiszolgáló végpont

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query;
use tracing::instrument;

use crate::AppState;

#[instrument(name = "register::post", skip(appstate))]
pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let password_hash = if let Some(password) = params.get("password") {
        password_auth::generate_hash(password)
    } else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query("INSERT INTO users (name, password) VALUES (?, ?);")
        .bind(Some(params.get("name")))
        .bind(password_hash)
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}
