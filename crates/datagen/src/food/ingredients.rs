//! Ingredient data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Vegetables.
static VEGETABLES: &[&str] = &[
    "Carrot",
    "Broccoli",
    "Spinach",
    "Tomato",
    "Onion",
    "Garlic",
    "Bell Pepper",
    "Zucchini",
    "Cucumber",
    "Celery",
    "Lettuce",
    "Kale",
    "Cauliflower",
    "Asparagus",
    "Green Beans",
    "Peas",
    "Corn",
    "Potato",
    "Sweet Potato",
    "Mushroom",
    "Eggplant",
    "Cabbage",
    "Brussels Sprouts",
    "Artichoke",
    "Beet",
    "Radish",
    "Turnip",
    "Leek",
    "Shallot",
    "Bok Choy",
];

/// Fruits.
static FRUITS: &[&str] = &[
    "Apple",
    "Banana",
    "Orange",
    "Strawberry",
    "Blueberry",
    "Raspberry",
    "Grape",
    "Mango",
    "Pineapple",
    "Watermelon",
    "Peach",
    "Pear",
    "Cherry",
    "Plum",
    "Kiwi",
    "Lemon",
    "Lime",
    "Grapefruit",
    "Coconut",
    "Papaya",
    "Pomegranate",
    "Fig",
    "Avocado",
    "Passion Fruit",
    "Dragon Fruit",
    "Guava",
    "Lychee",
    "Cantaloupe",
    "Honeydew",
    "Blackberry",
];

/// Meats and proteins.
static MEATS: &[&str] = &[
    "Chicken",
    "Beef",
    "Pork",
    "Lamb",
    "Turkey",
    "Duck",
    "Salmon",
    "Tuna",
    "Shrimp",
    "Lobster",
    "Crab",
    "Cod",
    "Halibut",
    "Tilapia",
    "Scallops",
    "Bacon",
    "Ham",
    "Sausage",
    "Ground Beef",
    "Steak",
    "Veal",
    "Venison",
    "Bison",
    "Rabbit",
    "Tofu",
    "Tempeh",
    "Seitan",
];

/// Spices and seasonings.
static SPICES: &[&str] = &[
    "Salt",
    "Black Pepper",
    "Paprika",
    "Cumin",
    "Coriander",
    "Turmeric",
    "Cinnamon",
    "Nutmeg",
    "Ginger",
    "Garlic Powder",
    "Onion Powder",
    "Oregano",
    "Basil",
    "Thyme",
    "Rosemary",
    "Sage",
    "Bay Leaf",
    "Cayenne",
    "Chili Powder",
    "Curry Powder",
    "Cardamom",
    "Cloves",
    "Allspice",
    "Fennel",
    "Dill",
    "Parsley",
    "Cilantro",
    "Mint",
    "Saffron",
    "Vanilla",
];

/// Common cooking ingredients.
static INGREDIENTS: &[&str] = &[
    "Olive Oil",
    "Butter",
    "Flour",
    "Sugar",
    "Honey",
    "Soy Sauce",
    "Vinegar",
    "Mustard",
    "Mayonnaise",
    "Ketchup",
    "Cream",
    "Milk",
    "Eggs",
    "Cheese",
    "Rice",
    "Pasta",
    "Bread",
    "Broth",
    "Wine",
    "Lemon Juice",
    "Worcestershire Sauce",
    "Hot Sauce",
    "Maple Syrup",
    "Coconut Milk",
    "Sesame Oil",
];

/// Generate a random vegetable.
pub fn vegetable<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, VEGETABLES)
}

/// Generate a random fruit.
pub fn fruit<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, FRUITS)
}

/// Generate a random meat/protein.
pub fn meat<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MEATS)
}

/// Generate a random spice.
pub fn spice<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, SPICES)
}

/// Generate a random cooking ingredient.
pub fn ingredient<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, INGREDIENTS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_vegetable() {
        let mut rng = StdRng::seed_from_u64(42);
        let v = vegetable(&mut rng);
        assert!(VEGETABLES.contains(&v));
    }

    #[test]
    fn test_fruit() {
        let mut rng = StdRng::seed_from_u64(42);
        let f = fruit(&mut rng);
        assert!(FRUITS.contains(&f));
    }

    #[test]
    fn test_meat() {
        let mut rng = StdRng::seed_from_u64(42);
        let m = meat(&mut rng);
        assert!(MEATS.contains(&m));
    }

    #[test]
    fn test_spice() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = spice(&mut rng);
        assert!(SPICES.contains(&s));
    }

    #[test]
    fn test_ingredient() {
        let mut rng = StdRng::seed_from_u64(42);
        let i = ingredient(&mut rng);
        assert!(INGREDIENTS.contains(&i));
    }
}
