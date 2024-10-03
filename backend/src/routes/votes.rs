//! Értékelések kezelését kiszolgáló végpont

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::query;
use tracing::instrument;

use crate::AppState;

#[instrument(name = "votes::post", skip(appstate))]
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
    let Some(vote) = params.get("vote") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query(
        "INSERT INTO votes (user_name, joke_id, vote) VALUES (?, ?, ?);",
    )
    .bind(name)
    .bind(joke_id)
    .bind(vote)
    .execute(&appstate.connection_pool)
    .await
    {
        Ok(..) => StatusCode::CREATED.into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => {
                (StatusCode::NOT_FOUND, Json(db_err.to_string()))
                    .into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

#[instrument(name = "votes::put", skip(appstate))]
pub async fn put(
    State(appstate): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(name) = params.get("name") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };
    let Some(joke_id) = params.get("joke_id") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };
    let Some(vote) = params.get("vote") else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    match query(
        "UPDATE votes SET vote = ? WHERE user_name = ? AND joke_id = ?;",
    )
    .bind(name)
    .bind(joke_id)
    .bind(vote)
    .execute(&appstate.connection_pool)
    .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => {
                (StatusCode::NOT_FOUND, Json(db_err.to_string()))
                    .into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

#[instrument(name = "votes::delete", skip(appstate))]
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

    match query("DELETE FROM votes WHERE user_name = ? AND joke_id = ?;")
        .bind(name)
        .bind(joke_id)
        .execute(&appstate.connection_pool)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => match error {
            sqlx::Error::Database(db_err) => {
                (StatusCode::NOT_FOUND, Json(db_err.to_string()))
                    .into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}
