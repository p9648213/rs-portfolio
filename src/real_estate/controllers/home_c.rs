use axum::{Extension, http::HeaderMap, response::Html};
use vy::IntoHtml;

use crate::{
    common::middlewares::auth_mw::UserAuth,
    real_estate::views::pages::home_v::{HomePageProps, render_home_page},
};

pub async fn get_home_page(
    Extension(user_auth): Extension<UserAuth>,
    _headers: HeaderMap,
) -> Html<String> {
    let props = HomePageProps { user_auth };
    Html(render_home_page(&props).into_string())
}
