use super::views::pages::home_v::DiscoverItem;

pub const FEATURES_TITLES: [&str; 3] = [
    "Trustworthy and Verified Listings",
    "Browse Rental Listings With Ease",
    "Simplify Your Rental Search With Advanced",
];

pub const FEATURES_DESCRIPTIONS: [&str; 3] = [
    "Discover the best rental options with user reviews and ratings",
    "Get access to user reviews and ratings for a better understanding of rental options.",
    "Find trustworthy and verified rental listings to ensure a hassle-free experience.",
];

pub const FEATURES_LINK_TEXTS: [&str; 3] = ["Explore", "Search", "Discover"];

pub const FEATURES_LINK_HREFS: [&str; 3] = ["/explore", "/search", "/discover"];

pub const FEATURES_IMAGE_PATHS: [&str; 3] = [
    "/assets/images/real-estate/landing-search1.png",
    "/assets/images/real-estate/landing-search2.png",
    "/assets/images/real-estate/landing-search3.png",
];

pub const DISCOVER_ITEMS: [DiscoverItem; 3] = [
    DiscoverItem {
        image_src: "/assets/images/real-estate/landing-icon-wand.png",
        title: "Search for Properties",
        description: "Browse through our extensive collection of rental property in your desired location.",
    },
    DiscoverItem {
        image_src: "/assets/images/real-estate/landing-icon-calendar.png",
        title: "Book Your Rental",
        description: "Once you've found the perfect rental property, easily book it online with just a few clicks.",
    },
    DiscoverItem {
        image_src: "/assets/images/real-estate/landing-icon-heart.png",
        title: "Enjoy your New Home",
        description: "Move into your new rental property and start enjoying your dream home.",
    },
];
