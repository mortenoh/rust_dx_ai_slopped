//! Dish and cuisine data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Dish names by category.
static APPETIZERS: &[&str] = &[
    "Bruschetta",
    "Calamari",
    "Spring Rolls",
    "Caprese Salad",
    "Shrimp Cocktail",
    "Stuffed Mushrooms",
    "Nachos",
    "Hummus Platter",
    "Chicken Wings",
    "Mozzarella Sticks",
    "Crab Cakes",
    "Deviled Eggs",
    "Spinach Dip",
    "Quesadilla",
    "Edamame",
];

static MAIN_COURSES: &[&str] = &[
    "Grilled Salmon",
    "Beef Tenderloin",
    "Chicken Parmesan",
    "Lobster Tail",
    "Lamb Chops",
    "Spaghetti Carbonara",
    "Pad Thai",
    "Butter Chicken",
    "Beef Bourguignon",
    "Fish and Chips",
    "Paella",
    "Ramen",
    "Bibimbap",
    "Tacos al Pastor",
    "Duck Confit",
    "Risotto",
    "Lasagna",
    "Coq au Vin",
    "Fajitas",
    "Kung Pao Chicken",
    "Teriyaki Chicken",
    "Pho",
    "Tikka Masala",
    "Schnitzel",
    "Moussaka",
];

static DESSERTS: &[&str] = &[
    "Tiramisu",
    "Cheesecake",
    "Chocolate Mousse",
    "Crème Brûlée",
    "Apple Pie",
    "Panna Cotta",
    "Brownie Sundae",
    "Key Lime Pie",
    "Churros",
    "Baklava",
    "Gelato",
    "Macarons",
    "Profiteroles",
    "Lemon Tart",
    "Red Velvet Cake",
    "Chocolate Lava Cake",
    "Carrot Cake",
    "Banana Split",
    "Bread Pudding",
    "Fruit Tart",
];

static SOUPS: &[&str] = &[
    "Tomato Soup",
    "French Onion Soup",
    "Chicken Noodle Soup",
    "Minestrone",
    "Clam Chowder",
    "Lobster Bisque",
    "Miso Soup",
    "Gazpacho",
    "Pho",
    "Tom Yum",
    "Butternut Squash Soup",
    "Lentil Soup",
    "Split Pea Soup",
    "Hot and Sour Soup",
    "Borscht",
];

static SALADS: &[&str] = &[
    "Caesar Salad",
    "Greek Salad",
    "Cobb Salad",
    "Waldorf Salad",
    "Caprese Salad",
    "Nicoise Salad",
    "Garden Salad",
    "Spinach Salad",
    "Arugula Salad",
    "Kale Salad",
    "Asian Salad",
    "Taco Salad",
    "Pasta Salad",
    "Potato Salad",
    "Coleslaw",
];

/// Cuisines.
static CUISINES: &[&str] = &[
    "Italian",
    "French",
    "Chinese",
    "Japanese",
    "Mexican",
    "Indian",
    "Thai",
    "Vietnamese",
    "Korean",
    "Greek",
    "Spanish",
    "American",
    "Brazilian",
    "Peruvian",
    "Ethiopian",
    "Moroccan",
    "Lebanese",
    "Turkish",
    "German",
    "British",
    "Caribbean",
    "Filipino",
    "Indonesian",
    "Malaysian",
    "Singaporean",
    "Australian",
    "Cajun",
    "Soul Food",
    "Mediterranean",
    "Middle Eastern",
];

/// Meal types.
static MEAL_TYPES: &[&str] = &[
    "Breakfast",
    "Brunch",
    "Lunch",
    "Dinner",
    "Snack",
    "Appetizer",
    "Main Course",
    "Side Dish",
    "Dessert",
    "Midnight Snack",
];

/// Generate a random dish name.
pub fn dish<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let all_dishes: Vec<&str> = APPETIZERS
        .iter()
        .chain(MAIN_COURSES)
        .chain(SOUPS)
        .chain(SALADS)
        .copied()
        .collect();
    pick(rng, &all_dishes)
}

/// Generate a random dessert.
pub fn dessert<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DESSERTS)
}

/// Generate a random cuisine type.
pub fn cuisine<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CUISINES)
}

/// Generate a random meal type.
pub fn meal_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MEAL_TYPES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_dish() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = dish(&mut rng);
        assert!(!d.is_empty());
    }

    #[test]
    fn test_dessert() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = dessert(&mut rng);
        assert!(DESSERTS.contains(&d));
    }

    #[test]
    fn test_cuisine() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = cuisine(&mut rng);
        assert!(CUISINES.contains(&c));
    }

    #[test]
    fn test_meal_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let m = meal_type(&mut rng);
        assert!(MEAL_TYPES.contains(&m));
    }
}
