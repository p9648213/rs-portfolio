use axum::{Extension, response::Html};
use vy::IntoHtml;

use crate::{
    common::middlewares::auth_mw::UserAuth,
    real_estate::views::pages::home_v::{HomePageProps, render_home_page},
};

pub async fn get_home_page(Extension(user_auth): Extension<UserAuth>) -> Html<String> {
    let props = HomePageProps {
        user_info: user_auth.0,
        is_dashboard_page: false,
    };

    Html(render_home_page(&props).into_string())
}
