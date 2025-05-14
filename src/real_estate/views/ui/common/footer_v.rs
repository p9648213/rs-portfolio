use vy::prelude::*;

pub fn render_footer() -> impl IntoHtml {
    footer!(
        class = "py-20 border-gray-200 border-t",
        div!(
            class = "flex flex-col items-center mx-auto px-6 sm:px-8 max-w-4l",
            div!(
                class = "mb-4",
                a!(
                    href = "/realestate",
                    class = "font-bold text-xl",
                    "RENTIFUL"
                )
            ),
            nav!(
                class = "mb-4",
                ul!(
                    class = "flex space-x-6",
                    li!(a!(href = "#about", "About")),
                    li!(a!(href = "#contact", "Contact")),
                    li!(a!(href = "#faq", "Faq")),
                    li!(a!(href = "#terms", "Terms")),
                    li!(a!(href = "#privacy", "Privacy"))
                )
            ),
            div!(
                class = "flex space-x-4 mb-4",
                a!(
                    href = "#facebook",
                    "aria-label" = "Facebook",
                    class = "w-6 h-6 hover:text-primary-600",
                    img!(
                        src = "/assets/images/real-estate/facebook.svg",
                        class = "w-full h-full"
                    )
                ),
                a!(
                    href = "#instagram",
                    "aria-label" = "Instagram",
                    class = "w-6 h-6 hover:text-primary-600",
                    img!(
                        src = "/assets/images/real-estate/instagram.svg",
                        class = "w-full h-full"
                    )
                ),
                a!(
                    href = "#linkedin",
                    "aria-label" = "Linkedin",
                    class = "w-6 h-6 hover:text-primary-600",
                    img!(
                        src = "/assets/images/real-estate/linkedin.svg",
                        class = "w-full h-full"
                    )
                ),
                a!(
                    href = "#youtube",
                    "aria-label" = "Youtube",
                    class = "w-6 h-6 hover:text-primary-600",
                    img!(
                        src = "/assets/images/real-estate/youtube.svg",
                        class = "w-full h-full"
                    )
                )
            ),
            div!(
                class = "flex justify-center space-x-4 mt-8 text-gray-500 text-sm text-center",
                span!("© RENTiful. All rights reserved"),
                a!(href = "/privacy", "Privacy Policy"),
                a!(href = "/terms", "Terms of Service"),
                a!(href = "/cookies", "Cookie Policy")
            )
        )
    )
}
