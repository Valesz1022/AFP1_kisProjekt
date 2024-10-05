//! Mentett viccek kezelését kiszolgáló végpont

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::{query, query_as, MySql};
use tracing::instrument;

use crate::{models::Joke, AppState};

#[instrument(name = "saved::get", skip(appstate))]
pub async fn get(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(name) = params.get("name") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

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
    .bind(name)
    .fetch_all(&appstate.connection_pool)
    .await
    {
        Ok(jokes) => (StatusCode::OK, Json(jokes)).into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => {
                (StatusCode::CONFLICT, Json(db_err.to_string()))
                    .into_response()
            }
            sqlx::Error::RowNotFound => {
                StatusCode::NOT_FOUND.into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

#[instrument(name = "saved::post", skip(appstate))]
pub async fn post(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(name) = params.get("name") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };
    let Some(joke_id) = params.get("joke_id") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query("INSERT INTO saved (user_name, joke_id) VALUES (?, ?);")
        .bind(name)
        .bind(joke_id)
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::CREATED.into_response(),
        Err(error) => match error {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

#[instrument(name = "saved::delete", skip(appstate))]
pub async fn delete(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(name) = params.get("name") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };
    let Some(joke_id) = params.get("joke_id") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query("DELETE FROM saved WHERE user_name = ? AND joke_id = ?;")
        .bind(name)
        .bind(joke_id)
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => {
                (StatusCode::CONFLICT, Json(db_err.to_string()))
                    .into_response()
            }
            sqlx::Error::RowNotFound => {
                StatusCode::NOT_FOUND.into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}
