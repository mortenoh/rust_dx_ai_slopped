//! Social Security Number (SSN) and national ID generation.
//!
//! Generate SSN-style identifiers for various countries.
//!
//! # Example
//!
//! ```
//! use dx_datagen::numeric::ssn::{ssn_us, ssn_no};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let us_ssn = ssn_us(&mut rng);
//! let no_ssn = ssn_no(&mut rng);
//! ```

use rand::Rng;

/// Generate a US Social Security Number (XXX-XX-XXXX format).
///
/// Note: Generates numbers in valid format but not necessarily valid
/// according to SSA rules (avoids certain reserved ranges).
pub fn ssn_us<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Area number: 001-899 (excluding 666)
    let mut area = rng.random_range(1..900);
    if area == 666 {
        area = 667;
    }

    // Group number: 01-99
    let group = rng.random_range(1..100);

    // Serial number: 0001-9999
    let serial = rng.random_range(1..10000);

    format!("{:03}-{:02}-{:04}", area, group, serial)
}

/// Generate a US SSN without hyphens.
pub fn ssn_us_plain<R: ?Sized + Rng>(rng: &mut R) -> String {
    ssn_us(rng).replace('-', "")
}

/// Generate a Norwegian national identity number (fødselsnummer).
///
/// Format: DDMMYYXXXCC where:
/// - DDMMYY: Date of birth
/// - XXX: Individual number (odd for males, even for females)
/// - CC: Check digits
pub fn ssn_no<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Generate random birth date components
    let day = rng.random_range(1..29); // Safe for all months
    let month = rng.random_range(1..13);
    let year = rng.random_range(50..100); // 1950-1999

    // Individual number (000-499 for 1900-1999)
    let individual = rng.random_range(0..500);

    // Build the base number
    let d1 = day / 10;
    let d2 = day % 10;
    let m1 = month / 10;
    let m2 = month % 10;
    let y1 = year / 10;
    let y2 = year % 10;
    let i1 = individual / 100;
    let i2 = (individual / 10) % 10;
    let i3 = individual % 10;

    // Calculate check digits using modulus 11
    let k1 = calculate_no_check1(d1, d2, m1, m2, y1, y2, i1, i2, i3);
    let k2 = calculate_no_check2(d1, d2, m1, m2, y1, y2, i1, i2, i3, k1);

    // If check digits are invalid (10), regenerate
    if k1 == 10 || k2 == 10 {
        return ssn_no(rng);
    }

    format!(
        "{:02}{:02}{:02}{:03}{:01}{:01}",
        day, month, year, individual, k1, k2
    )
}

/// Calculate the first check digit for Norwegian SSN.
fn calculate_no_check1(
    d1: u32,
    d2: u32,
    m1: u32,
    m2: u32,
    y1: u32,
    y2: u32,
    i1: u32,
    i2: u32,
    i3: u32,
) -> u32 {
    let sum = 3 * d1 + 7 * d2 + 6 * m1 + 1 * m2 + 8 * y1 + 9 * y2 + 4 * i1 + 5 * i2 + 2 * i3;
    let remainder = sum % 11;
    if remainder == 0 {
        0
    } else {
        11 - remainder
    }
}

/// Calculate the second check digit for Norwegian SSN.
fn calculate_no_check2(
    d1: u32,
    d2: u32,
    m1: u32,
    m2: u32,
    y1: u32,
    y2: u32,
    i1: u32,
    i2: u32,
    i3: u32,
    k1: u32,
) -> u32 {
    let sum =
        5 * d1 + 4 * d2 + 3 * m1 + 2 * m2 + 7 * y1 + 6 * y2 + 5 * i1 + 4 * i2 + 3 * i3 + 2 * k1;
    let remainder = sum % 11;
    if remainder == 0 {
        0
    } else {
        11 - remainder
    }
}

/// Validate a Norwegian SSN format and check digits.
pub fn validate_ssn_no(ssn: &str) -> bool {
    let digits: Vec<u32> = ssn
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 11 {
        return false;
    }

    let d1 = digits[0];
    let d2 = digits[1];
    let m1 = digits[2];
    let m2 = digits[3];
    let y1 = digits[4];
    let y2 = digits[5];
    let i1 = digits[6];
    let i2 = digits[7];
    let i3 = digits[8];
    let k1 = digits[9];
    let k2 = digits[10];

    let calc_k1 = calculate_no_check1(d1, d2, m1, m2, y1, y2, i1, i2, i3);
    let calc_k2 = calculate_no_check2(d1, d2, m1, m2, y1, y2, i1, i2, i3, calc_k1);

    k1 == calc_k1 && k2 == calc_k2
}

/// Generate a Norwegian SSN with spaces (DD MM YY XXX CC).
pub fn ssn_no_formatted<R: ?Sized + Rng>(rng: &mut R) -> String {
    let ssn = ssn_no(rng);
    format!(
        "{} {} {} {} {}",
        &ssn[0..2],
        &ssn[2..4],
        &ssn[4..6],
        &ssn[6..9],
        &ssn[9..11]
    )
}

/// Format a US SSN with hyphens.
pub fn format_ssn_us(ssn: &str) -> String {
    let digits: String = ssn.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 9 {
        return ssn.to_string();
    }
    format!("{}-{}-{}", &digits[0..3], &digits[3..5], &digits[5..9])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_ssn_us() {
        let mut rng = StdRng::seed_from_u64(42);
        let ssn = ssn_us(&mut rng);
        assert_eq!(ssn.len(), 11); // XXX-XX-XXXX
        assert!(ssn.contains('-'));

        // Check format
        let parts: Vec<&str> = ssn.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].len(), 3);
        assert_eq!(parts[1].len(), 2);
        assert_eq!(parts[2].len(), 4);
    }

    #[test]
    fn test_ssn_us_plain() {
        let mut rng = StdRng::seed_from_u64(42);
        let ssn = ssn_us_plain(&mut rng);
        assert_eq!(ssn.len(), 9);
        assert!(!ssn.contains('-'));
    }

    #[test]
    fn test_ssn_no() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let ssn = ssn_no(&mut rng);
            assert_eq!(ssn.len(), 11);
            assert!(validate_ssn_no(&ssn), "Invalid Norwegian SSN: {}", ssn);
        }
    }

    #[test]
    fn test_validate_ssn_no() {
        // Invalid SSNs should return false
        assert!(!validate_ssn_no("invalid"));
        assert!(!validate_ssn_no("12345")); // Too short
    }

    #[test]
    fn test_ssn_no_formatted() {
        let mut rng = StdRng::seed_from_u64(42);
        let ssn = ssn_no_formatted(&mut rng);
        assert_eq!(ssn.len(), 15); // DD MM YY XXX CC with spaces
        assert!(ssn.contains(' '));
    }

    #[test]
    fn test_format_ssn_us() {
        assert_eq!(format_ssn_us("123456789"), "123-45-6789");
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(ssn_us(&mut rng1), ssn_us(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let ssn = ssn_us(&mut *rng);
        assert!(!ssn.is_empty());
    }
}
