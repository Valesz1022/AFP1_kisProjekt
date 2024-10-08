//! Felhasználók bejelentkezését kiszolgáló végpont

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use axum_login::AuthSession;
use serde_json::json;
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
            return StatusCode::UNAUTHORIZED.into_response();
        }
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if auth_session.login(&user).await.is_ok() {
        let payload = json!({
            "username": user.name,
            "admin": user.admin
        });
        (StatusCode::OK, Json(payload)).into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
