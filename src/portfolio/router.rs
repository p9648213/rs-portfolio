use axum::{Router, routing::get};

use crate::common::models::state::AppState;

use super::controllers::{home_c::get_home_page, section_c::get_nav_section};

pub fn create_portfolio_router() -> Router<AppState> {
    Router::new().route("/", get(get_home_page)).nest(
        "/section",
        Router::new().route("/nav/{section}", get(get_nav_section)),
    )
}
