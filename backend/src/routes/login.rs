//! Felhasználók bejelentkezését kiszolgáló végpont

use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use axum_login::AuthSession;
use tracing::instrument;

use crate::users::{Backend, Credentials};

#[instrument("login::post", skip(auth_session))]
pub async fn post(
    mut auth_session: AuthSession<Backend>,
    Query(creds): Query<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return StatusCode::UNAUTHORIZED;
        }
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if auth_session.login(&user).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
