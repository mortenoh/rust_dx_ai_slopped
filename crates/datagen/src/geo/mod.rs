//! Geographic data generation.
//!
//! Generate random geographic coordinates, points, and GeoJSON geometries.
//!
//! # Features
//!
//! - `latitude()` / `longitude()` - Basic coordinate generation
//! - `coordinates()` - Returns (lon, lat) tuple (GeoJSON order)
//! - `coordinates_in_bounds()` - Generate within a bounding box
//! - `geojson_point()` - Generate GeoJSON Point geometry (requires `geo` feature)
//!
//! # Example
//!
//! ```
//! use dx_datagen::geo::{latitude, longitude, coordinates};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let lat = latitude(&mut rng);
//! let lon = longitude(&mut rng);
//! let (lon, lat) = coordinates(&mut rng);
//! ```

pub mod point;

pub use point::{
    coordinates, coordinates_in_bounds, coordinates_in_bounds_string, coordinates_string, latitude,
    latitude_in_range, longitude, longitude_in_range,
};

#[cfg(feature = "geo")]
pub use point::{
    geojson_point, geojson_point_in_bounds, geojson_point_in_bounds_string, geojson_point_string,
};
