//! Credit card number generation with Luhn validation.
//!
//! Generate valid credit card numbers for testing purposes.
//!
//! # Example
//!
//! ```
//! use dx_datagen::numeric::credit_card::{credit_card, credit_card_type, validate_luhn, CardType};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let visa = credit_card_type(&mut rng, CardType::Visa);
//! assert!(validate_luhn(&visa));
//! ```

use rand::Rng;

/// Credit card types with their IIN (Issuer Identification Number) prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Visa,
    Mastercard,
    Amex,
    Discover,
    DinersClub,
    Jcb,
}

impl CardType {
    /// Get the IIN prefix for this card type.
    fn prefix(&self) -> &'static [&'static str] {
        match self {
            CardType::Visa => &["4"],
            CardType::Mastercard => &["51", "52", "53", "54", "55"],
            CardType::Amex => &["34", "37"],
            CardType::Discover => &["6011", "65"],
            CardType::DinersClub => &["36", "38"],
            CardType::Jcb => &["35"],
        }
    }

    /// Get the card number length for this type.
    fn length(&self) -> usize {
        match self {
            CardType::Visa => 16,
            CardType::Mastercard => 16,
            CardType::Amex => 15,
            CardType::Discover => 16,
            CardType::DinersClub => 14,
            CardType::Jcb => 16,
        }
    }
}

/// All available card types.
pub const ALL_CARD_TYPES: &[CardType] = &[
    CardType::Visa,
    CardType::Mastercard,
    CardType::Amex,
    CardType::Discover,
    CardType::DinersClub,
    CardType::Jcb,
];

/// Validate a credit card number using the Luhn algorithm.
pub fn validate_luhn(number: &str) -> bool {
    let digits: Vec<u32> = number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.is_empty() {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum();

    sum % 10 == 0
}

/// Calculate the Luhn check digit for a partial card number.
fn luhn_check_digit(partial: &str) -> u8 {
    let digits: Vec<u32> = partial.chars().filter_map(|c| c.to_digit(10)).collect();

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 0 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum();

    ((10 - (sum % 10)) % 10) as u8
}

/// Generate a random credit card number (any type).
pub fn credit_card<R: ?Sized + Rng>(rng: &mut R) -> String {
    let card_type = ALL_CARD_TYPES[rng.random_range(0..ALL_CARD_TYPES.len())];
    credit_card_type(rng, card_type)
}

/// Generate a credit card number of a specific type.
pub fn credit_card_type<R: ?Sized + Rng>(rng: &mut R, card_type: CardType) -> String {
    let prefixes = card_type.prefix();
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let length = card_type.length();

    // Generate random digits for the middle part
    let remaining = length - prefix.len() - 1; // -1 for check digit
    let mut number = prefix.to_string();

    for _ in 0..remaining {
        number.push(char::from_digit(rng.random_range(0..10), 10).unwrap());
    }

    // Add Luhn check digit
    let check = luhn_check_digit(&number);
    number.push(char::from_digit(check as u32, 10).unwrap());

    number
}

/// Generate a Visa card number.
pub fn visa<R: ?Sized + Rng>(rng: &mut R) -> String {
    credit_card_type(rng, CardType::Visa)
}

/// Generate a Mastercard number.
pub fn mastercard<R: ?Sized + Rng>(rng: &mut R) -> String {
    credit_card_type(rng, CardType::Mastercard)
}

/// Generate an American Express card number.
pub fn amex<R: ?Sized + Rng>(rng: &mut R) -> String {
    credit_card_type(rng, CardType::Amex)
}

/// Format a card number with spaces (e.g., "4532 1234 5678 9012").
pub fn format_card_number(number: &str) -> String {
    let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() == 15 {
        // Amex format: XXXX XXXXXX XXXXX
        format!("{} {} {}", &digits[0..4], &digits[4..10], &digits[10..15])
    } else {
        // Standard format: XXXX XXXX XXXX XXXX
        digits
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Generate a CVV/CVC code.
pub fn cvv<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:03}", rng.random_range(0..1000))
}

/// Generate a 4-digit CVV (for Amex).
pub fn cvv4<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:04}", rng.random_range(0..10000))
}

/// Generate an expiry date (MM/YY format, future date).
pub fn expiry<R: ?Sized + Rng>(rng: &mut R) -> String {
    let month = rng.random_range(1..=12);
    let year = rng.random_range(25..35); // 2025-2034
    format!("{:02}/{:02}", month, year)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_validate_luhn_valid() {
        assert!(validate_luhn("4532015112830366")); // Valid Visa
        assert!(validate_luhn("5425233430109903")); // Valid Mastercard
        assert!(validate_luhn("374245455400126")); // Valid Amex
    }

    #[test]
    fn test_validate_luhn_invalid() {
        assert!(!validate_luhn("4532015112830367")); // Changed last digit
        assert!(!validate_luhn("1234567890123456"));
        assert!(!validate_luhn(""));
    }

    #[test]
    fn test_credit_card() {
        let mut rng = StdRng::seed_from_u64(42);
        let card = credit_card(&mut rng);
        assert!(validate_luhn(&card));
    }

    #[test]
    fn test_visa() {
        let mut rng = StdRng::seed_from_u64(42);
        let card = visa(&mut rng);
        assert!(card.starts_with('4'));
        assert_eq!(card.len(), 16);
        assert!(validate_luhn(&card));
    }

    #[test]
    fn test_mastercard() {
        let mut rng = StdRng::seed_from_u64(42);
        let card = mastercard(&mut rng);
        assert!(
            card.starts_with("51")
                || card.starts_with("52")
                || card.starts_with("53")
                || card.starts_with("54")
                || card.starts_with("55")
        );
        assert_eq!(card.len(), 16);
        assert!(validate_luhn(&card));
    }

    #[test]
    fn test_amex() {
        let mut rng = StdRng::seed_from_u64(42);
        let card = amex(&mut rng);
        assert!(card.starts_with("34") || card.starts_with("37"));
        assert_eq!(card.len(), 15);
        assert!(validate_luhn(&card));
    }

    #[test]
    fn test_format_card_number() {
        assert_eq!(
            format_card_number("4532015112830366"),
            "4532 0151 1283 0366"
        );
        assert_eq!(format_card_number("374245455400126"), "3742 454554 00126");
    }

    #[test]
    fn test_cvv() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = cvv(&mut rng);
        assert_eq!(code.len(), 3);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_expiry() {
        let mut rng = StdRng::seed_from_u64(42);
        let exp = expiry(&mut rng);
        assert_eq!(exp.len(), 5);
        assert!(exp.contains('/'));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(credit_card(&mut rng1), credit_card(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let card = credit_card(&mut *rng);
        assert!(validate_luhn(&card));
    }
}
