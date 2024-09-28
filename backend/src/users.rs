use core::fmt::{self, Debug, Formatter};
use std::{collections::HashSet, hash::Hash};

use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use thiserror::Error;
use tokio::task;
use tower_sessions::cookie::ParseError;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, FromRow)]
pub struct Permission {
    pub name: String,
}

#[async_trait]
impl AuthzBackend for Backend {
    type Permission = Permission;

    async fn get_group_permissions(
        &self,
        user: &Self::User
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        // Ha szeretnénk több jogosultságot hozzáadni, így kicsit könnyebb lesz
        // később.
        if user.admin {
            Ok(HashSet::from([Permission{name: "admin".to_owned()}]))
        } else {
            Ok(HashSet::new())
        }
    }
}
