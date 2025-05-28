use axum::{
    Router,
    routing::{get, patch},
};

use crate::{
    common::{controllers::user_c::update_user, models::state::AppState},
    real_estate::controllers::search_c::get_search_page,
};

use super::controllers::{
    home_c::get_home_page,
    search_c::{get_amenity_ui, get_property_type_ui},
    tenant_c::{get_tenant_page, get_tenant_ui},
};

pub fn create_real_estate_router() -> Router<AppState> {
    Router::new().nest(
        "/realestate",
        Router::new()
            .route("/", get(get_home_page))
            .route("/user", patch(update_user))
            .route("/search", get(get_search_page))
            .route("/tenant/{slug}", get(get_tenant_page))
            .route("/ui/tenant/{slug}", get(get_tenant_ui))
            .route("/ui/search/property_type/{slug}", get(get_property_type_ui))
            .route("/ui/search/amenity/{slug}", get(get_amenity_ui)),
    )
}
