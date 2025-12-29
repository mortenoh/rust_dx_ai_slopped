//! IBAN (International Bank Account Number) generation.
//!
//! Generate valid IBAN numbers with proper check digits.
//!
//! # Example
//!
//! ```
//! use dx_datagen::numeric::iban::{iban, iban_for_country, validate_iban};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let iban_no = iban_for_country(&mut rng, "NO");
//! assert!(validate_iban(&iban_no));
//! ```

use rand::Rng;

/// Country IBAN specifications: (country_code, total_length, bban_format).
pub const IBAN_SPECS: &[(&str, usize, &str)] = &[
    ("NO", 15, "4n7n"),     // Norway: 4 digits bank + 7 digits account
    ("DE", 22, "8n10n"),    // Germany: 8 digits bank + 10 digits account
    ("GB", 22, "4a14n"),    // UK: 4 letters bank + 14 digits
    ("FR", 27, "10n11n2a"), // France
    ("ES", 24, "20n"),      // Spain: 20 digits
    ("IT", 27, "1a10n12n"), // Italy
    ("NL", 18, "4a10n"),    // Netherlands: 4 letters + 10 digits
    ("SE", 24, "20n"),      // Sweden: 20 digits
    ("DK", 18, "14n"),      // Denmark: 14 digits
    ("FI", 18, "14n"),      // Finland: 14 digits
    ("CH", 21, "5n12n"),    // Switzerland: 5 digits + 12 alphanumeric
    ("AT", 20, "16n"),      // Austria: 16 digits
    ("BE", 16, "12n"),      // Belgium: 12 digits
    ("PL", 28, "24n"),      // Poland: 24 digits
];

/// Generate a random IBAN from a supported country.
pub fn iban<R: ?Sized + Rng>(rng: &mut R) -> String {
    let spec = IBAN_SPECS[rng.random_range(0..IBAN_SPECS.len())];
    iban_for_country(rng, spec.0)
}

/// Generate an IBAN for a specific country.
pub fn iban_for_country<R: ?Sized + Rng>(rng: &mut R, country_code: &str) -> String {
    let spec = IBAN_SPECS
        .iter()
        .find(|(code, _, _)| *code == country_code.to_uppercase())
        .unwrap_or(&("NO", 15, "4n7n")); // Default to Norway if not found

    let (code, total_length, _bban_format) = *spec;
    let bban_length = total_length - 4; // 2 country + 2 check digits

    // Generate random BBAN (simplified: all numeric for most cases)
    let mut bban = String::with_capacity(bban_length);
    for _ in 0..bban_length {
        bban.push(char::from_digit(rng.random_range(0..10), 10).unwrap());
    }

    // Calculate check digits
    let check_digits = calculate_iban_check_digits(code, &bban);

    format!("{}{:02}{}", code, check_digits, bban)
}

/// Calculate IBAN check digits (ISO 7064 Mod 97-10).
fn calculate_iban_check_digits(country_code: &str, bban: &str) -> u8 {
    // Rearrange: BBAN + country code + "00"
    let rearranged = format!("{}{}00", bban, country_code);

    // Convert letters to numbers (A=10, B=11, ..., Z=35)
    let numeric: String = rearranged
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let value = c.to_ascii_uppercase() as u32 - 'A' as u32 + 10;
                value.to_string()
            } else {
                c.to_string()
            }
        })
        .collect();

    // Calculate mod 97
    let remainder = mod97(&numeric);
    (98 - remainder) as u8
}

/// Calculate modulo 97 for a large number represented as a string.
fn mod97(number: &str) -> u32 {
    let mut remainder: u32 = 0;
    for c in number.chars() {
        if let Some(digit) = c.to_digit(10) {
            remainder = (remainder * 10 + digit) % 97;
        }
    }
    remainder
}

/// Validate an IBAN number.
pub fn validate_iban(iban: &str) -> bool {
    let cleaned: String = iban.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

    if cleaned.len() < 5 {
        return false;
    }

    // Rearrange: move first 4 characters to end
    let rearranged = format!("{}{}", &cleaned[4..], &cleaned[0..4]);

    // Convert letters to numbers
    let numeric: String = rearranged
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let value = c.to_ascii_uppercase() as u32 - 'A' as u32 + 10;
                value.to_string()
            } else {
                c.to_string()
            }
        })
        .collect();

    // Check if mod 97 equals 1
    mod97(&numeric) == 1
}

/// Generate a Norwegian IBAN.
pub fn iban_no<R: ?Sized + Rng>(rng: &mut R) -> String {
    iban_for_country(rng, "NO")
}

/// Generate a German IBAN.
pub fn iban_de<R: ?Sized + Rng>(rng: &mut R) -> String {
    iban_for_country(rng, "DE")
}

/// Generate a British IBAN.
pub fn iban_gb<R: ?Sized + Rng>(rng: &mut R) -> String {
    iban_for_country(rng, "GB")
}

/// Format an IBAN with spaces (groups of 4).
pub fn format_iban(iban: &str) -> String {
    let cleaned: String = iban.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

    cleaned
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Extract the country code from an IBAN.
pub fn iban_country(iban: &str) -> Option<String> {
    let cleaned: String = iban.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

    if cleaned.len() >= 2 {
        Some(cleaned[0..2].to_uppercase())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_validate_iban_valid() {
        // Known valid IBANs (test examples)
        assert!(validate_iban("GB82WEST12345698765432"));
        assert!(validate_iban("DE89370400440532013000"));
    }

    #[test]
    fn test_validate_iban_invalid() {
        assert!(!validate_iban("GB82WEST12345698765433")); // Wrong check
        assert!(!validate_iban("XX00")); // Too short
    }

    #[test]
    fn test_iban_no() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let iban = iban_no(&mut rng);
            assert!(iban.starts_with("NO"));
            assert_eq!(iban.len(), 15);
            assert!(validate_iban(&iban), "Invalid Norwegian IBAN: {}", iban);
        }
    }

    #[test]
    fn test_iban_de() {
        let mut rng = StdRng::seed_from_u64(42);
        let iban = iban_de(&mut rng);
        assert!(iban.starts_with("DE"));
        assert_eq!(iban.len(), 22);
        assert!(validate_iban(&iban), "Invalid German IBAN: {}", iban);
    }

    #[test]
    fn test_iban_random() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let iban = iban(&mut rng);
            assert!(validate_iban(&iban), "Invalid IBAN: {}", iban);
        }
    }

    #[test]
    fn test_format_iban() {
        assert_eq!(format_iban("NO9386011117947"), "NO93 8601 1117 947");
    }

    #[test]
    fn test_iban_country() {
        assert_eq!(iban_country("NO9386011117947"), Some("NO".to_string()));
        assert_eq!(
            iban_country("DE89370400440532013000"),
            Some("DE".to_string())
        );
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(iban(&mut rng1), iban(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let iban = iban(&mut *rng);
        assert!(validate_iban(&iban));
    }
}
