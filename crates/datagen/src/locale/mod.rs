//! Locale-specific data generation.
//!
//! Provides locale-aware data for names, addresses, phone numbers, and more.
//!
//! # Example
//!
//! ```
//! use dx_datagen::locale::{Locale, LocaleData};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let locale = Locale::EnUs;
//!
//! let first = locale.first_name(&mut rng);
//! let last = locale.last_name(&mut rng);
//! let phone = locale.phone(&mut rng);
//! ```

pub mod en_us;
pub mod no_no;

use rand::Rng;

/// Supported locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    /// US English (default)
    #[default]
    EnUs,
    /// Norwegian (Bokmål)
    NoNo,
}

impl Locale {
    /// Get locale from string code (e.g., "en_US", "no_NO").
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().replace('-', "_").as_str() {
            "en_us" | "en" | "us" => Some(Locale::EnUs),
            "no_no" | "no" | "nb" | "nb_no" => Some(Locale::NoNo),
            _ => None,
        }
    }

    /// Get the locale code.
    pub fn code(&self) -> &'static str {
        match self {
            Locale::EnUs => "en_US",
            Locale::NoNo => "no_NO",
        }
    }

    /// Get the language name.
    pub fn language(&self) -> &'static str {
        match self {
            Locale::EnUs => "English",
            Locale::NoNo => "Norwegian",
        }
    }

    /// Get the country name.
    pub fn country(&self) -> &'static str {
        match self {
            Locale::EnUs => "United States",
            Locale::NoNo => "Norway",
        }
    }
}

/// Trait for locale-specific data generation.
pub trait LocaleData {
    /// Get a random first name for this locale.
    fn first_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random male first name for this locale.
    fn first_name_male<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random female first name for this locale.
    fn first_name_female<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random last name for this locale.
    fn last_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Generate a full name for this locale.
    fn full_name<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        format!("{} {}", self.first_name(rng), self.last_name(rng))
    }

    /// Generate a phone number for this locale.
    fn phone<R: ?Sized + Rng>(&self, rng: &mut R) -> String;

    /// Get a random city for this locale.
    fn city<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random street suffix (Street, Avenue, etc.).
    fn street_suffix<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Generate a street address for this locale.
    fn street_address<R: ?Sized + Rng>(&self, rng: &mut R) -> String;

    /// Generate a postal/zip code for this locale.
    fn postal_code<R: ?Sized + Rng>(&self, rng: &mut R) -> String;
}

impl LocaleData for Locale {
    fn first_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::first_name(rng),
            Locale::NoNo => no_no::first_name(rng),
        }
    }

    fn first_name_male<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::first_name_male(rng),
            Locale::NoNo => no_no::first_name_male(rng),
        }
    }

    fn first_name_female<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::first_name_female(rng),
            Locale::NoNo => no_no::first_name_female(rng),
        }
    }

    fn last_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::last_name(rng),
            Locale::NoNo => no_no::last_name(rng),
        }
    }

    fn phone<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        match self {
            Locale::EnUs => en_us::phone(rng),
            Locale::NoNo => no_no::phone(rng),
        }
    }

    fn city<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::city(rng),
            Locale::NoNo => no_no::city(rng),
        }
    }

    fn street_suffix<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::street_suffix(rng),
            Locale::NoNo => no_no::street_suffix(rng),
        }
    }

    fn street_address<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        match self {
            Locale::EnUs => en_us::street_address(rng),
            Locale::NoNo => no_no::street_address(rng),
        }
    }

    fn postal_code<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        match self {
            Locale::EnUs => en_us::postal_code(rng),
            Locale::NoNo => no_no::postal_code(rng),
        }
    }
}

pub use en_us::EnUs;
pub use no_no::NoNo;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_locale_from_code() {
        assert_eq!(Locale::from_code("en_US"), Some(Locale::EnUs));
        assert_eq!(Locale::from_code("en-US"), Some(Locale::EnUs));
        assert_eq!(Locale::from_code("no_NO"), Some(Locale::NoNo));
        assert_eq!(Locale::from_code("nb"), Some(Locale::NoNo));
        assert_eq!(Locale::from_code("invalid"), None);
    }

    #[test]
    fn test_locale_code() {
        assert_eq!(Locale::EnUs.code(), "en_US");
        assert_eq!(Locale::NoNo.code(), "no_NO");
    }

    #[test]
    fn test_locale_data_en_us() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::EnUs;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_no_no() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::NoNo;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_full_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::EnUs;
        let name = locale.full_name(&mut rng);
        assert!(name.contains(' '));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let locale = Locale::EnUs;

        assert_eq!(locale.first_name(&mut rng1), locale.first_name(&mut rng2));
    }
}
