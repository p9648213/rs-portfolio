use vy::prelude::*;

use crate::real_estate::constant::{MANAGER_NAV, TENANT_NAV, UserNavItem};

pub fn render_sidebar(role: &str) -> impl IntoHtml {
    (
        PreEscaped(
            r#"
                <script defer type="module">
                    import { setupTenantSidebar } from "/assets/js/realestate/tenant/sidebar.js"
                    setupTenantSidebar()
                </script>
            "#,
        ),
        div!(
            id = "tenant-sidebar",
            div!(
                class = "flex justify-between items-center gap-3",
                header!(class = "text-base font-bold", "Renter View"),
                button!(
                    id = "tenant-sidebar-button",
                    class = "h-6 w-6",
                    img!(
                        id = "sidebar-chevron-left",
                        class = "w-full h-full",
                        src = "/assets/images/common/chevron-left.svg",
                        alt = "arrow-back"
                    ),
                    img!(
                        id = "sidebar-chevron-right",
                        class = "w-full h-full",
                        src = "/assets/images/common/chevron-right.svg",
                        alt = "arrow-back"
                    )
                )
            ),
            ul!(
                class = "flex flex-col gap-2 mt-6",
                if role == "manager" {
                    MANAGER_NAV.map(|item| render_nav_item(item, "Properties"))
                } else {
                    TENANT_NAV.map(|item| render_nav_item(item, "Favorites"))
                }
            )
        ),
    )
}

fn render_nav_item(item: UserNavItem<'static>, active_item: &str) -> impl IntoHtml {
    li!(if item.label == active_item {
        a!(
            class = "flex gap-3 items-center px-3 py-2 bg-zinc-100 rounded-md",
            href = item.href,
            "hx-target" = "#tenant-content",
            div!(
                class = "h-4 w-4",
                img!(class = "w-full h-full", src = item.img, alt = "arrow-back")
            ),
            span!(class = "li-title", item.label)
        )
    } else {
        a!(
            class = "flex gap-3 items-center px-3 py-2 hover:bg-zinc-100 hover:rounded-md",
            href = item.href,
            "hx-target" = "#tenant-content",
            div!(
                class = "h-4 w-4",
                img!(class = "w-full h-full", src = item.img, alt = "arrow-back")
            ),
            span!(class = "li-title", item.label)
        )
    })
}
