use axum::{Extension, extract::Request, response::Html};
use vy::IntoHtml;

use crate::{
    common::middlewares::auth_mw::UserAuth,
    real_estate::views::pages::home_v::{HomePageProps, render_home_page},
};

pub async fn get_home_page(
    Extension(user_auth): Extension<UserAuth>,
    request: Request,
) -> Html<String> {
    let pathname = request.uri().path();

    let is_dashboard_page = pathname.contains("/managers") || pathname.contains("/tenants");

    let props = HomePageProps {
        user_auth,
        is_dashboard_page,
    };

    Html(render_home_page(&props).into_string())
}
