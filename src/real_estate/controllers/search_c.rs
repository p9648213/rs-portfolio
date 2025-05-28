use axum::{Extension, extract::Path, response::Html};
use vy::IntoHtml;

use crate::{
    common::middlewares::auth_mw::UserAuth,
    real_estate::views::{
        pages::search_v::{SearchPageProps, render_search_page},
        ui::search::filter_full_v::{render_ameniy, render_property_type},
    },
};

pub async fn get_search_page(Extension(user_auth): Extension<UserAuth>) -> Html<String> {
    let props = SearchPageProps {
        user_info: user_auth.0,
        is_dashboard_page: false,
    };

    Html(render_search_page(&props).into_string())
}

pub async fn get_property_type_ui(Path(slug): Path<String>) -> Html<String> {
    Html(render_property_type(slug.as_str()).into_string())
}

pub async fn get_amenity_ui(Path(slug): Path<String>) -> Html<String> {
    Html(render_ameniy(slug.as_str()).into_string())
}
