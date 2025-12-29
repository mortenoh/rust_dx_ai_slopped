//! Vehicle data generation.
//!
//! This module provides generators for:
//! - Vehicle makes and models
//! - Vehicle types and fuel types
//! - VIN (Vehicle Identification Number)
//! - License plates
//!
//! # Example
//!
//! ```
//! use dx_datagen::vehicle;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let make = vehicle::vehicle_make(&mut rng);
//! let model = vehicle::vehicle_model(&mut rng);
//! let vin = vehicle::vin(&mut rng);
//! ```

use rand::Rng;

/// Vehicle makes
pub const VEHICLE_MAKES: &[&str] = &[
    "Toyota",
    "Honda",
    "Ford",
    "Chevrolet",
    "Nissan",
    "BMW",
    "Mercedes-Benz",
    "Audi",
    "Volkswagen",
    "Hyundai",
    "Kia",
    "Subaru",
    "Mazda",
    "Lexus",
    "Porsche",
    "Tesla",
    "Volvo",
    "Jaguar",
    "Land Rover",
    "Jeep",
    "Dodge",
    "Ram",
    "GMC",
    "Cadillac",
    "Buick",
    "Acura",
    "Infiniti",
    "Lincoln",
    "Genesis",
    "Alfa Romeo",
    "Fiat",
    "Mini",
    "Mitsubishi",
];

/// Vehicle models (generic)
pub const VEHICLE_MODELS: &[&str] = &[
    "Accord",
    "Camry",
    "Civic",
    "Corolla",
    "Mustang",
    "F-150",
    "Silverado",
    "Altima",
    "Sentra",
    "3 Series",
    "5 Series",
    "C-Class",
    "E-Class",
    "A4",
    "Q5",
    "Golf",
    "Jetta",
    "Elantra",
    "Sonata",
    "Optima",
    "Outback",
    "Impreza",
    "Mazda3",
    "CX-5",
    "Model 3",
    "Model Y",
    "Cherokee",
    "Wrangler",
    "Grand Cherokee",
    "Charger",
    "Challenger",
    "Tahoe",
    "Suburban",
    "Escalade",
    "Navigator",
    "Range Rover",
];

/// Vehicle types
pub const VEHICLE_TYPES: &[&str] = &[
    "Sedan",
    "SUV",
    "Truck",
    "Coupe",
    "Hatchback",
    "Convertible",
    "Wagon",
    "Van",
    "Minivan",
    "Crossover",
    "Sports Car",
    "Pickup",
    "Luxury",
    "Electric",
    "Hybrid",
    "Compact",
    "Midsize",
    "Full-size",
];

/// Fuel types
pub const FUEL_TYPES: &[&str] = &[
    "Gasoline",
    "Diesel",
    "Electric",
    "Hybrid",
    "Plug-in Hybrid",
    "Flex Fuel",
    "Natural Gas",
    "Hydrogen",
];

/// VIN characters (excludes I, O, Q per standard)
const VIN_CHARS: &[u8] = b"ABCDEFGHJKLMNPRSTUVWXYZ0123456789";

/// License plate formats by style
const PLATE_FORMATS: &[&str] = &[
    "AAA-0000", // Common US format
    "0AAA000",  // California
    "AAA 0000", // Some states with space
    "AA-00-AA", // European style
    "A000AAA",  // Mixed format
];

/// Get a random vehicle make.
///
/// # Example
/// ```
/// use dx_datagen::vehicle::vehicle_make;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let make = vehicle_make(&mut rng);
/// assert!(!make.is_empty());
/// ```
pub fn vehicle_make<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    VEHICLE_MAKES[rng.random_range(0..VEHICLE_MAKES.len())]
}

/// Get a random vehicle model.
///
/// # Example
/// ```
/// use dx_datagen::vehicle::vehicle_model;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let model = vehicle_model(&mut rng);
/// assert!(!model.is_empty());
/// ```
pub fn vehicle_model<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    VEHICLE_MODELS[rng.random_range(0..VEHICLE_MODELS.len())]
}

/// Get a random vehicle type.
pub fn vehicle_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    VEHICLE_TYPES[rng.random_range(0..VEHICLE_TYPES.len())]
}

/// Get a random fuel type.
pub fn fuel_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    FUEL_TYPES[rng.random_range(0..FUEL_TYPES.len())]
}

