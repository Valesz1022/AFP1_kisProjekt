//! Mentett viccek kezelését kiszolgáló végpont

use crate::{models::Joke, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::{query, query_as, MySql};
use std::{collections::HashMap, sync::Arc};
use tracing::instrument;

#[instrument(name = "saved::get", skip(appstate))]
pub async fn get(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query_as::<MySql, Joke>(
        "SELECT 
            jokes.id, 
            jokes.user_name, 
            jokes.content,
            COALESCE(CAST(SUM(votes.vote) AS SIGNED), 0) AS votes
        FROM saved
        INNER JOIN users ON saved.user_name = users.name
        INNER JOIN jokes ON saved.joke_id = jokes.id
        LEFT JOIN votes ON votes.joke_id = jokes.id
        WHERE users.name = ?
        GROUP BY jokes.id, jokes.user_name, jokes.content;",
    )
    .bind(&Some(params.get("user_name")))
    .fetch_all(&appstate.connection_pool)
    .await
    {
        Ok(jokes) => (StatusCode::OK, Json(jokes)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

#[instrument(name = "saved::post", skip(appstate))]
pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query("INSERT INTO saved (user_name, joke_id) VALUES (?, ?);")
        .bind(&Some(params.get("user_name")))
        .bind(&Some(params.get("joke_id")))
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

#[instrument(name = "saved::delete", skip(appstate))]
pub async fn delete(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query("DELETE FROM saved WHERE user_name = ? AND joke_id = ?;")
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
