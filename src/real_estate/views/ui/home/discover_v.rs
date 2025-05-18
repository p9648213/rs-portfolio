use vy::prelude::*;

use crate::real_estate::{constant::DISCOVER_ITEMS, views::pages::home_v::DiscoverItem};

pub fn render_home_discover() -> impl IntoHtml {
    div!(
        class = "bg-white mb-16 py-12",
        div!(
            class = "mx-auto px-6 sm:px-8 lg:px-12 xl:px-16 max-w-6xl xl:max-w-7xl",
            div!(
                class = "my-12 text-center",
                h2!(
                    class = "font-semibold text-gray-800 text-3xl leading-tight",
                    "Discover"
                ),
                p!(
                    class = "mt-4 text-gray-600 text-lg",
                    "Find your Dream Rental Property Today!"
                ),
                p!(
                    class = "mx-auto mt-2 max-w-3xl text-gray-500",
                    "Searching for your dream rental property has never been easier. With our user-friendly search feature, you can quickly find the perfect home that meets all your needs. Start your search tody and discover your dream rental property!"
                )
            ),
            div!(
                class = "gap-8 xl-gap-16 lg:gap-12 grid grid-cols-1 md:grid-cols-3 text-center",
                DISCOVER_ITEMS.map(|item| { render_discover_card(item) })
            )
        )
    )
}

pub fn render_discover_card(item: DiscoverItem) -> impl IntoHtml {
    div!(
        class = "bg-zinc-50 shadow-lg px-4 py-12 rounded-lg md:h-72",
        div!(
            class = "bg-zinc-700 mx-auto mb-4 p-[0.6rem] rounded-full w-10 h-10",
            img!(
                src = item.image_src,
                alt = item.title,
                class = "w-full h-full"
            )
        ),
        h3!(class = "mt-4 font-medium text-gray-800 text-xl", item.title),
        p!(class = "mt-4 text-gray-500 text-base", item.description)
    )
}
