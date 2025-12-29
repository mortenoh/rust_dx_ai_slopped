//! Beverage data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// General beverages.
static BEVERAGES: &[&str] = &[
    "Water",
    "Sparkling Water",
    "Orange Juice",
    "Apple Juice",
    "Grape Juice",
    "Lemonade",
    "Iced Tea",
    "Hot Tea",
    "Coffee",
    "Espresso",
    "Cappuccino",
    "Latte",
    "Milkshake",
    "Smoothie",
    "Hot Chocolate",
    "Soda",
    "Cola",
    "Root Beer",
    "Ginger Ale",
    "Tonic Water",
    "Energy Drink",
    "Sports Drink",
    "Coconut Water",
    "Kombucha",
    "Milk",
];

/// Coffee drinks.
static COFFEE_DRINKS: &[&str] = &[
    "Espresso",
    "Americano",
    "Cappuccino",
    "Latte",
    "Flat White",
    "Macchiato",
    "Mocha",
    "Cortado",
    "Ristretto",
    "Lungo",
    "Cold Brew",
    "Iced Coffee",
    "Frappuccino",
    "Affogato",
    "Irish Coffee",
    "Turkish Coffee",
    "Vietnamese Coffee",
    "Café au Lait",
    "Nitro Cold Brew",
    "Pour Over",
];

/// Tea types.
static TEA_TYPES: &[&str] = &[
    "Green Tea",
    "Black Tea",
    "White Tea",
    "Oolong Tea",
    "Pu-erh Tea",
    "Earl Grey",
    "English Breakfast",
    "Chai",
    "Chamomile",
    "Peppermint",
    "Jasmine",
    "Matcha",
    "Rooibos",
    "Hibiscus",
    "Ginger Tea",
    "Lemon Tea",
    "Darjeeling",
    "Assam",
    "Ceylon",
    "Sencha",
];

/// Beer styles.
static BEER_STYLES: &[&str] = &[
    "Lager",
    "Pilsner",
    "Pale Ale",
    "IPA",
    "Double IPA",
    "Stout",
    "Porter",
    "Wheat Beer",
    "Hefeweizen",
    "Belgian Ale",
    "Amber Ale",
    "Brown Ale",
    "Saison",
    "Sour Beer",
    "Gose",
    "Lambic",
    "Bock",
    "Dunkel",
    "Kolsch",
    "Red Ale",
    "Cream Ale",
    "Barleywine",
    "Tripel",
    "Dubbel",
    "Hazy IPA",
];

/// Wine varieties.
static WINE_VARIETIES: &[&str] = &[
    "Cabernet Sauvignon",
    "Merlot",
    "Pinot Noir",
    "Syrah",
    "Malbec",
    "Zinfandel",
    "Sangiovese",
    "Tempranillo",
    "Chardonnay",
    "Sauvignon Blanc",
    "Pinot Grigio",
    "Riesling",
    "Moscato",
    "Gewurztraminer",
    "Viognier",
    "Chenin Blanc",
    "Champagne",
    "Prosecco",
    "Cava",
    "Rose",
    "Port",
    "Sherry",
    "Madeira",
    "Vermouth",
    "Chianti",
];

/// Generate a random beverage.
pub fn beverage<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, BEVERAGES)
}

/// Generate a random coffee drink.
pub fn coffee_drink<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, COFFEE_DRINKS)
}

/// Generate a random tea type.
pub fn tea_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, TEA_TYPES)
}

/// Generate a random beer style.
pub fn beer_style<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, BEER_STYLES)
}

/// Generate a random wine variety.
pub fn wine_variety<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, WINE_VARIETIES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_beverage() {
        let mut rng = StdRng::seed_from_u64(42);
        let b = beverage(&mut rng);
        assert!(BEVERAGES.contains(&b));
    }

    #[test]
    fn test_coffee_drink() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = coffee_drink(&mut rng);
        assert!(COFFEE_DRINKS.contains(&c));
    }

    #[test]
    fn test_tea_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = tea_type(&mut rng);
        assert!(TEA_TYPES.contains(&t));
    }

    #[test]
    fn test_beer_style() {
        let mut rng = StdRng::seed_from_u64(42);
        let b = beer_style(&mut rng);
        assert!(BEER_STYLES.contains(&b));
    }

    #[test]
    fn test_wine_variety() {
        let mut rng = StdRng::seed_from_u64(42);
        let w = wine_variety(&mut rng);
        assert!(WINE_VARIETIES.contains(&w));
    }
}
