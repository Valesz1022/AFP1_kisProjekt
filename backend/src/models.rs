//! Json modellek

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub admin: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Joke {
    pub id: i32,
    pub user_name: String,
    pub content: String,
    pub votes: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Saved {
    pub user_name: String,
    pub joke_id: i32,
}
