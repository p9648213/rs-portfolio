use vy::prelude::*;

pub fn render_resume() -> impl IntoHtml {
    (
        h2!(class = "text-xl font-bold mb-2", "🦀 Resume"),
        div!(
            class = "flex gap-2 ml-3 whitespace-pre",
            h2!(class = "text-base font-semibold", "- Github:"),
            a!(
                href = "https://github.com/p9648213",
                target = "_blank",
                class = "text-blue-400 underline",
                "github.com/p9648213"
            )
        ),
        div!(
            class = "flex gap-2 ml-3 whitespace-pre",
            h2!(class = "text-base font-semibold", "- Technical Skills:"),
            "Rust, Javascript, Html, Css, Sql, Nosql, Docker."
        ),
        div!(
            class = "flex gap-2 ml-3",
            h2!(
                class = "text-base font-semibold whitespace-pre",
                "- Education:"
            ),
            "Bachelor of Science in Information Technology, UIT."
        ),
        div!(
            class = "flex flex-col gap-2 ml-3",
            h2!(class = "text-base font-semibold", "- Experience:"),
            span!(
                class = "ml-3",
                "<> CodeforceVina (2023 - Present): Developed a real estate content management system."
            ),
            span!(
                class = "ml-3",
                "<> TMA Solutions (2022 - 2023): Developed an intership management application."
            )
        ),
        a!(
            href = "/assets/file/resume.pdf",
            target = "_blank",
            class = "text-blue-400 underline ml-3",
            "Preview full resume"
        ),
    )
}
