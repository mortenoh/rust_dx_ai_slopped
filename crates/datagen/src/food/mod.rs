//! Food and beverage data generators.
//!
//! Provides generators for food-related fake data including dishes,
//! ingredients, beverages, and restaurant information.

pub mod beverages;
pub mod dishes;
pub mod ingredients;
pub mod restaurants;

// Re-export common functions
pub use beverages::{beer_style, beverage, coffee_drink, tea_type, wine_variety};
pub use dishes::{cuisine, dessert, dish, meal_type};
pub use ingredients::{fruit, ingredient, meat, spice, vegetable};
pub use restaurants::{restaurant_name, restaurant_type};
