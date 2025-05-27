use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::{
    common::{controllers::user_c::update_user, models::state::AppState},
    real_estate::controllers::search_c::get_search_page,
};

use super::controllers::{
    home_c::get_home_page,
    search_c::search,
    tenant_c::{get_tenant_page, get_tenant_ui},
};

pub fn create_real_estate_router() -> Router<AppState> {
    Router::new().nest(
        "/realestate",
        Router::new()
            .route("/", get(get_home_page))
            .route("/user", patch(update_user))
            .route("/search", get(get_search_page))
            .route("/search", post(search))
            .route("/tenant/{slug}", get(get_tenant_page))
            .route("/ui/tenant/{slug}", get(get_tenant_ui)),
    )
}
