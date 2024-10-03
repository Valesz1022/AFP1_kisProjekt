//! Felhasználók regisztrálását kiszolgáló végpont

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use sqlx::query;
use tracing::instrument;

use crate::AppState;

#[instrument(name = "register::post", skip(appstate))]
pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(name) = params.get("name") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };
    let password_hash = if let Some(password) = params.get("password") {
        password_auth::generate_hash(password)
    } else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query("INSERT INTO users (name, password) VALUES (?, ?);")
        .bind(name)
        .bind(password_hash)
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::CREATED.into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => 
                (StatusCode::CONFLICT, Json(db_err.to_string())).into_response(),
            _ => 
                StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
