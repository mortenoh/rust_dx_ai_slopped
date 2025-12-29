//! Science data generation.
//!
//! This module provides generators for:
//! - Chemical elements and symbols
//! - Scientific units
//!
//! # Example
//!
//! ```
//! use dx_datagen::science;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let element = science::chemical_element(&mut rng);
//! let symbol = science::chemical_symbol(&mut rng);
//! ```

use rand::Rng;

/// Chemical elements (name, symbol, atomic number)
pub const ELEMENTS: &[(&str, &str, u8)] = &[
    ("Hydrogen", "H", 1),
    ("Helium", "He", 2),
    ("Lithium", "Li", 3),
    ("Beryllium", "Be", 4),
    ("Boron", "B", 5),
    ("Carbon", "C", 6),
    ("Nitrogen", "N", 7),
    ("Oxygen", "O", 8),
    ("Fluorine", "F", 9),
    ("Neon", "Ne", 10),
    ("Sodium", "Na", 11),
    ("Magnesium", "Mg", 12),
    ("Aluminum", "Al", 13),
    ("Silicon", "Si", 14),
    ("Phosphorus", "P", 15),
    ("Sulfur", "S", 16),
    ("Chlorine", "Cl", 17),
    ("Argon", "Ar", 18),
    ("Potassium", "K", 19),
    ("Calcium", "Ca", 20),
    ("Scandium", "Sc", 21),
    ("Titanium", "Ti", 22),
    ("Vanadium", "V", 23),
    ("Chromium", "Cr", 24),
    ("Manganese", "Mn", 25),
    ("Iron", "Fe", 26),
    ("Cobalt", "Co", 27),
    ("Nickel", "Ni", 28),
    ("Copper", "Cu", 29),
    ("Zinc", "Zn", 30),
    ("Gallium", "Ga", 31),
    ("Germanium", "Ge", 32),
    ("Arsenic", "As", 33),
    ("Selenium", "Se", 34),
    ("Bromine", "Br", 35),
    ("Krypton", "Kr", 36),
    ("Rubidium", "Rb", 37),
    ("Strontium", "Sr", 38),
    ("Yttrium", "Y", 39),
    ("Zirconium", "Zr", 40),
    ("Niobium", "Nb", 41),
    ("Molybdenum", "Mo", 42),
    ("Technetium", "Tc", 43),
    ("Ruthenium", "Ru", 44),
    ("Rhodium", "Rh", 45),
    ("Palladium", "Pd", 46),
    ("Silver", "Ag", 47),
    ("Cadmium", "Cd", 48),
    ("Indium", "In", 49),
    ("Tin", "Sn", 50),
    ("Gold", "Au", 79),
    ("Lead", "Pb", 82),
    ("Uranium", "U", 92),
];

/// SI base units
pub const SI_UNITS: &[(&str, &str)] = &[
    ("meter", "m"),
    ("kilogram", "kg"),
    ("second", "s"),
    ("ampere", "A"),
    ("kelvin", "K"),
    ("mole", "mol"),
    ("candela", "cd"),
];

/// Common derived units
pub const DERIVED_UNITS: &[(&str, &str)] = &[
    ("hertz", "Hz"),
    ("newton", "N"),
    ("pascal", "Pa"),
    ("joule", "J"),
    ("watt", "W"),
    ("coulomb", "C"),
    ("volt", "V"),
    ("farad", "F"),
    ("ohm", "Ω"),
    ("siemens", "S"),
    ("weber", "Wb"),
    ("tesla", "T"),
    ("henry", "H"),
    ("lumen", "lm"),
    ("lux", "lx"),
    ("becquerel", "Bq"),
    ("gray", "Gy"),
    ("sievert", "Sv"),
];

/// Get a random chemical element name.
///
/// # Example
/// ```
/// use dx_datagen::science::chemical_element;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let element = chemical_element(&mut rng);
/// assert!(!element.is_empty());
/// ```
pub fn chemical_element<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..ELEMENTS.len());
    ELEMENTS[idx].0
}

/// Get a random chemical symbol.
///
/// # Example
/// ```
/// use dx_datagen::science::chemical_symbol;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let symbol = chemical_symbol(&mut rng);
/// assert!(!symbol.is_empty());
/// ```
pub fn chemical_symbol<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..ELEMENTS.len());
    ELEMENTS[idx].1
}

/// Get a random element with its full info (name, symbol, atomic number).
pub fn element_full<R: ?Sized + Rng>(rng: &mut R) -> (&'static str, &'static str, u8) {
    let idx = rng.random_range(0..ELEMENTS.len());
    ELEMENTS[idx]
}

/// Get a random SI unit name.
///
/// # Example
/// ```
/// use dx_datagen::science::unit;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let u = unit(&mut rng);
/// assert!(!u.is_empty());
/// ```
pub fn unit<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..SI_UNITS.len());
    SI_UNITS[idx].0
}

/// Get a random unit symbol.
pub fn unit_symbol<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..SI_UNITS.len());
    SI_UNITS[idx].1
}

/// Get a random derived unit name.
pub fn derived_unit<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..DERIVED_UNITS.len());
    DERIVED_UNITS[idx].0
}

/// Get a random derived unit symbol.
pub fn derived_unit_symbol<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..DERIVED_UNITS.len());
    DERIVED_UNITS[idx].1
}

/// Generate a value in scientific notation.
///
/// # Example
/// ```
/// use dx_datagen::science::scientific_notation;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let notation = scientific_notation(&mut rng);
/// assert!(notation.contains('e') || notation.contains('E'));
/// ```
pub fn scientific_notation<R: ?Sized + Rng>(rng: &mut R) -> String {
    let mantissa: f64 = rng.random_range(1.0..10.0);
    let exponent: i32 = rng.random_range(-10..=10);
    format!("{:.3}e{:+}", mantissa, exponent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_chemical_element() {
        let mut rng = StdRng::seed_from_u64(42);
        let element = chemical_element(&mut rng);
        assert!(ELEMENTS.iter().any(|(name, _, _)| *name == element));
    }

    #[test]
    fn test_chemical_symbol() {
        let mut rng = StdRng::seed_from_u64(42);
        let symbol = chemical_symbol(&mut rng);
        assert!(ELEMENTS.iter().any(|(_, sym, _)| *sym == symbol));
    }

    #[test]
    fn test_element_full() {
        let mut rng = StdRng::seed_from_u64(42);
        let (name, symbol, num) = element_full(&mut rng);
        assert!(!name.is_empty());
        assert!(!symbol.is_empty());
        assert!(num > 0);
    }

    #[test]
    fn test_unit() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = unit(&mut rng);
        assert!(SI_UNITS.iter().any(|(name, _)| *name == u));
    }

    #[test]
    fn test_derived_unit() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = derived_unit(&mut rng);
        assert!(DERIVED_UNITS.iter().any(|(name, _)| *name == u));
    }

    #[test]
    fn test_scientific_notation() {
        let mut rng = StdRng::seed_from_u64(42);
        let notation = scientific_notation(&mut rng);
        assert!(notation.contains('e'));
        // Should have format X.XXXe±Y
        let parts: Vec<&str> = notation.split('e').collect();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(chemical_element(&mut rng1), chemical_element(&mut rng2));
        assert_eq!(
            scientific_notation(&mut rng1),
            scientific_notation(&mut rng2)
        );
    }

    #[test]
    fn test_trait_object() {
        use rand::RngCore;
        let mut rng: Box<dyn RngCore> = Box::new(StdRng::seed_from_u64(42));
        let element = chemical_element(&mut *rng);
        assert!(!element.is_empty());
    }
}
