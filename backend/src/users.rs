use core::fmt::{self, Debug, Formatter};

use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use thiserror::Error;
use tokio::task;

#[derive(Clone, Serialize, FromRow)]
pub struct User {
    pub name: String,
    password: String,
    admin: bool,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("password", &"******")
            .field("admin", &self.admin)
            .finish()
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: MySqlPool,
}

impl Backend {
    pub const fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    MySql(#[from] sqlx::Error),
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type Credentials = Credentials;
    type Error = Error;
    type User = User;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> =
            sqlx::query_as("SELECT * FROM users WHERE name = ?")
                .bind(creds.name)
                .fetch_optional(&self.db)
                .await?;

        task::spawn_blocking(|| {
            Ok(user.filter(|user| {
                password_auth::verify_password(creds.password, &user.password)
                    .is_ok()
            }))
        })
        .await?
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("SELECT * FROM users WHERE name = ?")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}
