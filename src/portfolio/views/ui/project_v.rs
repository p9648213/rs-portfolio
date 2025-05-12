use vy::prelude::*;

pub fn render_project() -> impl IntoHtml {
    (
        h2!(class = "text-xl font-bold", "Project"),
        div!(
            class = "flex gap-2",
            span!(
                class = "ml-3 font-semibold",
                "- Rentiful: ",
                span!(class = "font-normal", "SSR Fullstack Real Estate Web App |")
            ),
            a!(
                class = "text-blue-400 underline",
                href = "/realestate",
                "Preview"
            ),
        ),
    )
}
