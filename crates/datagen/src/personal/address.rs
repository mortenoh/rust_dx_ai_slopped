//! Address generation.
//!
//! Generate street addresses, cities, zip codes, and full addresses.
//!
//! # Example
//!
//! ```
//! use dx_datagen::personal::address::{street_address, full_address};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let street = street_address(&mut rng);
//! let addr = full_address(&mut rng);
//!
//! println!("Street: {}", street);
//! println!("Full: {}", addr.format_us());
//! ```

use super::names::last_name;
use crate::text::words::noun;
use rand::Rng;

/// Street suffixes.
pub const STREET_SUFFIXES: &[&str] = &[
    "Street",
    "Avenue",
    "Boulevard",
    "Drive",
    "Lane",
    "Road",
    "Way",
    "Place",
    "Court",
    "Circle",
    "Trail",
    "Parkway",
    "Commons",
    "Terrace",
    "Heights",
    "Plaza",
    "Square",
    "Crossing",
];

/// Street prefixes/directions.
pub const STREET_DIRECTIONS: &[&str] = &[
    "North", "South", "East", "West", "N", "S", "E", "W", "NE", "NW", "SE", "SW",
];

/// US States (abbreviations).
pub const US_STATES: &[&str] = &[
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY",
];

/// US State full names.
pub const US_STATE_NAMES: &[&str] = &[
    "Alabama",
    "Alaska",
    "Arizona",
    "Arkansas",
    "California",
    "Colorado",
    "Connecticut",
    "Delaware",
    "Florida",
    "Georgia",
    "Hawaii",
    "Idaho",
    "Illinois",
    "Indiana",
    "Iowa",
    "Kansas",
    "Kentucky",
    "Louisiana",
    "Maine",
    "Maryland",
    "Massachusetts",
    "Michigan",
    "Minnesota",
    "Mississippi",
    "Missouri",
    "Montana",
    "Nebraska",
    "Nevada",
    "New Hampshire",
    "New Jersey",
    "New Mexico",
    "New York",
    "North Carolina",
    "North Dakota",
    "Ohio",
    "Oklahoma",
    "Oregon",
    "Pennsylvania",
    "Rhode Island",
    "South Carolina",
    "South Dakota",
    "Tennessee",
    "Texas",
    "Utah",
    "Vermont",
    "Virginia",
    "Washington",
    "West Virginia",
    "Wisconsin",
    "Wyoming",
];

/// Major US cities.
pub const US_CITIES: &[&str] = &[
    "New York",
    "Los Angeles",
    "Chicago",
    "Houston",
    "Phoenix",
    "Philadelphia",
    "San Antonio",
    "San Diego",
    "Dallas",
    "San Jose",
    "Austin",
    "Jacksonville",
    "Fort Worth",
    "Columbus",
    "Charlotte",
    "San Francisco",
    "Indianapolis",
    "Seattle",
    "Denver",
    "Washington",
    "Boston",
    "Nashville",
    "Detroit",
    "Portland",
    "Memphis",
    "Oklahoma City",
    "Las Vegas",
    "Louisville",
    "Baltimore",
    "Milwaukee",
    "Albuquerque",
    "Tucson",
    "Fresno",
    "Sacramento",
    "Kansas City",
    "Atlanta",
    "Miami",
    "Raleigh",
    "Omaha",
    "Minneapolis",
    "Cleveland",
    "Tampa",
];

/// A structured address.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    /// Street address line (e.g., "123 Main Street")
    pub street: String,
    /// City name
    pub city: String,
    /// State/province/region
    pub state: String,
    /// Postal/zip code
    pub zip: String,
    /// Country
    pub country: String,
}

impl Address {
    /// Format as a US-style address.
    pub fn format_us(&self) -> String {
        format!(
            "{}\n{}, {} {}",
            self.street, self.city, self.state, self.zip
        )
    }

    /// Format as a single line.
    pub fn format_line(&self) -> String {
        format!(
            "{}, {}, {} {}, {}",
            self.street, self.city, self.state, self.zip, self.country
        )
    }

    /// Format as European style (zip before city).
    pub fn format_eu(&self) -> String {
        format!(
            "{}\n{} {}\n{}",
            self.street, self.zip, self.city, self.country
        )
    }
}

