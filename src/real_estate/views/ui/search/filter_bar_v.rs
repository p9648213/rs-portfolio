use vy::prelude::*;

use crate::real_estate::{
    constant::{BEDS, MAX_PRICE, MIN_PRICE, PROPERTY_TYPE},
    controllers::search_c::SearchQuery,
};

pub fn render_filter_bar(search_query: &SearchQuery) -> impl IntoHtml {
    let location: &str = search_query.location.as_deref().unwrap_or("");
    let min_price = search_query.min_price.as_deref().unwrap_or("none");
    let max_price = search_query.max_price.as_deref().unwrap_or("none");
    let beds = search_query.beds.as_deref().unwrap_or("none");
    let baths = search_query.baths.as_deref().unwrap_or("none");
    let property_type = search_query.property_type.as_deref().unwrap_or("none");

    (
        PreEscaped(
            r#"
                <script defer type="module">
                    import { setupPriceRange, setupSquareFeet, toggleFilterFull } from "/assets/js/realestate/search/search.js";
                    setupPriceRange();
                    setupSquareFeet();
                    toggleFilterFull();
                </script>
            "#,
        ),
        div!(
            class = "flex justify-between items-center py-5 w-full",
            form!(
                "hx-get" = "/realestate/search",
                "hx-target" = "main",
                "hx-trigger" = "submit,change",
                "hx-push-url" = "true",
                class = "flex justify-between items-center gap-4 p-2",
                // All Filters
                button!(
                    id = "all-filter-btn",
                    "type" = "button",
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
                        value = location
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
                        render_beds(beds),
                        // Baths
                        render_baths(baths),
                    ),
                    // Property Type
                    render_property_type(property_type),
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
        ),
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

pub fn render_beds(value: &str) -> impl IntoHtml {
    select!(
        name = "beds",
        class = "border-zinc-400 rounded-md h-8.5 py-0",
        BEDS.map(|bed| {
            let selected = if bed == value { Some(()) } else { None };

            if bed == "none" {
                option!(
                    value = bed,
                    selected? = selected,
                    hidden = true,
                    format!("Beds")
                )
            } else {
                option!(
                    value = bed,
                    selected? = selected,
                    format!(
                        "{}",
                        if bed.is_empty() {
                            "Any beds".to_owned()
                        } else {
                            format!("{}+ beds", bed)
                        }
                    )
                )
            }
        }),
    )
}

pub fn render_baths(value: &str) -> impl IntoHtml {
    select!(
        name = "baths",
        class = "border-zinc-400 rounded-md h-8.5 py-0",
        BEDS.map(|bath| {
            let selected = if bath == value { Some(()) } else { None };

            if bath == "none" {
                option!(
                    value = bath,
                    selected? = selected,
                    hidden = true,
                    format!("Baths")
                )
            } else {
                option!(
                    value = bath,
                    selected? = selected,
                    format!(
                        "{}",
                        if bath.is_empty() {
                            "Any baths".to_owned()
                        } else {
                            format!("{}+ baths", bath)
                        }
                    )
                )
            }
        }),
    )
}

pub fn render_property_type(value: &str) -> impl IntoHtml {
    select!(
        name = "property_type",
        class = "border-zinc-400 rounded-md h-8.5 py-0",
        PROPERTY_TYPE.map(|property_type| {
            let selected = if property_type == value {
                Some(())
            } else {
                None
            };

            if property_type == "none" {
                option!(
                    value = property_type,
                    selected? = selected,
                    hidden = true,
                    format!("Property Type")
                )
            } else {
                option!(
                    value = property_type,
                    selected? = selected,
                    format!(
                        "{}",
                        if property_type.is_empty() {
                            "Any property type"
                        } else {
                            property_type
                        }
                    )
                )
            }
        })
    )
}
