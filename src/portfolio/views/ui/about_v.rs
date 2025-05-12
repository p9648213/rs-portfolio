use vy::prelude::*;

pub fn render_about_me() -> impl IntoHtml {
    (
        h2!(class = "text-xl font-bold", "🦀 About Me"),
        p!(
            "Hi, I'm a software engineer powered by caffeine, Rust, and an unreasonable number of open terminal tabs. By day, I build robust, scalable systems that probably won't crash. By night, I write weird side projects in Rust just to see if I can—like a CLI that reminds me to touch grass (I’m still debugging it)."
        ),
        p!(
            "I speak fluent async, dream in match statements, and believe memory safety is not just a feature—it's a lifestyle. When something goes wrong, I don’t panic; I just blame the borrow checker (even if it’s a TypeError in JavaScript)."
        ),
        p!(
            "If you need someone who writes code that compiles and has a sense of humor about it, we might just be a match—no pun intended. 🧡"
        ),
    )
}
