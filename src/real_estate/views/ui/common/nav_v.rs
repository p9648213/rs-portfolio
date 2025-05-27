use vy::prelude::*;

use crate::common::{
    middlewares::auth_mw::UserInfo,
    views::ui::auth_modal_v::{render_login_modal, render_register_modal},
};

pub struct NavBarProps<'a> {
    pub user_info: Option<&'a UserInfo>,
    pub is_dashboard_page: bool,
}

pub fn render_navbar(props: NavBarProps) -> impl IntoHtml {
    div!(
        class = "fixed top-0 left-0 w-full z-50 shadow-xl h-[52px]",
        div!(
            class =
                "flex justify-between items-center h-full w-full py-3 px-8 bg-zinc-800 text-white",
            div!(
                class = "flex items-center gap-4 md:gap-6",
                a!(
                    href = "/realestate",
                    class = "cursor-pointer hover:text-zinc-300",
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
                            span!(class = "text-red-400 font-light", "IFUL")
                        )
                    )
                )
            ),
            p!(
                class = "text-zinc-200 hidden md:block",
                "Discover your perfect rental apartment with out advance search"
            ),
            if let Some(user) = &props.user_info {
                (
                    PreEscaped(
                        r#"
                            <script defer type="module">
                                import { setupUserDropdown } from "/assets/js/realestate/home/user-dropdown.js"
                                setupUserDropdown()
                            </script>
                        "#,
                    ),
                    match props.is_dashboard_page {
                        true => {
                            if user.rs_role == "manager" {
                                a!(
                                    href = "/realestate/managers/newproperty",
                                    class = "bg-zinc-50 text-zinc-700 px-3 py-2 rounded-md hover:opacity-80",
                                    div!(
                                        class = "flex gap-2 items-center",
                                        img!(
                                            class = "w-4 h-4",
                                            src = "/assets/images/real-estate/plus.svg",
                                            alt = "plus"
                                        ),
                                        span!("Add New Property")
                                    )
                                )
                            } else {
                                a!(
                                    href = "/realestate/search",
                                    class = "bg-zinc-50  text-zinc-700  px-3 py-2 rounded-md hover:opacity-80",
                                    div!(
                                        class = "flex gap-2 items-center",
                                        img!(
                                            class = "w-4 h-4",
                                            src = "/assets/images/real-estate/search.svg",
                                            alt = "search"
                                        ),
                                        span!("Search Properties")
                                    )
                                )
                            }
                        }
                        false => PreEscaped(""),
                    },
                    div!(
                        id = "user-dropdown",
                        class = "flex items-center gap-2 cursor-pointer relative",
                        img!(
                            class = "rounded-full h-7 w-7",
                            src = &user.image_url,
                            alt = "avatar"
                        ),
                        span!(id = "user-username", &user.username),
                        span!(PreEscaped("&#11167;")),
                        div!(
                            id = "user-dropdown-options",
                            class = "hidden w-max top-9 right-0 text-black absolute flex-col gap-1 bg-white px-3 py-2 border border-neutral-600 rounded-md",
                            div!(
                                class = "flex flex-col bg-white text-zinc-700",
                                a!(
                                    class = "hover:bg-zinc-700 hover:text-zinc-100 py-1 px-2 rounded-md w-full",
                                    name = "dropdown-item",
                                    href = if user.rs_role == "manager" {
                                        "/realestate/manager/properties"
                                    } else {
                                        "/realestate/tenant/favorites"
                                    },
                                    "Go to dashboard"
                                ),
                                a!(
                                    class = "hover:bg-zinc-700 hover:text-zinc-100 py-1 px-2 rounded-md w-full",
                                    name = "dropdown-item",
                                    href = format!("/realestate/{}/settings", user.rs_role),
                                    "Setting"
                                ),
                                form!(
                                    class = "w-full",
                                    name = "dropdown-item",
                                    "hx-post" = "/auth/logout",
                                    "hx-swap" = "none",
                                    button!(
                                        class = "text-start hover:bg-zinc-700 hover:text-zinc-100 py-1 px-2 rounded-md w-full",
                                        "type" = "submit",
                                        "Sign out"
                                    )
                                )
                            ),
                        ),
                    ),
                )
            } else {
                (
                    PreEscaped(
                        r#"
                            <script defer type="module">
                                import { setupAuthModal} from "/assets/js/realestate/home/auth-modal.js"
                                setupAuthModal()
                            </script>
                        "#,
                    ),
                    div!(
                        class = "flex items-center gap-5",
                        button!(
                            id = "sign-in-button",
                            class = "text-white border-white bg-transparent hover:bg-white hover:text-zinc-700 rounded-lg cursor-pointer py-1.5 px-3 border",
                            "Sign In"
                        ),
                        render_login_modal(),
                        render_register_modal(),
                    ),
                )
            }
        )
    )
}
