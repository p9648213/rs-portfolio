use axum::{Extension, response::Html};
use vy::IntoHtml;

use crate::{
    common::middlewares::auth_mw::UserAuth,
    real_estate::views::pages::search_v::{SearchPageProps, render_search_page},
};

pub async fn get_search_page(Extension(user_auth): Extension<UserAuth>) -> Html<String> {
    let props = SearchPageProps {
        user_info: user_auth.0,
        is_dashboard_page: false,
    };

    Html(render_search_page(&props).into_string())
}

pub async fn search() {}
