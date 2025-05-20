use vy::prelude::*;

pub fn render_settings_form(username: &str, phone_number: &str) -> impl IntoHtml {
    div!(
        class = "py-5 px-10",
        div!(
            class = "flex flex-col gap-4",
            div!(
                h2!(class = "text-lg font-bold", "Tenant Settings"),
                p!(
                    class = "text-zinc-500",
                    "Manage your account preferences and persional information"
                )
            ),
            form!(
                "hx-patch" = "/realestate/user",
                "hx-swap" = "none",
                "hx-disabled-elt" = "find button",
                class = "p-5 bg-white rounded-md max-w-md",
                div!(
                    class = "flex flex-col gap-4",
                    div!(
                        class = "flex flex-col gap-2",
                        label!("Name: "),
                        input!(
                            class = "rounded-md w-full h-9.5",
                            name = "username",
                            "type" = "text",
                            autocomplete = "on",
                            placeholder = "Name",
                            value = username
                        )
                    ),
                    div!(
                        class = "flex flex-col gap-2",
                        label!("Phone Number: "),
                        input!(
                            class = "rounded-md w-full h-9.5",
                            name = "phone_number",
                            "type" = "text",
                            autocomplete = "on",
                            placeholder = "Phone number",
                            value = phone_number
                        ),
                    ),
                    button!(
                        class = "bg-zinc-400 hover:bg-zinc-500 py-1.5 rounded-md w-full h-9.5 px-5 text-white font-bold",
                        "type" = "submit",
                        "Save"
                    )
                )
            )
        )
    )
}
