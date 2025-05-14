use vy::prelude::*;

use crate::real_estate::constant::{
    FEATURES_DESCRIPTIONS, FEATURES_IMAGE_PATHS, FEATURES_LINK_HREFS, FEATURES_LINK_TEXTS,
    FEATURES_TITLES,
};

pub fn render_home_feature() -> impl IntoHtml {
    div!(
        class = "bg-white px-6 sm:px-8 lg:px-12 xl:px-16 py-24",
        div!(
            class = "mx-auto max-w-4xl xl:max-w-6xl",
            h2!(
                class = "mx-auto mb-12 w-full sm:w-2/3 font-bold text-3xl text-center",
                "Quickly find the home you want using out effective search filters!"
            ),
            div!(
                class = "gap-8 xl-gap-16 lg:gap-12 grid grid-cols-1 md:grid-cols-3",
                (0..3).map(|index| {
                    render_home_feature_card(
                        FEATURES_IMAGE_PATHS[index],
                        FEATURES_TITLES[index],
                        FEATURES_DESCRIPTIONS[index],
                        FEATURES_LINK_TEXTS[index],
                        FEATURES_LINK_HREFS[index],
                    )
                })
            ),
        )
    )
}

pub fn render_home_feature_card<'a>(
    image_src: &'a str,
    title: &'a str,
    description: &'a str,
    link_text: &'a str,
    link_href: &'a str,
) -> impl IntoHtml + use<'a> {
    div!(
        class = "text-center",
        div!(
            class = "flex justify-center items-center mb-4 p-4 rounded-lg",
            div!(
                class = "xl:h-[200px] lg:h-[150px]",
                img!(
                    src = image_src,
                    alt = title,
                    class = "w-full h-full object-contain"
                )
            )
        ),
        h3!(class = "mb-2 font-semibold text-xl", title),
        p!(class = "mb-4", description),
        a!(
            href = link_href,
            "hx-swap" = "none",
            class = "inline-block hover:bg-gray-100 px-4 py-2 border border-gray-300 rounded",
            link_text
        )
    )
}
