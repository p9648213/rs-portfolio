use axum::{
    Extension,
    extract::{Path, Query},
    http::HeaderMap,
    response::Html,
};
use serde::Deserialize;
use vy::IntoHtml;

use crate::{
    common::middlewares::auth_mw::UserAuth,
    real_estate::views::{
        pages::search_v::{SearchPageProps, render_search_page, render_search_section},
        ui::search::filter_full_v::{render_ameniy, render_property_type},
    },
};

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub location: Option<String>,
    pub property_type: Option<String>,
    pub amenity: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub beds: Option<String>,
    pub baths: Option<String>,
    pub min_square: Option<String>,
    pub max_square: Option<String>,
    pub available_date: Option<String>,
}

pub async fn get_search_page(
    headers: HeaderMap,
    Extension(user_auth): Extension<UserAuth>,
    Query(search_query): Query<SearchQuery>,
) -> Html<String> {
    let hx_current_url = headers.get("HX-Current-URL");

    let props = SearchPageProps {
        user_info: user_auth.0,
        is_dashboard_page: false,
        search_query: &search_query,
    };

    if let Some(url) = hx_current_url {
        if url
            .to_str()
            .unwrap_or_default()
            .contains("/realestate/search")
        {
            return Html(render_search_section(&search_query).into_string());
        }
    }

    Html(render_search_page(&props).into_string())
}

pub async fn get_property_type_ui(Path(slug): Path<String>) -> Html<String> {
    Html(render_property_type(slug.as_str()).into_string())
}

pub async fn get_amenity_ui(Path(slug): Path<String>) -> Html<String> {
    Html(render_ameniy(slug.as_str()).into_string())
}
