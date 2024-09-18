use crate::models::{Joke, Saved};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::{query, query_as, MySql, MySqlPool};

async fn get_saved(
    State(database): State<&MySqlPool>,
    saved: Json<String>,
) -> impl IntoResponse {
    match query_as::<MySql, Joke>(
        "SELECT 
            jokes.id, 
            jokes.user_name, 
            jokes.content,
            COALESCE(SUM(votes.vote), 0) AS votes
        FROM saved
        INNER JOIN users ON saved.user_name = users.name
        INNER JOIN jokes ON saved.joke_id = jokes.id
        LEFT JOIN votes ON votes.joke_id = jokes.id
        WHERE users.name = '?'
        GROUP BY jokes.id, jokes.user_name, jokes.content;",
    )
    .bind(&*saved)
    .fetch_all(database)
    .await
    {
        Ok(saved) => (StatusCode::OK, Json(saved)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}

async fn put_saved(
    State(database): State<&MySqlPool>,
    saved: Json<Saved>,
) -> impl IntoResponse {
    match query("INSERT INTO saved (user_name, joke_id) VALUES ('?', '?');")
        .bind(&saved.user_name)
        .bind(&saved.joke_id)
        .execute(database)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string())
            .into_response(),
    }
}

async fn delete_saved(
    State(database): State<&MySqlPool>,
    saved: Json<Saved>,
) -> impl IntoResponse {
    match query("DELETE FROM saved WHERE user_name = '?' AND joke_id = ?;")
        .bind(&saved.user_name)
        .bind(&saved.joke_id)
        .execute(database)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}
