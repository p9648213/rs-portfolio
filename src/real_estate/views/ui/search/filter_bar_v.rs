use vy::prelude::*;

use crate::real_estate::{
    constant::{MAX_PRICE, MIN_PRICE},
    controllers::search_c::SearchQuery,
};

pub fn render_filter_bar(search_query: &SearchQuery) -> impl IntoHtml {
    let location: &str = search_query.location.as_deref().unwrap_or("");
    let min_price = search_query.min_price.as_deref().unwrap_or("none");
    let max_price = search_query.max_price.as_deref().unwrap_or("none");

    div!(
        class = "flex justify-between items-center py-5 w-full",
        form!(
            "hx-get" = "/realestate/search",
            "hx-swap" = "none",
            "hx-trigger" = "submit,change",
            "hx-push-url" = "true",
            class = "flex justify-between items-center gap-4 p-2",
            // All Filters
            button!(
                class = "w-23 flex items-center gap-2 px-2 h-8.5 border border-zinc-400 rounded-md hover:opacity-80",
                img!(
                    class = "w-4 h-4",
                    src = "/assets/images/real-estate/funnel.svg",
                    alt = "all-filter"
                ),
                span!("All Filter")
            ),
            // Search Location
            div!(
                class = "flex items-center",
                input!(
                    class = "border-zinc-400 rounded-r-none rounded-l-md w-40 h-8.5 px-2",
                    name = "location",
                    placeholder = "Search Location",
                ),
                button!(
                    "type" = "button",
                    class = "w-8 border border-zinc-400 border-l-0 rounded-r-md rounded-l-none h-8.5 px-2",
                    img!(
                        class = "w-4 h-4",
                        src = "/assets/images/real-estate/search.svg",
                        alt = "search-location"
                    ),
                )
            ),
            // Price Range
            div!(
                class = "flex gap-1",
                // Minimum Price Selector
                render_min_price(min_price),
                // Maximum Price Selector
                render_max_price(max_price), // Beds and Baths
                div!(
                    class = "flex gap-1",
                    // Beds
                    select!(
                        name = "beds",
                        class = "border-zinc-400 rounded-md h-8.5 py-0",
                        option!(value = "", selected = "true", hidden = "true", "Beds"),
                        option!(value = "", "Any beds"),
                        option!(value = "1", "1+ beds"),
                        option!(value = "2", "2+ beds"),
                        option!(value = "3", "3+ beds"),
                        option!(value = "4", "4+ beds"),
                    ),
                    // Baths
                    select!(
                        name = "baths",
                        class = "border-zinc-400 rounded-md h-8.5 py-0",
                        option!(value = "", selected = "true", hidden = "true", "Baths"),
                        option!(value = "", "Any baths"),
                        option!(value = "1", "1+ baths"),
                        option!(value = "2", "2+ baths"),
                        option!(value = "3", "3+ baths"),
                    ),
                ),
                // Property Type
                select!(
                    name = "property_type",
                    class = "border-zinc-400 rounded-md h-8.5 py-0",
                    option!(
                        value = "",
                        selected = "true",
                        hidden = "true",
                        "Property Type"
                    ),
                    option!(value = "", "Any Property Type"),
                    option!(value = "rooms", "Rooms"),
                    option!(value = "tinyhouse", "Tinyhouse"),
                    option!(value = "apartment", "Apartment"),
                    option!(value = "villa", "Villa"),
                    option!(value = "townhouse", "Townhouse"),
                    option!(value = "cottage", "Cottage"),
                ),
            )
        ),
        // View Mode
        div!(
            class = "flex justify-between items-center gap-4 p-2",
            div!(
                class = "flex border border-zinc-400 rounded-md",
                button!(
                    class = "rounded-none rounded-l-md h-8.5 py-0 px-2",
                    img!(
                        class = "w-5 h-5",
                        src = "/assets/images/real-estate/list.svg",
                        alt = "list"
                    )
                ),
                button!(
                    class = "rounded-none rounded-r-md h-8.5 py-0 px-2 bg-zinc-300",
                    img!(
                        class = "w-5 h-5",
                        src = "/assets/images/real-estate/grid-3x3.svg",
                        alt = "list"
                    )
                )
            )
        )
    )
}

pub fn render_min_price(value: &str) -> impl IntoHtml {
    select!(
        name = "min_price",
        class = "border-zinc-400 rounded-md h-8.5 py-0",
        MIN_PRICE.map(|price| {
            let selected = if price == value { Some(()) } else { None };

            if price == "none" {
                option!(
                    value = price,
                    selected? = selected,
                    hidden = true,
                    format!("Min Price")
                )
            } else {
                option!(
                    value = price,
                    selected? = selected,
                    format!(
                        "{}",
                        if price.is_empty() {
                            "Any".to_owned()
                        } else {
                            format!("{}$", price)
                        }
                    )
                )
            }
        }),
    )
}

pub fn render_max_price(value: &str) -> impl IntoHtml {
    select!(
        name = "max_price",
        class = "border-zinc-400 rounded-md h-8.5 py-0",
        MAX_PRICE.map(|price| {
            let selected = if price == value { Some(()) } else { None };

            if price == "none" {
                option!(
                    value = price,
                    selected? = selected,
                    hidden = true,
                    format!("Max Price")
                )
            } else {
                option!(
                    value = price,
                    selected? = selected,
                    format!(
                        "{}",
                        if price.is_empty() {
                            "Any".to_owned()
                        } else {
                            format!("{}$", price)
                        }
                    )
                )
            }
        }),
    )
}
