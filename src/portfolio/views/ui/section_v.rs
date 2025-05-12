use reqwest::StatusCode;
use vy::prelude::*;

use crate::{
    common::models::error::AppError,
    portfolio::views::ui::{
        about_v::render_about_me, project_v::render_project, resume_v::render_resume,
    },
};

use super::nav_v::render_navigation;

pub fn render_nav_section(section: &str) -> Result<String, AppError> {
    match section {
        "about" => Ok((
            render_navigation("About"),
            div!(
                id = "right-content",
                "hx-swap-oob"="true",
                class = "flex flex-col gap-2 border border-gray-900 rounded-md p-5 text-base text-justify",
                render_about_me()
            ),
        ).into_string()),
        "project" => Ok((
            render_navigation("Project"),
            div!(
                id = "right-content",
                "hx-swap-oob"="true",
                class = "flex flex-col gap-2 border border-gray-900 rounded-md p-5 text-base text-justify",
                render_project()
            ),
        ).into_string()),
        "resume" => Ok((
            render_navigation("Resume"),
            div!(
                id = "right-content",
                "hx-swap-oob"="true",
                class = "flex flex-col gap-2 border border-gray-900 rounded-md p-5 text-base text-justify",
                render_resume()
            ),
        ).into_string()),
        _ =>  Err(AppError::new(StatusCode::NOT_FOUND, "Section not found"))
    }
}
