use vy::prelude::*;

use crate::{
    common::{middlewares::auth_mw::UserInfo, views::head_v::render_head},
    real_estate::views::ui::{
        common::nav_v::{NavBarProps, render_navbar},
        tenant::sidebar_v::render_tenant_sidebar,
    },
};

#[derive(PartialEq, Eq)]
pub enum TenantSlug {
    Favorites,
    Settings,
}

pub struct TenantPageProps<'a> {
    pub user_info: &'a UserInfo,
    pub is_dashboard_page: bool,
    pub slug: TenantSlug,
}

pub fn render_tenant_page(props: &TenantPageProps) -> impl IntoHtml {
    let nav_props = NavBarProps {
        user_info: Some(props.user_info),
        is_dashboard_page: props.is_dashboard_page,
    };

    (
        DOCTYPE,
        html!(
            lang = "en",
            render_head(),
            title!("Tenant - Rentiful"),
            body!(
                class = "text-sm",
                "hx-boost" = true,
                render_navbar(nav_props),
                main!(render_tenant_section(&props.slug))
            ),
            div!(id = "toast")
        ),
    )
}

pub fn render_tenant_section(slug: &TenantSlug) -> impl IntoHtml {
    div!(
        class = "h-screen pt-[52px] flex",
        id = "tenant-section",
        render_tenant_sidebar(slug),
        div!(
            class = "bg-zinc-100 flex-1",
            if *slug == TenantSlug::Favorites {
                render_tenant_favorites()
            } else {
                render_tenant_settings()
            }
        )
    )
}

pub fn render_tenant_favorites() -> impl IntoHtml {
    div!("Favorites")
}

pub fn render_tenant_settings() -> impl IntoHtml {
    div!(
        "hx-get" = "/realestate/ui/tenant/settings-form",
        "hx-trigger" = "load",
    )
}
