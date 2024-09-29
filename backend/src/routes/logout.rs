//! A kijelentkezésre szolgáló végpont.

use axum::{http::StatusCode, response::IntoResponse};
use axum_login::AuthSession;
use tracing::instrument;

use crate::users::Backend;

#[instrument("logout::get", skip_all)]
pub async fn get(mut auth_session: AuthSession<Backend>) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
