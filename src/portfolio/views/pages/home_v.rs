use vy::prelude::*;

use crate::{
    common::views::head_v::render_head,
    portfolio::views::ui::{about_v::render_about_me, nav_v::render_navigation},
};

pub fn render_home_page() -> impl IntoHtml {
    (
        DOCTYPE,
        html!(
            render_head(),
            title!("Portfolio"),
            body!(
                "hx-boost" = true,
                main!(
                    class = "h-screen flex justify-center items-center",
                    div!(
                        class = "grid grid-cols-[auto_1fr] w-full max-w-5xl gap-5",
                        div!(
                            class = "border border-gray-900 rounded-md p-5",
                            ul!(
                                class = "flex flex-col gap-2 text-lg",
                                li!(
                                    class = "flex flex-col items-center gap-2",
                                    div!(
                                        class = "rounded-md overflow-hidden w-40 h-40 border-gray-400 border",
                                        img!(
                                            src = "/assets/images/portfolio/elephant.jpg",
                                            alt = "elephant",
                                            class = "w-full h-full"
                                        ),
                                    ),
                                    div!("Phat Luong"),
                                    div!(
                                        class = "flex gap-2 items-center",
                                        div!(
                                            class = "h-5 w-5",
                                            img!(
                                                src = "/assets/images/portfolio/mail.svg",
                                                alt = "mail",
                                                class = "w-full h-full"
                                            )
                                        ),
                                        span!(class = "text-base", "p9648213750912@gmail.com")
                                    )
                                ),
                                render_navigation("About")
                            )
                        ),
                        div!(
                            id = "right-content",
                            class = "flex flex-col gap-2 border border-gray-900 rounded-md p-5 text-base text-justify",
                            render_about_me()
                        )
                    )
                )
            )
        ),
    )
}
