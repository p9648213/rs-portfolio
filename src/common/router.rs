use axum::{
    Router,
    routing::{get, post},
};

use crate::common::controllers::auth_c::{google_callback, google_login, login, logout, register};

use super::models::state::AppState;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/logout", post(logout))
        .route("/auth/google/login", get(google_login))
        .route("/auth/google/callback", get(google_callback))
}
