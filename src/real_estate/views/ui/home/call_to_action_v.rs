use vy::prelude::*;

pub fn render_call_to_action() -> impl IntoHtml {
    div!(
        class = "relative h-80",
        img!(
            class = "object-center object-cover w-full h-full",
            alt = "Rentiful Search Section Background",
            src = "/assets/images/real-estate/landing-call-to-action.jpg"
        ),
        div!(
            class = "absolute inset-0 bg-black/60 flex items-center",
            div!(
                class =
                    "relative mx-auto px-6 sm:px-8 lg:px-12 xl:px-16 py-12 max-w-5xl xl:max-w-7xl",
                div!(
                    class = "flex md:flex-row flex-col justify-between items-center",
                    div!(
                        class = "md:mr-10 mb-6 md:mb-0",
                        h2!(
                            class = "font-bold text-white text-2xl",
                            "Find Your Dream rental Property"
                        )
                    ),
                    div!(
                        class = "flex flex-col gap-3 max-md:items-center",
                        p!(
                            class = "text-white",
                            "Discover a wide range of rental properties in your desired location."
                        ),
                        div!(
                            class = "flex justify-center md:justify-start gap-4",
                            button!(
                                onclick = "window.scrollTo({top: 0, behavior: 'smooth'})",
                                class = "inline-block bg-white hover:bg-primary-500 px-6 py-3 rounded-lg font-semibold text-primary-700 hover:text-primary-50 cursor-pointer",
                                "Search"
                            )
                        ),
                        a!(
                            href = "/realestate/sign-up",
                            "hx-swap" = "none",
                            class = "inline-block bg-secondary-500 hover:bg-secondary-600 px-6 py-3 rounded-lg font-semibold text-white cursor-pointer w-fit",
                            "Sign Up"
                        )
                    )
                )
            )
        )
    )
}
