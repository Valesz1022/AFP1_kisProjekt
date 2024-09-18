use sqlx::{FromRow, MySqlConnection, MySqlPool};
use serde::{Deserialize, Serialize};

#[derive(FromRow, Deserialize, Serialize)]
struct User {
    name: String,
    password: String,
    #[serde(skip_deserializing)]
    admin: bool,
}

impl User {
    pub async fn get_all(conn: &mut MySqlConnection) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users;").fetch_all(conn).await
    }
}
