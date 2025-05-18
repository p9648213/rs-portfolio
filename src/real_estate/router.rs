use axum::{Router, routing::get};

use crate::common::models::state::AppState;

use super::controllers::{home_c::get_home_page, tenant_c::get_tenant_page};

pub fn create_real_estate_router() -> Router<AppState> {
    Router::new().nest(
        "/realestate",
        Router::new()
            .route("/", get(get_home_page))
            .route("/tenant/{slug}", get(get_tenant_page)),
    )
}