/// Generate a street address (e.g., "123 Oak Street").
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let number = rng.random_range(1..9999);
    let street_name = street_name(rng);
    let suffix = STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())];

    // Sometimes add a direction
    if rng.random_bool(0.2) {
        let direction = STREET_DIRECTIONS[rng.random_range(0..STREET_DIRECTIONS.len())];
        format!("{} {} {} {}", number, direction, street_name, suffix)
    } else {
        format!("{} {} {}", number, street_name, suffix)
    }
}

/// Generate a street name (without number or suffix).
pub fn street_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Mix of name-based and noun-based street names
    if rng.random_bool(0.5) {
        last_name(rng).to_string()
    } else {
        let n = noun(rng);
        format!("{}{}", n[..1].to_uppercase(), &n[1..])
    }
}

/// Generate a US city name.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    US_CITIES[rng.random_range(0..US_CITIES.len())]
}

/// Generate a US state abbreviation.
pub fn state<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    US_STATES[rng.random_range(0..US_STATES.len())]
}

/// Generate a US state full name.
pub fn state_full<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    US_STATE_NAMES[rng.random_range(0..US_STATE_NAMES.len())]
}

/// Generate a US zip code (5 digits).
pub fn zip_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:05}", rng.random_range(10000..99999))
}

/// Generate a US zip+4 code.
pub fn zip_code_plus4<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{:05}-{:04}",
        rng.random_range(10000..99999),
        rng.random_range(1000..9999)
    )
}

/// Generate a full US address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> Address {
    Address {
        street: street_address(rng),
        city: city(rng).to_string(),
        state: state(rng).to_string(),
        zip: zip_code(rng),
        country: "USA".to_string(),
    }
}

/// Generate a secondary address (apt, suite, etc.).
pub fn secondary_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let types = ["Apt.", "Suite", "Unit", "Floor", "#"];
    let addr_type = types[rng.random_range(0..types.len())];
    let number = rng.random_range(1..500);
    format!("{} {}", addr_type, number)
}

/// Generate a full address with secondary line.
pub fn full_address_with_secondary<R: ?Sized + Rng>(rng: &mut R) -> Address {
    let mut addr = full_address(rng);
    addr.street = format!("{}, {}", addr.street, secondary_address(rng));
    addr
}

/// Generate a building number.
pub fn building_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{}", rng.random_range(1..9999))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_street_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = street_address(&mut rng);
        assert!(!addr.is_empty());
        // Should contain a number at the start
        assert!(addr.chars().next().unwrap().is_ascii_digit());
    }

    #[test]
    fn test_city() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = city(&mut rng);
        assert!(US_CITIES.contains(&c));
    }

    #[test]
    fn test_state() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = state(&mut rng);
        assert!(US_STATES.contains(&s));
    }

    #[test]
    fn test_zip_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let zip = zip_code(&mut rng);
        assert_eq!(zip.len(), 5);
        assert!(zip.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_zip_code_plus4() {
        let mut rng = StdRng::seed_from_u64(42);
        let zip = zip_code_plus4(&mut rng);
        assert_eq!(zip.len(), 10); // XXXXX-XXXX
        assert!(zip.contains('-'));
    }

    #[test]
    fn test_full_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = full_address(&mut rng);
        assert!(!addr.street.is_empty());
        assert!(!addr.city.is_empty());
        assert_eq!(addr.state.len(), 2);
        assert_eq!(addr.zip.len(), 5);
        assert_eq!(addr.country, "USA");
    }

    #[test]
    fn test_address_format() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = full_address(&mut rng);

        let us_format = addr.format_us();
        assert!(us_format.contains(&addr.city));
        assert!(us_format.contains(&addr.state));

        let line_format = addr.format_line();
        assert!(line_format.contains(&addr.country));
    }

    #[test]
    fn test_secondary_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let sec = secondary_address(&mut rng);
        assert!(!sec.is_empty());
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        let addr1 = full_address(&mut rng1);
        let addr2 = full_address(&mut rng2);

        assert_eq!(addr1, addr2);
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let addr = street_address(&mut *rng);
        assert!(!addr.is_empty());
    }
}
