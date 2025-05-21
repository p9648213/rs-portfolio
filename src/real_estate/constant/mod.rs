use super::views::pages::home_v::DiscoverItem;

pub struct UserNavItem<'a> {
    pub img: &'a str,
    pub label: &'a str,
    pub href: &'a str,
}

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

pub const MANAGER_NAV: [UserNavItem; 2] = [
    UserNavItem {
        img: "/assets/images/real-estate/building.svg",
        label: "Properties",
        href: "realestate/manager/property",
    },
    UserNavItem {
        img: "/assets/images/real-estate/file-text.svg",
        label: "Applications",
        href: "/realestate/manager/applications",
    },
];

pub const TENANT_NAV: [UserNavItem; 4] = [
    UserNavItem {
        img: "/assets/images/real-estate/heart.svg",
        label: "Favorites",
        href: "/realestate/tenant/favorites",
    },
    UserNavItem {
        img: "/assets/images/real-estate/file-text.svg",
        label: "Applications",
        href: "/realestate/tenant/applications",
    },
    UserNavItem {
        img: "/assets/images/real-estate/house.svg",
        label: "Residences",
        href: "/realestate/tenant/residences",
    },
    UserNavItem {
        img: "/assets/images/real-estate/settings.svg",
        label: "Settings",
        href: "/realestate/tenant/settings",
    },
];
