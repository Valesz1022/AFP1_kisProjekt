//! Felhasználók kezelését kiszolgáló végpont

use crate::{models::User, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{query, query_as, MySql};
use std::{collections::HashMap, sync::Arc};
use tracing::instrument;

#[instrument(name = "users::get", skip(appstate))]
pub async fn get(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query_as::<MySql, User>(
        "SELECT * FROM users
        WHERE name = ? AND password = ?;",
    )
    .bind(&Some(params.get("name")))
    .bind(&Some(params.get("password")))
    .fetch_all(&appstate.connection_pool)
    .await
    {
        Ok(users) => if users.len() == 0 {
            StatusCode::UNAUTHORIZED
        } else {
            StatusCode::OK
        }
        .into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(name = "users::post", skip(appstate))]
pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query("INSERT INTO users (name, password) VALUES (?, ?);")
        .bind(&Some(params.get("name")))
        .bind(&Some(params.get("password")))
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}
