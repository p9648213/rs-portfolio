use vy::prelude::*;

pub fn render_navbar() -> impl IntoHtml {
    div!(
        class = "fixed top-0 left-0 w-full z-50 shadow-xl h-[52px]",
        div!(
            class = "flex justify-between items-center h-full w-full py-3 px-8 bg-primary-700 text-white",
            div!(
                class = "flex items-center gap-4 md:gap-6",
                a!(
                    href = "/realestate",
                    class = "cursor-pointer hover:!text-primary-300",
                    div!(
                        class = "flex items-center gap-3",
                        img!(
                            src = "/assets/images/real-estate/logo.svg",
                            alt = "Rentiful Logo",
                            class = "w-6 h-6"
                        ),
                        div!(
                            class = "text-xl font-bold",
                            "RENT",
                            span!(
                                class = "text-secondary-500 font-light hover:!text-primary-300",
                                "IFUL"
                            )
                        )
                    )
                )
            ),
            p!(
                class = "text-primary-200 hidden md:block",
                "Discover your perfect rental apartment with out advance search"
            ),
            div!(
                class = "flex items-center gap-5",
                a!(
                    href = "/signin",
                    class = "text-white border-white bg-transparent hover:bg-white hover:text-primary-700 rounded-lg cursor-pointer py-1.5 px-3 border",
                    "Sign In"
                ),
                a!(
                    href = "/signup",
                    class = "bg-secondary-600 hover:bg-white  hover:text-primary-700 rounded-lg cursor-pointer py-1.5 px-3 border",
                    "Sign Up"
                )
            )
        )
    )
}
