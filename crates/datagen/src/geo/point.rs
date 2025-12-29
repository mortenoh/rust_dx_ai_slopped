//! Geographic point and coordinate generation.
//!
//! Generate random latitude/longitude coordinates and GeoJSON points.
//!
//! # Example
//!
//! ```
//! use dx_datagen::geo::point::{latitude, longitude, coordinates};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let lat = latitude(&mut rng);  // -90 to 90
//! let lon = longitude(&mut rng); // -180 to 180
//! let (lon, lat) = coordinates(&mut rng);
//! ```

use rand::Rng;

/// Generate a random latitude (-90 to 90 degrees).
pub fn latitude<R: ?Sized + Rng>(rng: &mut R) -> f64 {
    rng.random_range(-90.0..=90.0)
}

/// Generate a random longitude (-180 to 180 degrees).
pub fn longitude<R: ?Sized + Rng>(rng: &mut R) -> f64 {
    rng.random_range(-180.0..=180.0)
}

/// Generate random coordinates as (longitude, latitude) tuple.
///
/// Note: Returns (lon, lat) order to match GeoJSON convention.
pub fn coordinates<R: ?Sized + Rng>(rng: &mut R) -> (f64, f64) {
    (longitude(rng), latitude(rng))
}

/// Generate a latitude within a specific range.
pub fn latitude_in_range<R: ?Sized + Rng>(rng: &mut R, min: f64, max: f64) -> f64 {
    let min = min.max(-90.0);
    let max = max.min(90.0);
    rng.random_range(min..=max)
}

/// Generate a longitude within a specific range.
pub fn longitude_in_range<R: ?Sized + Rng>(rng: &mut R, min: f64, max: f64) -> f64 {
    let min = min.max(-180.0);
    let max = max.min(180.0);
    rng.random_range(min..=max)
}

/// Generate coordinates within a bounding box.
///
/// The bbox is specified as [min_lon, min_lat, max_lon, max_lat].
pub fn coordinates_in_bounds<R: ?Sized + Rng>(rng: &mut R, bbox: [f64; 4]) -> (f64, f64) {
    let [min_lon, min_lat, max_lon, max_lat] = bbox;
    (
        longitude_in_range(rng, min_lon, max_lon),
        latitude_in_range(rng, min_lat, max_lat),
    )
}

/// Generate a GeoJSON Point geometry object.
#[cfg(feature = "geo")]
pub fn geojson_point<R: ?Sized + Rng>(rng: &mut R) -> geojson::Geometry {
    let (lon, lat) = coordinates(rng);
    geojson::Geometry::new(geojson::Value::Point(vec![lon, lat]))
}

/// Generate a GeoJSON Point within a bounding box.
#[cfg(feature = "geo")]
pub fn geojson_point_in_bounds<R: ?Sized + Rng>(rng: &mut R, bbox: [f64; 4]) -> geojson::Geometry {
    let (lon, lat) = coordinates_in_bounds(rng, bbox);
    geojson::Geometry::new(geojson::Value::Point(vec![lon, lat]))
}

/// Generate a GeoJSON Point as a JSON string.
#[cfg(feature = "geo")]
pub fn geojson_point_string<R: ?Sized + Rng>(rng: &mut R) -> String {
    geojson_point(rng).to_string()
}

/// Generate a GeoJSON Point within bounds as a JSON string.
#[cfg(feature = "geo")]
pub fn geojson_point_in_bounds_string<R: ?Sized + Rng>(rng: &mut R, bbox: [f64; 4]) -> String {
    geojson_point_in_bounds(rng, bbox).to_string()
}

/// Format coordinates as a simple string "(lon, lat)".
pub fn coordinates_string<R: ?Sized + Rng>(rng: &mut R) -> String {
    let (lon, lat) = coordinates(rng);
    format!("({:.6}, {:.6})", lon, lat)
}

/// Format bounded coordinates as a simple string "(lon, lat)".
pub fn coordinates_in_bounds_string<R: ?Sized + Rng>(rng: &mut R, bbox: [f64; 4]) -> String {
    let (lon, lat) = coordinates_in_bounds(rng, bbox);
    format!("({:.6}, {:.6})", lon, lat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_latitude() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let lat = latitude(&mut rng);
            assert!((-90.0..=90.0).contains(&lat), "lat {} out of range", lat);
        }
    }

    #[test]
    fn test_longitude() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let lon = longitude(&mut rng);
            assert!((-180.0..=180.0).contains(&lon), "lon {} out of range", lon);
        }
    }

    #[test]
    fn test_coordinates() {
        let mut rng = StdRng::seed_from_u64(42);
        let (lon, lat) = coordinates(&mut rng);
        assert!((-180.0..=180.0).contains(&lon));
        assert!((-90.0..=90.0).contains(&lat));
    }

    #[test]
    fn test_latitude_in_range() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let lat = latitude_in_range(&mut rng, 50.0, 70.0);
            assert!((50.0..=70.0).contains(&lat), "lat {} out of range", lat);
        }
    }

    #[test]
    fn test_longitude_in_range() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let lon = longitude_in_range(&mut rng, 4.0, 31.0);
            assert!((4.0..=31.0).contains(&lon), "lon {} out of range", lon);
        }
    }

    #[test]
    fn test_coordinates_in_bounds() {
        let mut rng = StdRng::seed_from_u64(42);
        // Norway bounding box approximately
        let bbox = [4.0, 57.0, 31.0, 71.0];
        for _ in 0..100 {
            let (lon, lat) = coordinates_in_bounds(&mut rng, bbox);
            assert!((4.0..=31.0).contains(&lon), "lon {} out of bounds", lon);
            assert!((57.0..=71.0).contains(&lat), "lat {} out of bounds", lat);
        }
    }

    #[test]
    fn test_coordinates_string() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = coordinates_string(&mut rng);
        assert!(s.starts_with('('));
        assert!(s.ends_with(')'));
        assert!(s.contains(','));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        assert_eq!(latitude(&mut rng1), latitude(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let lat = latitude(&mut *rng);
        assert!((-90.0..=90.0).contains(&lat));
    }

    #[cfg(feature = "geo")]
    #[test]
    fn test_geojson_point() {
        let mut rng = StdRng::seed_from_u64(42);
        let geom = geojson_point(&mut rng);
        match geom.value {
            geojson::Value::Point(coords) => {
                assert_eq!(coords.len(), 2);
                assert!((-180.0..=180.0).contains(&coords[0]));
                assert!((-90.0..=90.0).contains(&coords[1]));
            }
            _ => panic!("Expected Point geometry"),
        }
    }

    #[cfg(feature = "geo")]
    #[test]
    fn test_geojson_point_string() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = geojson_point_string(&mut rng);
        assert!(s.contains("\"type\":\"Point\""));
        assert!(s.contains("\"coordinates\""));
    }
}
