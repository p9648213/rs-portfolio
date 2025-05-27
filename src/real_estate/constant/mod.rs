use super::views::pages::home_v::DiscoverItem;

pub struct UserNavItem<'a> {
    pub img: &'a str,
    pub label: &'a str,
    pub href: &'a str,
}

pub struct FilterItem<'a> {
    pub title: &'a str,
    pub img: &'a str,
    pub alt: &'a str,
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

pub const PROPERTY_TYPE: [FilterItem; 6] = [
    FilterItem {
        title: "Rooms",
        img: "/assets/images/real-estate/house.svg",
        alt: "rooms",
    },
    FilterItem {
        title: "Tinyhouse",
        img: "/assets/images/real-estate/warehouse.svg",
        alt: "tinyhouse",
    },
    FilterItem {
        title: "Apartment",
        img: "/assets/images/real-estate/building.svg",
        alt: "apartment",
    },
    FilterItem {
        title: "Villa",
        img: "/assets/images/real-estate/castle.svg",
        alt: "villa",
    },
    FilterItem {
        title: "Townhouse",
        img: "/assets/images/real-estate/house.svg",
        alt: "townhouse",
    },
    FilterItem {
        title: "Cottage",
        img: "/assets/images/real-estate/trees.svg",
        alt: "cottage",
    },
];

pub const AMENITY: [FilterItem; 13] = [
    FilterItem {
        title: "Washer Dryer",
        img: "/assets/images/real-estate/waves.svg",
        alt: "washer dryer",
    },
    FilterItem {
        title: "Air Conditioning",
        img: "/assets/images/real-estate/thermometer.svg",
        alt: "air conditioning",
    },
    FilterItem {
        title: "Dish Washer",
        img: "/assets/images/real-estate/waves.svg",
        alt: "dish washer",
    },
    FilterItem {
        title: "High Speed Internet",
        img: "/assets/images/real-estate/wifi.svg",
        alt: "high speed internet",
    },
    FilterItem {
        title: "Hardwood Floors",
        img: "/assets/images/real-estate/house.svg",
        alt: "hardwood floors",
    },
    FilterItem {
        title: "Walk In Closets",
        img: "/assets/images/real-estate/maximize.svg",
        alt: "walk in closets",
    },
    FilterItem {
        title: "Microwave",
        img: "/assets/images/real-estate/tv.svg",
        alt: "microwave",
    },
    FilterItem {
        title: "Refrigerator",
        img: "/assets/images/real-estate/thermometer.svg",
        alt: "refrigerator",
    },
    FilterItem {
        title: "Pool",
        img: "/assets/images/real-estate/waves.svg",
        alt: "pool",
    },
    FilterItem {
        title: "Gym",
        img: "/assets/images/real-estate/dumbbell.svg",
        alt: "gym",
    },
    FilterItem {
        title: "Parking",
        img: "/assets/images/real-estate/car.svg",
        alt: "parking",
    },
    FilterItem {
        title: "Pets Allowed",
        img: "/assets/images/real-estate/paw-print.svg",
        alt: "pets allowed",
    },
    FilterItem {
        title: "WiFi",
        img: "/assets/images/real-estate/wifi.svg",
        alt: "wifi",
    },
];
