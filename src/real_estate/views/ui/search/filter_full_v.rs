use vy::prelude::*;

use crate::real_estate::constant::{AMENITY, PROPERTY_TYPE};

pub fn render_filter_full() -> impl IntoHtml {
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
            "hx-swap" = "none",
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
                            placeholder = "Search Location"
                        ),
                    ),
                ),
                // Property Type
                div!(
                    h4!(class = "mb-2 font-bold", "Property Type"),
                    render_property_type("Rooms")
                ),
                // Price Range
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
                            value = "0"
                        ),
                        input!(
                            id = "max_price",
                            name = "max_price",
                            "type" = "range",
                            min = "0",
                            max = "10000",
                            step = "100",
                            value = "10000"
                        )
                    ),
                    div!(
                        class = "flex justify-between gap-2",
                        span!(id = "min_price_value", "0$"),
                        span!(id = "max_price_value", "10000$")
                    )
                ),
                // Beds and Baths
                div!(
                    class = "flex gap-4",
                    div!(
                        class = "flex-1",
                        h4!(class = "mb-2 font-bold", "Beds"),
                        select!(
                            name = "beds",
                            class = "border-zinc-400 rounded-md h-8.5 py-0 w-full",
                            option!(value = "", selected = "true", hidden = "true", "Beds"),
                            option!(value = "", "Any beds"),
                            option!(value = "1", "1+ beds"),
                            option!(value = "2", "2+ beds"),
                            option!(value = "3", "3+ beds"),
                            option!(value = "4", "4+ beds"),
                        ),
                    ),
                    div!(
                        class = "flex-1",
                        h4!(class = "mb-2 font-bold", "Baths"),
                        select!(
                            name = "baths",
                            class = "border-zinc-400 rounded-md h-8.5 py-0 w-full",
                            option!(value = "", selected = "true", hidden = "true", "Baths"),
                            option!(value = "", "Any baths"),
                            option!(value = "1", "1+ baths"),
                            option!(value = "2", "2+ baths"),
                            option!(value = "3", "3+ baths"),
                        ),
                    )
                ),
                // Square Feet
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
                            value = "0",
                        ),
                        input!(
                            name = "max_square",
                            id = "max_square",
                            "type" = "range",
                            min = "0",
                            max = "5000",
                            step = "100",
                            value = "5000",
                        )
                    ),
                    div!(
                        class = "flex justify-between gap-2",
                        span!(id = "min_square_value", "0 sq ft"),
                        span!(id = "max_square_value", "5000 sq ft")
                    )
                ),
                // Amenities
                div!(
                    h4!(class = "mb-2 font-bold", "Amenities"),
                    render_ameniy("Washer Dryer")
                ),
                // Available From
                div!(
                    h4!(class = "mb-2 font-bold", "Available From"),
                    input!(
                        name = "available_date",
                        "type" = "date",
                        class = "rounded-md w-full",
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
        PROPERTY_TYPE.map(|property| {
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
