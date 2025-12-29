//! ISBN (International Standard Book Number) generation.
//!
//! Generate valid ISBN-10 and ISBN-13 numbers with proper check digits.
//!
//! # Example
//!
//! ```
//! use dx_datagen::numeric::isbn::{isbn10, isbn13, validate_isbn10, validate_isbn13};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let isbn = isbn13(&mut rng);
//! assert!(validate_isbn13(&isbn));
//! ```

use rand::Rng;

/// ISBN-13 prefixes (EAN prefixes for books).
pub const ISBN13_PREFIXES: &[&str] = &["978", "979"];

/// Common registration group elements (country/language codes).
pub const REGISTRATION_GROUPS: &[&str] = &[
    "0", "1",  // English-speaking countries
    "2",  // French-speaking countries
    "3",  // German-speaking countries
    "4",  // Japan
    "5",  // Russia
    "7",  // China
    "82", // Norway
    "87", // Denmark
    "91", // Sweden
];

/// Validate an ISBN-10 number.
pub fn validate_isbn10(isbn: &str) -> bool {
    let chars: Vec<char> = isbn.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

    if chars.len() != 10 {
        return false;
    }

    let sum: u32 = chars
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let value = if c == 'X' || c == 'x' {
                10
            } else {
                c.to_digit(10).unwrap_or(0)
            };
            value * (10 - i as u32)
        })
        .sum();

    sum.is_multiple_of(11)
}

/// Validate an ISBN-13 number.
pub fn validate_isbn13(isbn: &str) -> bool {
    let digits: Vec<u32> = isbn
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 13 {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| if i % 2 == 0 { d } else { d * 3 })
        .sum();

    sum.is_multiple_of(10)
}

/// Calculate the ISBN-10 check digit.
fn isbn10_check_digit(partial: &str) -> char {
    let digits: Vec<u32> = partial.chars().filter_map(|c| c.to_digit(10)).collect();

    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (10 - i as u32))
        .sum();

    let check = (11 - (sum % 11)) % 11;
    if check == 10 {
        'X'
    } else {
        char::from_digit(check, 10).unwrap()
    }
}

/// Calculate the ISBN-13 check digit.
fn isbn13_check_digit(partial: &str) -> char {
    let digits: Vec<u32> = partial.chars().filter_map(|c| c.to_digit(10)).collect();

    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| if i % 2 == 0 { d } else { d * 3 })
        .sum();

    let check = (10 - (sum % 10)) % 10;
    char::from_digit(check, 10).unwrap()
}

/// Generate a random ISBN-10.
pub fn isbn10<R: ?Sized + Rng>(rng: &mut R) -> String {
    let group = REGISTRATION_GROUPS[rng.random_range(0..REGISTRATION_GROUPS.len())];

    // Generate registrant and publication elements
    let remaining = 9 - group.len();
    let mut number = group.to_string();

    for _ in 0..remaining {
        number.push(char::from_digit(rng.random_range(0..10), 10).unwrap());
    }

    // Add check digit
    let check = isbn10_check_digit(&number);
    number.push(check);

    number
}

/// Generate a random ISBN-13.
pub fn isbn13<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = ISBN13_PREFIXES[rng.random_range(0..ISBN13_PREFIXES.len())];
    let group = REGISTRATION_GROUPS[rng.random_range(0..REGISTRATION_GROUPS.len())];

    // Generate registrant and publication elements
    let remaining = 12 - prefix.len() - group.len();
    let mut number = format!("{}{}", prefix, group);

    for _ in 0..remaining {
        number.push(char::from_digit(rng.random_range(0..10), 10).unwrap());
    }

    // Add check digit
    let check = isbn13_check_digit(&number);
    number.push(check);

    number
}

/// Format an ISBN-13 with hyphens (e.g., "978-0-306-40615-7").
pub fn format_isbn13(isbn: &str) -> String {
    let digits: String = isbn.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 13 {
        return isbn.to_string();
    }

    // Simple format: prefix-group-registrant-publication-check
    format!(
        "{}-{}-{}-{}-{}",
        &digits[0..3],
        &digits[3..4],
        &digits[4..7],
        &digits[7..12],
        &digits[12..13]
    )
}

/// Format an ISBN-10 with hyphens (e.g., "0-306-40615-2").
pub fn format_isbn10(isbn: &str) -> String {
    let chars: String = isbn.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if chars.len() != 10 {
        return isbn.to_string();
    }

    // Simple format: group-registrant-publication-check
    format!(
        "{}-{}-{}-{}",
        &chars[0..1],
        &chars[1..4],
        &chars[4..9],
        &chars[9..10]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_validate_isbn10_valid() {
        assert!(validate_isbn10("0306406152"));
        assert!(validate_isbn10("0-306-40615-2")); // With hyphens
        assert!(validate_isbn10("080442957X")); // With X check digit
    }

    #[test]
    fn test_validate_isbn10_invalid() {
        assert!(!validate_isbn10("0306406153")); // Wrong check digit
        assert!(!validate_isbn10("123456789")); // Too short
    }

    #[test]
    fn test_validate_isbn13_valid() {
        assert!(validate_isbn13("9780306406157"));
        assert!(validate_isbn13("978-0-306-40615-7")); // With hyphens
    }

    #[test]
    fn test_validate_isbn13_invalid() {
        assert!(!validate_isbn13("9780306406158")); // Wrong check digit
        assert!(!validate_isbn13("123456789012")); // Too short
    }

    #[test]
    fn test_isbn10() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let isbn = isbn10(&mut rng);
            assert_eq!(isbn.len(), 10, "ISBN-10 should be 10 chars: {}", isbn);
            assert!(validate_isbn10(&isbn), "Invalid ISBN-10: {}", isbn);
        }
    }

    #[test]
    fn test_isbn13() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let isbn = isbn13(&mut rng);
            assert_eq!(isbn.len(), 13, "ISBN-13 should be 13 chars: {}", isbn);
            assert!(validate_isbn13(&isbn), "Invalid ISBN-13: {}", isbn);
        }
    }

    #[test]
    fn test_format_isbn13() {
        assert_eq!(format_isbn13("9780306406157"), "978-0-306-40615-7");
    }

    #[test]
    fn test_format_isbn10() {
        assert_eq!(format_isbn10("0306406152"), "0-306-40615-2");
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(isbn13(&mut rng1), isbn13(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let isbn = isbn13(&mut *rng);
        assert!(validate_isbn13(&isbn));
    }
}
