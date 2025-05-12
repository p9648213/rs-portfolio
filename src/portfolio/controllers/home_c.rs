use axum::response::Html;
use vy::IntoHtml;

use crate::portfolio::views::pages::home_v::render_home_page;

pub async fn get_home_page() -> Html<String> {
    Html(render_home_page().into_string())
}
