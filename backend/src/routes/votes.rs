use crate::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query;
use std::{collections::HashMap, sync::Arc};

pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query(
        "INSERT INTO votes (user_name, joke_id, vote) VALUES (?, ?, ?);",
    )
    .bind(&Some(params.get("user_name")))
    .bind(&Some(params.get("joke_id")))
    .bind(&Some(params.get("vote")))
    .execute(&appstate.connection_pool)
    .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

pub async fn put(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query(
        "UPDATE votes SET vote = ? WHERE user_name = ? AND joke_id = ?;",
    )
    .bind(&Some(params.get("user_name")))
    .bind(&Some(params.get("joke_id")))
    .bind(&Some(params.get("vote")))
    .execute(&appstate.connection_pool)
    .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

pub async fn delete(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query("DELETE FROM votes WHERE user_name = ? AND joke_id = ?;")
        .bind(&Some(params.get("user_name")))
        .bind(&Some(params.get("joke_id")))
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}
