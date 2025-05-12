use axum::{extract::Path, response::Html};

use crate::{common::models::error::AppError, portfolio::views::ui::section_v::render_nav_section};

pub async fn get_nav_section(Path(section): Path<String>) -> Result<Html<String>, AppError> {
    Ok(Html(render_nav_section(&section)?))
}
