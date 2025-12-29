//! Predefined category constants for realistic test data generation.

// Geographic
/// Major world cities
pub const CITIES: &[&str] = &[
    "New York",
    "London",
    "Paris",
    "Tokyo",
    "Sydney",
    "Berlin",
    "Rome",
    "Toronto",
    "Dubai",
    "Singapore",
];

/// Major countries
pub const COUNTRIES: &[&str] = &[
    "USA",
    "UK",
    "France",
    "Germany",
    "Japan",
    "Canada",
    "Australia",
    "Brazil",
    "India",
    "China",
];

// Business
/// Common workflow statuses
pub const STATUSES: &[&str] = &["pending", "active", "completed", "cancelled", "archived"];

/// Priority levels
pub const PRIORITIES: &[&str] = &["low", "medium", "high", "critical"];

/// Common business departments
pub const DEPARTMENTS: &[&str] = &[
    "Engineering",
    "Marketing",
    "Sales",
    "HR",
    "Finance",
    "Support",
    "Operations",
    "Legal",
];

// Product
/// Common fruits
pub const FRUITS: &[&str] = &[
    "apple",
    "banana",
    "orange",
    "grape",
    "mango",
    "strawberry",
    "pineapple",
    "kiwi",
    "peach",
    "cherry",
];

/// Basic colors
pub const COLORS: &[&str] = &[
    "red", "blue", "green", "yellow", "purple", "orange", "pink", "brown", "black", "white",
];

/// Clothing sizes
pub const SIZES: &[&str] = &["XS", "S", "M", "L", "XL", "XXL"];

// Temporal
/// Days of the week
pub const DAYS: &[&str] = &[
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// Months of the year
pub const MONTHS: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Seasons
pub const SEASONS: &[&str] = &["Spring", "Summer", "Autumn", "Winter"];

// Demographics
/// Genders (inclusive set)
pub const GENDERS: &[&str] = &["male", "female", "non-binary", "other", "prefer not to say"];

/// Binary genders (for name generation, etc.)
pub const GENDERS_BINARY: &[&str] = &["male", "female"];

/// Age groups for demographic segmentation
pub const AGE_GROUPS: &[&str] = &[
    "0-12",  // Child
    "13-17", // Teenager
    "18-24", // Young adult
    "25-34", // Adult
    "35-44", // Middle adult
    "45-54", // Middle-aged
    "55-64", // Pre-senior
    "65+",   // Senior
];

/// Named age groups
pub const AGE_GROUP_NAMES: &[&str] = &[
    "child",
    "teenager",
    "young adult",
    "adult",
    "middle-aged",
    "senior",
];

/// Generation names
pub const GENERATIONS: &[&str] = &[
    "Gen Alpha",    // 2013-2024
    "Gen Z",        // 1997-2012
    "Millennials",  // 1981-1996
    "Gen X",        // 1965-1980
    "Baby Boomers", // 1946-1964
    "Silent Gen",   // 1928-1945
];

/// Marital status options
pub const MARITAL_STATUSES: &[&str] = &[
    "single",
    "married",
    "divorced",
    "widowed",
    "separated",
    "domestic partnership",
];

/// Education levels
pub const EDUCATION_LEVELS: &[&str] = &[
    "no formal education",
    "primary school",
    "high school",
    "some college",
    "associate degree",
    "bachelor's degree",
    "master's degree",
    "doctoral degree",
];

/// Employment statuses
pub const EMPLOYMENT_STATUSES: &[&str] = &[
    "employed full-time",
    "employed part-time",
    "self-employed",
    "unemployed",
    "student",
    "retired",
    "homemaker",
];

// Technology
/// Common file extensions
pub const FILE_EXTENSIONS: &[&str] = &[
    "txt", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "jpg", "png", "gif", "mp3", "mp4",
    "zip", "tar", "gz", "json", "xml", "csv", "html", "css", "js", "rs", "py", "java",
];

/// Common MIME types
pub const MIME_TYPES: &[&str] = &[
    "text/plain",
    "text/html",
    "text/css",
    "application/json",
    "application/xml",
    "application/pdf",
    "image/jpeg",
    "image/png",
    "image/gif",
    "audio/mpeg",
    "video/mp4",
];

/// HTTP methods
pub const HTTP_METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

/// HTTP status codes (as strings)
pub const HTTP_STATUS_CODES: &[&str] = &[
    "200", "201", "204", "301", "302", "400", "401", "403", "404", "500", "502", "503",
];

// Finance
/// Major currencies
pub const CURRENCIES: &[&str] = &[
    "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "CNY", "INR", "NOK", "SEK", "DKK",
];

/// Payment methods
pub const PAYMENT_METHODS: &[&str] = &[
    "credit card",
    "debit card",
    "bank transfer",
    "cash",
    "PayPal",
    "Apple Pay",
    "Google Pay",
    "cryptocurrency",
];

/// Transaction types
pub const TRANSACTION_TYPES: &[&str] = &[
    "purchase",
    "refund",
    "transfer",
    "deposit",
    "withdrawal",
    "payment",
    "fee",
];

// E-commerce
/// Product categories
pub const PRODUCT_CATEGORIES: &[&str] = &[
    "electronics",
    "clothing",
    "home & garden",
    "sports",
    "toys",
    "books",
    "food & beverages",
    "health & beauty",
    "automotive",
    "jewelry",
];

/// Order statuses
pub const ORDER_STATUSES: &[&str] = &[
    "pending",
    "confirmed",
    "processing",
    "shipped",
    "in transit",
    "delivered",
    "cancelled",
    "returned",
];

/// Shipping methods
pub const SHIPPING_METHODS: &[&str] = &[
    "standard",
    "express",
    "overnight",
    "same day",
    "pickup",
    "freight",
];
