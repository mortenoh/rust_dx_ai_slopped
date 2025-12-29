//! Restaurant data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Restaurant name patterns.
static NAME_PATTERNS: &[&str] = &[
    "The {adj} {noun}",
    "{owner}'s {type}",
    "Café {noun}",
    "The {noun} House",
    "{adj} {type}",
    "{noun} & {noun}",
    "Little {noun}",
    "Golden {noun}",
    "Blue {noun}",
    "The {noun} Kitchen",
    "{noun} Grill",
    "{noun} Bistro",
    "House of {noun}",
    "La {noun}",
    "Il {noun}",
];

static ADJECTIVES: &[&str] = &[
    "Golden", "Silver", "Royal", "Grand", "Little", "Old", "New", "Rustic", "Modern", "Classic",
    "Urban", "Country", "Cozy", "Elegant", "Hidden",
];

static NOUNS: &[&str] = &[
    "Dragon", "Phoenix", "Garden", "Moon", "Sun", "Star", "Oak", "Olive", "Vine", "Rose", "Bell",
    "Crown", "Lion", "Eagle", "Harbor", "Lantern", "Willow", "Pepper", "Basil", "Sage",
];

static OWNERS: &[&str] = &[
    "Mario", "Luigi", "Antonio", "Giuseppe", "Marco", "Maria", "Rosa", "Chen", "Wong", "Kim",
    "Park", "Garcia", "Martinez", "Pierre", "Jacques", "Hans", "Otto", "Yuki", "Kenji", "Raj",
];

static RESTAURANT_TYPES_LIST: &[&str] = &[
    "Kitchen",
    "Bistro",
    "Grill",
    "Tavern",
    "Café",
    "Diner",
    "Eatery",
    "Trattoria",
    "Brasserie",
    "Cantina",
    "Pizzeria",
    "Steakhouse",
    "Gastropub",
    "Bakery",
    "Patisserie",
];

/// Restaurant types.
static RESTAURANT_TYPES: &[&str] = &[
    "Fine Dining",
    "Casual Dining",
    "Fast Casual",
    "Fast Food",
    "Café",
    "Bistro",
    "Brasserie",
    "Trattoria",
    "Pizzeria",
    "Steakhouse",
    "Seafood Restaurant",
    "Sushi Bar",
    "Ramen Shop",
    "Dim Sum",
    "Taqueria",
    "Food Truck",
    "Deli",
    "Bakery",
    "Patisserie",
    "Ice Cream Parlor",
    "Juice Bar",
    "Wine Bar",
    "Gastropub",
    "Sports Bar",
    "Buffet",
    "Family Restaurant",
    "Pop-up Restaurant",
    "Ghost Kitchen",
    "Diner",
    "Tavern",
];

/// Generate a random restaurant name.
pub fn restaurant_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, NAME_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, ADJECTIVES))
        .replace("{noun}", pick(rng, NOUNS))
        .replace("{owner}", pick(rng, OWNERS))
        .replace("{type}", pick(rng, RESTAURANT_TYPES_LIST))
}

/// Generate a random restaurant type.
pub fn restaurant_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, RESTAURANT_TYPES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_restaurant_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = restaurant_name(&mut rng);
        assert!(!name.is_empty());
        assert!(!name.contains('{'));
    }

    #[test]
    fn test_restaurant_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = restaurant_type(&mut rng);
        assert!(RESTAURANT_TYPES.contains(&t));
    }
}
