use vy::prelude::*;

use crate::{
    common::{middlewares::auth_mw::UserInfo, views::head_v::render_head},
    real_estate::{
        controllers::search_c::SearchQuery,
        views::ui::{
            common::nav_v::{NavBarProps, render_navbar},
            search::{filter_bar_v::render_filter_bar, filter_full_v::render_filter_full},
        },
    },
};

pub struct SearchPageProps<'a> {
    pub user_info: Option<UserInfo>,
    pub is_dashboard_page: bool,
    pub search_query: &'a SearchQuery,
}

pub fn render_search_page(props: &SearchPageProps) -> impl IntoHtml {
    let nav_props = NavBarProps {
        user_info: props.user_info.as_ref(),
        is_dashboard_page: props.is_dashboard_page,
    };

    (
        DOCTYPE,
        html!(
            lang = "en",
            render_head(),
            title!("Search - Rentiful"),
            body!(
                class = "text-sm",
                "hx-boost" = true,
                render_navbar(nav_props),
                main!(render_search_section(props.search_query))
            ),
            div!(id = "toast")
        ),
    )
}

pub fn render_search_section(search_query: &SearchQuery) -> impl IntoHtml {
    div!(
        class = "h-screen pt-[52px] flex flex-col px-5",
        render_filter_bar(search_query),
        div!(
            class = "flex flex-1 justify-between gap-3 mb-5 overflow-hidden",
            div!(
                id = "filter-full",
                class = "hidden w-94 min-w-94 h-full overflow-auto",
                render_filter_full(search_query)
            ),
            div!("Map"),
            div!(class = "overflow-y-auto basic-4/12", "Listings")
        )
    )
}
