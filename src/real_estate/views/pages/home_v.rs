use vy::prelude::*;

use crate::{
    common::{middlewares::auth_mw::UserAuth, views::head_v::render_head},
    real_estate::{
        constant::{
            FEATURES_DESCRIPTIONS, FEATURES_IMAGE_PATHS, FEATURES_LINK_HREFS, FEATURES_LINK_TEXTS,
            FEATURES_TITLES,
        },
        views::ui::nav_v::render_navbar,
    },
};

pub struct HomePageProps {
    pub user_auth: UserAuth,
}

pub fn render_home_page(_props: &HomePageProps) -> impl IntoHtml {
    (
        DOCTYPE,
        html!(
            render_head(),
            title!("Home - Rentiful"),
            body!("hx-boost" = true, render_navbar(), render_home_section())
        ),
    )
}

pub fn render_home_section() -> impl IntoHtml {
    (render_home_hero(), render_home_feature())
}

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
                            class = "bg-secondary-500 text-white rounded-none rounded-r-xl border-none hover:bg-secondary-600 h-12 px-4",
                            "Search"
                        )
                    )
                )
            )
        )
    )
}
