use vy::prelude::*;

pub fn render_navigation(highlight_text: &str) -> impl IntoHtml {
    li!(
        id = "navigation",
        class = "flex gap-3 justify-center",
        ["About", "Project", "Resume"].map(|t| {
            a!(
                "hx-get" = format!("/section/nav/{}", t.to_lowercase()),
                "hx-target" = "#navigation",
                class = if highlight_text.to_lowercase() == t.to_lowercase() {
                    r#"font-bold"#
                } else {
                    ""
                },
                t
            )
        })
    )
}
