//! Phone number generation.
//!
//! Generate phone numbers in various formats.
//!
//! # Example
//!
//! ```
//! use dx_datagen::personal::phone::{phone_us, phone_e164};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let us_phone = phone_us(&mut rng);
//! println!("US: {}", us_phone);  // e.g., "(555) 123-4567"
//!
//! let intl = phone_e164(&mut rng, "1");  // US country code
//! println!("E.164: {}", intl);  // e.g., "+15551234567"
//! ```

use crate::text::patterns::from_pattern;
use rand::Rng;

/// Generate a US phone number in (XXX) XXX-XXXX format.
pub fn phone_us<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Avoid area codes starting with 0 or 1, and special codes like 555
    let area_code = format!(
        "{}{}{}",
        rng.random_range(2..10),
        rng.random_range(0..10),
        rng.random_range(0..10)
    );
    from_pattern(rng, &format!("({}) ###-####", area_code))
}

/// Generate a phone number in E.164 international format.
///
/// E.164 format: +[country code][subscriber number]
/// Example: +14155552671
pub fn phone_e164<R: ?Sized + Rng>(rng: &mut R, country_code: &str) -> String {
    let subscriber = from_pattern(rng, "##########");
    format!("+{}{}", country_code, subscriber)
}

/// Generate a phone number with a custom format pattern.
///
/// Use `#` for digits in the pattern.
pub fn phone_with_format<R: ?Sized + Rng>(rng: &mut R, format: &str) -> String {
    from_pattern(rng, format)
}

/// Generate a generic phone number (random 10 digits).
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    from_pattern(rng, "###-###-####")
}

/// Generate a Norwegian phone number.
///
/// Norwegian mobile numbers start with 4 or 9.
pub fn phone_no<R: ?Sized + Rng>(rng: &mut R) -> String {
    let first_digit = if rng.random_bool(0.5) { '4' } else { '9' };
    from_pattern(rng, &format!("{} ## ## ###", first_digit))
}

/// Generate a Norwegian phone number in E.164 format.
pub fn phone_no_e164<R: ?Sized + Rng>(rng: &mut R) -> String {
    let first_digit = if rng.random_bool(0.5) { '4' } else { '9' };
    let subscriber = from_pattern(rng, "#######");
    format!("+47{}{}", first_digit, subscriber)
}

/// Generate a UK phone number.
pub fn phone_uk<R: ?Sized + Rng>(rng: &mut R) -> String {
    // UK mobile numbers typically start with 07
    from_pattern(rng, "07### ######")
}

/// Generate a German phone number.
pub fn phone_de<R: ?Sized + Rng>(rng: &mut R) -> String {
    // German mobile numbers start with 015, 016, 017
    let prefix = ["015", "016", "017"][rng.random_range(0..3)];
    from_pattern(rng, &format!("{} ########", prefix))
}

/// Generate a French phone number.
pub fn phone_fr<R: ?Sized + Rng>(rng: &mut R) -> String {
    // French mobile numbers start with 06 or 07
    let prefix = if rng.random_bool(0.5) { "06" } else { "07" };
    from_pattern(rng, &format!("{} ## ## ## ##", prefix))
}

/// Generate a mobile phone number (generic international).
pub fn mobile_phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    phone_us(rng) // Default to US format
}

/// Generate a landline phone number (generic).
pub fn landline<R: ?Sized + Rng>(rng: &mut R) -> String {
    from_pattern(rng, "(###) ###-####")
}

/// Country code to phone format mapping.
pub const COUNTRY_PHONE_FORMATS: &[(&str, &str)] = &[
    ("US", "(###) ###-####"),
    ("UK", "+44 #### ######"),
    ("DE", "+49 ### ########"),
    ("FR", "+33 # ## ## ## ##"),
    ("NO", "+47 ### ## ###"),
    ("SE", "+46 ## ### ## ##"),
    ("DK", "+45 ## ## ## ##"),
    ("JP", "+81 ## #### ####"),
    ("AU", "+61 # #### ####"),
    ("BR", "+55 ## #####-####"),
];

/// Generate a phone number for a specific country code.
pub fn phone_for_country<R: ?Sized + Rng>(rng: &mut R, country: &str) -> String {
    let format = COUNTRY_PHONE_FORMATS
        .iter()
        .find(|(c, _)| *c == country.to_uppercase())
        .map(|(_, f)| *f)
        .unwrap_or("###-###-####");

    from_pattern(rng, format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_phone_us() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone = phone_us(&mut rng);
        assert!(phone.starts_with('('));
        assert!(phone.contains(')'));
        assert!(phone.contains('-'));
        assert_eq!(phone.len(), 14); // (XXX) XXX-XXXX
    }

    #[test]
    fn test_phone_e164() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone = phone_e164(&mut rng, "1");
        assert!(phone.starts_with("+1"));
        assert_eq!(phone.len(), 12); // +1 + 10 digits
    }

    #[test]
    fn test_phone_no() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone = phone_no(&mut rng);
        assert!(phone.starts_with('4') || phone.starts_with('9'));
    }

    #[test]
    fn test_phone_no_e164() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone = phone_no_e164(&mut rng);
        assert!(phone.starts_with("+47"));
    }

    #[test]
    fn test_phone_with_format() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone = phone_with_format(&mut rng, "+1-###-###-####");
        assert!(phone.starts_with("+1-"));
        assert_eq!(phone.len(), 15);
    }

    #[test]
    fn test_phone_for_country() {
        let mut rng = StdRng::seed_from_u64(42);

        let us = phone_for_country(&mut rng, "US");
        assert!(us.starts_with('('));

        let no = phone_for_country(&mut rng, "NO");
        assert!(no.starts_with("+47"));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(phone_us(&mut rng1), phone_us(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let phone = phone_us(&mut *rng);
        assert!(!phone.is_empty());
    }
}
