//! Product name generation.

use rand::Rng;

/// Product adjectives
pub const PRODUCT_ADJECTIVES: &[&str] = &[
    "Rustic",
    "Handcrafted",
    "Intelligent",
    "Gorgeous",
    "Incredible",
    "Fantastic",
    "Practical",
    "Sleek",
    "Awesome",
    "Generic",
    "Unbranded",
    "Tasty",
    "Refined",
    "Modern",
    "Elegant",
    "Ergonomic",
    "Licensed",
    "Recycled",
    "Small",
    "Luxurious",
    "Premium",
    "Professional",
    "Custom",
    "Vintage",
    "Classic",
];

/// Product materials
pub const PRODUCT_MATERIALS: &[&str] = &[
    "Steel", "Wooden", "Concrete", "Plastic", "Cotton", "Granite", "Rubber", "Leather", "Silk",
    "Metal", "Soft", "Fresh", "Frozen", "Aluminum", "Bronze", "Copper", "Glass", "Paper", "Wool",
    "Bamboo",
];

/// Product categories
pub const PRODUCT_CATEGORIES: &[&str] = &[
    "Electronics",
    "Clothing",
    "Furniture",
    "Books",
    "Sports",
    "Toys",
    "Games",
    "Health",
    "Beauty",
    "Garden",
    "Automotive",
    "Computers",
    "Baby",
    "Grocery",
    "Industrial",
    "Jewelry",
    "Home",
    "Movies",
    "Music",
    "Office",
    "Outdoors",
    "Pet Supplies",
    "Shoes",
    "Software",
    "Tools",
];

/// Product nouns
const PRODUCT_NOUNS: &[&str] = &[
    "Chair", "Car", "Computer", "Keyboard", "Mouse", "Bike", "Ball", "Gloves", "Pants", "Shirt",
    "Table", "Shoes", "Hat", "Towels", "Soap", "Tuna", "Chicken", "Fish", "Cheese", "Bacon",
    "Pizza", "Salad", "Sausages", "Chips", "Watch", "Bag", "Phone", "Lamp", "Bottle", "Book",
];

/// Generate a product name.
///
/// # Example
/// ```
/// use dx_datagen::commerce::product_name;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let name = product_name(&mut rng);
/// assert!(!name.is_empty());
/// ```
pub fn product_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let adj = PRODUCT_ADJECTIVES[rng.random_range(0..PRODUCT_ADJECTIVES.len())];
    let material = PRODUCT_MATERIALS[rng.random_range(0..PRODUCT_MATERIALS.len())];
    let noun = PRODUCT_NOUNS[rng.random_range(0..PRODUCT_NOUNS.len())];
    format!("{} {} {}", adj, material, noun)
}

/// Get a random product adjective.
pub fn product_adjective<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    PRODUCT_ADJECTIVES[rng.random_range(0..PRODUCT_ADJECTIVES.len())]
}

/// Get a random product material.
pub fn product_material<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    PRODUCT_MATERIALS[rng.random_range(0..PRODUCT_MATERIALS.len())]
}

/// Get a random product category.
pub fn product_category<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    PRODUCT_CATEGORIES[rng.random_range(0..PRODUCT_CATEGORIES.len())]
}

/// Generate a random price.
///
/// # Example
/// ```
/// use dx_datagen::commerce::price;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let p = price(&mut rng, 1.0, 1000.0);
/// assert!(p >= 1.0 && p <= 1000.0);
/// ```
pub fn price<R: ?Sized + Rng>(rng: &mut R, min: f64, max: f64) -> f64 {
    let raw = rng.random_range(min..=max);
    (raw * 100.0).round() / 100.0 // Round to 2 decimal places
}

/// Generate a formatted price with currency symbol.
///
/// # Example
/// ```
/// use dx_datagen::commerce::price_formatted;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let p = price_formatted(&mut rng, "$", 1.0, 100.0);
/// assert!(p.starts_with('$'));
/// ```
pub fn price_formatted<R: ?Sized + Rng>(rng: &mut R, symbol: &str, min: f64, max: f64) -> String {
    let p = price(rng, min, max);
    format!("{}{:.2}", symbol, p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_product_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = product_name(&mut rng);
        assert!(!name.is_empty());
        // Should have 3 words
        assert_eq!(name.split(' ').count(), 3);
    }

    #[test]
    fn test_product_adjective() {
        let mut rng = StdRng::seed_from_u64(42);
        let adj = product_adjective(&mut rng);
        assert!(PRODUCT_ADJECTIVES.contains(&adj));
    }

    #[test]
    fn test_product_category() {
        let mut rng = StdRng::seed_from_u64(42);
        let cat = product_category(&mut rng);
        assert!(PRODUCT_CATEGORIES.contains(&cat));
    }

    #[test]
    fn test_price() {
        let mut rng = StdRng::seed_from_u64(42);
        let p = price(&mut rng, 10.0, 100.0);
        assert!(p >= 10.0 && p <= 100.0);
        // Verify 2 decimal places
        let s = format!("{:.2}", p);
        assert!(s.contains('.'));
    }

    #[test]
    fn test_price_formatted() {
        let mut rng = StdRng::seed_from_u64(42);
        let p = price_formatted(&mut rng, "$", 10.0, 100.0);
        assert!(p.starts_with('$'));
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(product_name(&mut rng1), product_name(&mut rng2));
    }
}
