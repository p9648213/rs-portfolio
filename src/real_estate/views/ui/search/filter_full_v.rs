use vy::prelude::*;

use crate::real_estate::{
    constant::{AMENITY, PROPERTY_TYPE_FF},
    controllers::search_c::SearchQuery,
    views::ui::search::filter_bar_v::{render_baths, render_beds},
};

pub fn render_filter_full(search_query: &SearchQuery) -> impl IntoHtml {
    let location: &str = search_query.location.as_deref().unwrap_or("");
    let min_price = search_query.min_price.as_deref().unwrap_or("0");
    let max_price = search_query.max_price.as_deref().unwrap_or("10000");
    let min_square = search_query.min_square.as_deref().unwrap_or("0");
    let max_square = search_query.max_square.as_deref().unwrap_or("5000");
    let beds = search_query.beds.as_deref().unwrap_or("none");
    let baths = search_query.baths.as_deref().unwrap_or("none");
    let property_type = search_query.property_type.as_deref().unwrap_or("Rooms");
    let amenity = search_query.amenity.as_deref().unwrap_or("Washer Dryer");
    let available_date = search_query.available_date.as_deref().unwrap_or("");

    (
        PreEscaped(
            r#"
                <script defer type="module">
                    import { setupPriceRange, setupSquareFeet } from "/assets/js/realestate/search/search.js";
                    setupPriceRange();
                    setupSquareFeet();
                </script>
            "#,
        ),
        link!(rel = "stylesheet", href = "/assets/css/lib/dual-range.css"),
        form!(
            "hx-get" = "/realestate/search",
            "hx-target" = "main",
            "hx-trigger" = "submit",
            "hx-push-url" = "true",
            "hx-vals" = r#"
                js:{
                    property_type: getFilterFullPropertyType(),
                    amenity: getFilterFullAmenity()
                }
            "#,
            class = "bg-white px-4 pb-10 rounded-md h-full overflow-auto",
            div!(
                class = "flex flex-col space-y-6",
                // Location
                div!(
                    h4!(class = "mb-2 font-bold", "Location"),
                    div!(
                        class = "flex items-center",
                        input!(
                            name = "location",
                            class = "border-zinc-400  rounded-md h-8.5 px-2 w-full",
                            placeholder = "Search Location",
                            value = location
                        ),
                    ),
                ),
                // Property Type
                div!(
                    h4!(class = "mb-2 font-bold", "Property Type"),
                    render_property_type(property_type)
                ),
                // Price Range
                render_price_range(min_price, max_price),
                // Beds and Baths
                div!(
                    class = "flex gap-4",
                    div!(
                        class = "flex-1",
                        h4!(class = "mb-2 font-bold", "Beds"),
                        render_beds(beds)
                    ),
                    div!(
                        class = "flex-1",
                        h4!(class = "mb-2 font-bold", "Baths"),
                        render_baths(baths),
                    )
                ),
                // Square Feet
                render_square_range(min_square, max_square),
                // Amenities
                div!(
                    h4!(class = "mb-2 font-bold", "Amenities"),
                    render_ameniy(amenity)
                ),
                // Available From
                div!(
                    h4!(class = "mb-2 font-bold", "Available From"),
                    input!(
                        "type" = "date",
                        class = "rounded-md w-full",
                        name = "available_date",
                        value = available_date
                    )
                ),
                // Apply and Reset buttons
                div!(
                    class = "flex gap-4 mt-6",
                    button!(
                        "type" = "submit",
                        class =
                            "flex-1 bg-zinc-700 hover:opacity-80 px-3 rounded-md h-8.5 text-white",
                        "Apply"
                    ),
                    button!(
                        "hx-get" = "/realestate/search",
                        "hx-target" = "main",
                        "hx-params" = "none",
                        "type" = "button",
                        class =
                            "flex-1 hover:opacity-80 px-3 border border-zinc-400 rounded-md h-8.5",
                        "Reset Filters"
                    )
                )
            )
        ),
    )
}

