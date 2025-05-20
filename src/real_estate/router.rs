use axum::{
    Router,
    routing::{get, patch},
};

use crate::common::{controllers::user_c::update_user, models::state::AppState};

use super::controllers::{
    home_c::get_home_page,
    tenant_c::{get_tenant_page, get_tenant_ui},
};

pub fn create_real_estate_router() -> Router<AppState> {
    Router::new().nest(
        "/realestate",
        Router::new()
            .route("/", get(get_home_page))
            .route("/user", patch(update_user))
            .route("/tenant/{slug}", get(get_tenant_page))
            .route("/ui/tenant/{slug}", get(get_tenant_ui)),
    )
}
