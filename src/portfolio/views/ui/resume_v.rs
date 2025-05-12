use vy::prelude::*;

pub fn render_resume() -> impl IntoHtml {
    (h2!(class = "text-xl font-bold", "Resume"), p!("Resume"))
}