pub fn render_property_type(highlight: &str) -> impl IntoHtml {
    div!(
        id="ff_property_type",
        class = "gap-4 grid grid-cols-2",
        PROPERTY_TYPE_FF.map(|property| {
            if highlight == property.title {
                button!(
                    "hx-get" = format!("/realestate/ui/search/property_type/{}", property.title),
                    disabled = "true",
                    id = "selected_property",
                    class = "flex flex-col items-center justify-center p-4 rounded-md border border-zinc-950",
                    img!(
                        class = "mb-2 w-6 h-6",
                        src = property.img,
                        alt = property.alt
                    ),
                    span!(property.title)
                )
            } else {
                button!(
                    "hx-get" = format!("/realestate/ui/search/property_type/{}", property.title),
                    "hx-target" = "#ff_property_type",
                    "hx-swap" = "outerHTML",
                    "hx-push-url"="false",
                    "type" = "button",
                    class = "flex flex-col items-center justify-center p-4 rounded-md border border-zinc-300",
                    img!(
                        class = "mb-2 w-6 h-6",
                        src = property.img,
                        alt = property.alt
                    ),
                    span!(property.title)
                )
            }
        })
    )
}

pub fn render_ameniy(highlight: &str) -> impl IntoHtml {
    div!(
        id = "ff_amenity",
        class = "flex flex-wrap gap-2",
        AMENITY.map(|amenity| {
            if highlight == amenity.title {
                button!(
                    "hx-get" = format!("/realestate/ui/search/amenity/{}", amenity.title),
                    disabled = "true",
                    id = "selected_amenity",
                    class = "flex items-center space-x-2 p-2 border border-zinc-950 rounded-lg",
                    img!(class = "w-5 h-5", src = amenity.img, alt = amenity.alt),
                    span!(amenity.title)
                )
            } else {
                button!(
                    "hx-get" = format!("/realestate/ui/search/amenity/{}", amenity.title),
                    "hx-target" = "#ff_amenity",
                    "hx-swap" = "outerHTML",
                    "hx-push-url" = "false",
                    "type" = "button",
                    class = "flex items-center space-x-2 p-2 border border-zinc-300 rounded-lg",
                    img!(class = "w-5 h-5", src = amenity.img, alt = amenity.alt),
                    span!(amenity.title)
                )
            }
        })
    )
}

pub fn render_price_range(min_price: &str, max_price: &str) -> impl IntoHtml {
    div!(
        h4!(class = "mb-2 font-bold", "Price Range"),
        div!(
            class = "dual-range-input",
            input!(
                id = "min_price",
                name = "min_price",
                "type" = "range",
                min = "0",
                max = "10000",
                step = "100",
                value = min_price
            ),
            input!(
                id = "max_price",
                name = "max_price",
                "type" = "range",
                min = "0",
                max = "10000",
                step = "100",
                value = max_price
            )
        ),
        div!(
            class = "flex justify-between gap-2",
            span!(id = "min_price_value", format!("{}$", min_price)),
            span!(id = "max_price_value", format!("{}$", max_price))
        )
    )
}

pub fn render_square_range(min_square: &str, max_square: &str) -> impl IntoHtml {
    div!(
        h4!(class = "mb-2 font-bold", "Square Feet"),
        div!(
            class = "dual-range-input",
            input!(
                name = "min_square",
                id = "min_square",
                "type" = "range",
                min = "0",
                max = "5000",
                step = "100",
                value = min_square,
            ),
            input!(
                name = "max_square",
                id = "max_square",
                "type" = "range",
                min = "0",
                max = "5000",
                step = "100",
                value = max_square,
            )
        ),
        div!(
            class = "flex justify-between gap-2",
            span!(id = "min_square_value", format!("{} sq ft", min_square)),
            span!(id = "max_square_value", format!("{} sq ft", max_square))
        )
    )
}
