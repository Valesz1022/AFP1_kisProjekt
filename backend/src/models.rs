use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Joke {
    pub id: u64,
    pub user_name: String,
    pub content: String,
    pub votes: i64,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Vote {
    pub user_name: String,
    pub joke_id: u64,
    pub vote: i8,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Saved {
    pub user_name: String,
    pub joke_id: u64,
}
