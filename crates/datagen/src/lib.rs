//! Data generation utilities for test data, random values, and predefined categories.
//!
//! This crate provides:
//! - **categories**: Predefined value sets (cities, countries, fruits, colors, etc.)
//! - **generators**: Core random data generators (int, float, string, boolean)
//! - **password**: Password and charset-based string generation
//! - **uuid**: UUID generation (v4, v7) with formatting options
//!
//! # Example
//!
//! ```
//! use dx_datagen::{generators, categories, password, uuid};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Generate random values
//! let num = generators::int_range(&mut rng, 1, 100);
//! let name = generators::alphanumeric(&mut rng, 8);
//!
//! // Pick from categories
//! let city = generators::pick_one(&mut rng, categories::CITIES);
//! let fruit = generators::pick_one(&mut rng, categories::FRUITS);
//!
//! // Generate passwords
//! let pwd = password::password(&mut rng, 16, true);
//!
//! // Generate UUIDs
//! let id = uuid::v4();
//! ```

pub mod categories;
pub mod generators;
pub mod password;
pub mod uuid;

// Re-export commonly used items at crate root
pub use generators::{
    alphanumeric, boolean, float_range, hex_bytes, hex_string, int_range, pick_one, shuffle,
};
pub use uuid::{v4, v7, Uuid, UuidFormat, UuidVersion};
