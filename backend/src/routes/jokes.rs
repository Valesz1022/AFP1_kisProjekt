//! Viccek kezelését kiszolgáló végpont

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::{query, query_as, MySql};
use tracing::instrument;

use crate::{models::Joke, AppState};

#[instrument(name = "jokes::get", skip(appstate))]
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

#[instrument(name = "jokes::post", skip(appstate))]
pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(user_name) = params.get("name") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };
    let Some(content) = params.get("content") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query("INSERT INTO jokes (user_name, content) VALUES (?, ?);")
        .bind(user_name)
        .bind(content)
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::CREATED.into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => {
                (StatusCode::CONFLICT, db_err.to_string()).into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

#[instrument(name = "jokes::delete", skip(appstate))]
pub async fn delete(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(id) = params.get("id") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query("DELETE FROM jokes WHERE id = ?;")
        .bind(id)
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => {
                (StatusCode::NOT_FOUND, db_err.to_string()).into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}
