use vy::prelude::*;

use crate::{
    common::{middlewares::auth_mw::UserAuth, views::head_v::render_head},
    real_estate::views::ui::{
        common::{footer_v::render_footer, nav_v::render_navbar},
        home::{
            call_to_action_v::render_call_to_action, discover_v::render_home_discover,
            feature_v::render_home_feature, hero_v::render_home_hero,
        },
    },
};

pub struct HomePageProps {
    pub user_auth: UserAuth,
}

pub struct DiscoverItem<'a> {
    pub image_src: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}

pub fn render_home_page(_props: &HomePageProps) -> impl IntoHtml {
    (
        DOCTYPE,
        html!(
            render_head(),
            title!("Home - Rentiful"),
            body!(
                "hx-boost" = true,
                render_navbar(),
                render_home_section(),
                render_footer()
            )
        ),
    )
}

pub fn render_home_section() -> impl IntoHtml {
    (
        render_home_hero(),
        render_home_feature(),
        render_home_discover(),
        render_call_to_action(),
    )
}