/// Generate a random vehicle year.
///
/// # Example
/// ```
/// use dx_datagen::vehicle::vehicle_year;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let year = vehicle_year(&mut rng);
/// assert!(year >= 1990 && year <= 2025);
/// ```
pub fn vehicle_year<R: ?Sized + Rng>(rng: &mut R) -> u16 {
    rng.random_range(1990..=2025)
}

/// Generate a random VIN (Vehicle Identification Number).
///
/// Generates a 17-character VIN following basic VIN format rules.
/// Note: This is a realistic but not valid VIN (check digit not calculated).
///
/// # Example
/// ```
/// use dx_datagen::vehicle::vin;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let v = vin(&mut rng);
/// assert_eq!(v.len(), 17);
/// ```
pub fn vin<R: ?Sized + Rng>(rng: &mut R) -> String {
    let mut result = String::with_capacity(17);

    // Position 9 is the check digit (0-9 or X)
    for i in 0..17 {
        if i == 8 {
            // Check digit position - use 0-9 or X
            let check_chars = b"0123456789X";
            let idx = rng.random_range(0..check_chars.len());
            result.push(check_chars[idx] as char);
        } else {
            let idx = rng.random_range(0..VIN_CHARS.len());
            result.push(VIN_CHARS[idx] as char);
        }
    }

    result
}

/// Generate a random license plate.
///
/// # Example
/// ```
/// use dx_datagen::vehicle::license_plate;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let plate = license_plate(&mut rng);
/// assert!(!plate.is_empty());
/// ```
pub fn license_plate<R: ?Sized + Rng>(rng: &mut R) -> String {
    let format = PLATE_FORMATS[rng.random_range(0..PLATE_FORMATS.len())];

    format
        .chars()
        .map(|c| match c {
            'A' => (b'A' + rng.random_range(0..26u8)) as char,
            '0' => (b'0' + rng.random_range(0..10u8)) as char,
            other => other,
        })
        .collect()
}

/// Generate a full vehicle description.
///
/// # Example
/// ```
/// use dx_datagen::vehicle::vehicle_full;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let desc = vehicle_full(&mut rng);
/// assert!(desc.contains(' '));
/// ```
pub fn vehicle_full<R: ?Sized + Rng>(rng: &mut R) -> String {
    let year = vehicle_year(rng);
    let make = vehicle_make(rng);
    let model = vehicle_model(rng);
    format!("{} {} {}", year, make, model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_vehicle_make() {
        let mut rng = StdRng::seed_from_u64(42);
        let make = vehicle_make(&mut rng);
        assert!(VEHICLE_MAKES.contains(&make));
    }

    #[test]
    fn test_vehicle_model() {
        let mut rng = StdRng::seed_from_u64(42);
        let model = vehicle_model(&mut rng);
        assert!(VEHICLE_MODELS.contains(&model));
    }

    #[test]
    fn test_vehicle_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let vtype = vehicle_type(&mut rng);
        assert!(VEHICLE_TYPES.contains(&vtype));
    }

    #[test]
    fn test_fuel_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let fuel = fuel_type(&mut rng);
        assert!(FUEL_TYPES.contains(&fuel));
    }

    #[test]
    fn test_vehicle_year() {
        let mut rng = StdRng::seed_from_u64(42);
        let year = vehicle_year(&mut rng);
        assert!(year >= 1990 && year <= 2025);
    }

    #[test]
    fn test_vin() {
        let mut rng = StdRng::seed_from_u64(42);
        let v = vin(&mut rng);
        assert_eq!(v.len(), 17);
        // VIN should not contain I, O, or Q
        assert!(!v.contains('I') && !v.contains('O') && !v.contains('Q'));
    }

    #[test]
    fn test_license_plate() {
        let mut rng = StdRng::seed_from_u64(42);
        let plate = license_plate(&mut rng);
        assert!(!plate.is_empty());
        assert!(plate.len() >= 6);
    }

    #[test]
    fn test_vehicle_full() {
        let mut rng = StdRng::seed_from_u64(42);
        let desc = vehicle_full(&mut rng);
        assert!(desc.contains(' '));
        // Should start with a year
        let year_str: String = desc.chars().take(4).collect();
        assert!(year_str.parse::<u16>().is_ok());
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(vehicle_make(&mut rng1), vehicle_make(&mut rng2));
        assert_eq!(vin(&mut rng1), vin(&mut rng2));
    }

    #[test]
    fn test_trait_object() {
        use rand::RngCore;
        let mut rng: Box<dyn RngCore> = Box::new(StdRng::seed_from_u64(42));
        let make = vehicle_make(&mut *rng);
        assert!(!make.is_empty());
    }
}
