//! Felhasználók kezelésére szolgáló szolgáló végpont.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use sqlx::query;

use crate::{models::User, AppState};

pub async fn add_user(
    State(database): State<Arc<AppState>>,
    user: Json<User>,
) -> impl IntoResponse {
    match query("").bind(&user.name).bind(&user.password).bind(&user.password)
}
