use crate::{models::Joke, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::{query, query_as, MySql};
use std::{collections::HashMap, sync::Arc};

pub async fn get(State(appstate): State<Arc<AppState>>) -> impl IntoResponse {
    match query_as::<MySql, Joke>(
        "SELECT
             jokes.id, 
             jokes.user_name, 
             jokes.content,
             COALESCE(CAST(SUM(votes.vote) AS SIGNED), 0) AS votes
         FROM jokes
         LEFT JOIN votes ON votes.joke_id = jokes.id
         GROUP BY jokes.id, jokes.user_name, jokes.content;",
    )
    .fetch_all(&appstate.connection_pool)
    .await
    {
        Ok(jokes) => (StatusCode::OK, Json(jokes)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query("INSERT INTO jokes (user_name, content) VALUES (?, ?);")
        .bind(&Some(params.get("user_name")))
        .bind(&Some(params.get("content")))
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::CONFLICT, error.to_string()).into_response(),
    }
}

pub async fn delete(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match query("DELETE FROM jokes WHERE id = ?;")
        .bind(&Some(params.get("id")))
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}
