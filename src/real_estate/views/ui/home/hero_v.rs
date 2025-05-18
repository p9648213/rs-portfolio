use vy::prelude::*;

pub fn render_home_hero() -> impl IntoHtml {
    div!(
        class = "relative h-screen",
        img!(
            src = "/assets/images/real-estate/landing-splash.jpg",
            alt = "Rentiful Rental Platform Hero Section",
            class = "object-cover object-center h-full w-full"
        ),
        div!(
            class = "absolute inset-0 bg-black/60",
            div!(
                class = "absolute top-1/2 transform -translate-y-1/2 text-center w-full",
                div!(
                    class = "max-w-4xl mx-auto px-16 sm:px-12",
                    h1!(
                        class = "text-5xl font-bold text-white mb-4",
                        "Start your journey to finding the perfect place to call home"
                    ),
                    p!(
                        class = "text-xl text-white mb-8",
                        "Explore our wide range of rental properties tailored to fit your
                        lifestyle and needs!"
                    ),
                    div!(
                        class = "flex justify-center",
                        input!(
                            "type" = "text",
                            placeholder = "Search by city, neightborhood or address",
                            class = "w-full max-w-lg rounded-none rounded-l-xl border-none bg-white h-12"
                        ),
                        button!(
                            class = "bg-red-400 text-white rounded-none rounded-r-xl border-none hover:bg-red-500 h-12 px-4",
                            "Search"
                        )
                    )
                )
            )
        )
    )
}
