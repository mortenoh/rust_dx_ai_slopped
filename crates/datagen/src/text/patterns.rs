//! Pattern-based text generation.
//!
//! Generate strings from format patterns where special characters are replaced
//! with random values.
//!
//! # Pattern Characters
//!
//! - `#` - digit (0-9)
//! - `?` - lowercase letter (a-z)
//! - `A` - uppercase letter (A-Z)
//! - `*` - alphanumeric (a-z, A-Z, 0-9)
//! - `\\` - escape next character (use literal)
//!
//! # Example
//!
//! ```
//! use dx_datagen::text::from_pattern;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Phone number format
//! let phone = from_pattern(&mut rng, "(###) ###-####");
//! assert_eq!(phone.len(), 14);
//!
//! // License plate
//! let plate = from_pattern(&mut rng, "AA-#####");
//!
//! // Escape special characters
//! let escaped = from_pattern(&mut rng, "Price: \\#100");
//! assert!(escaped.contains("#100"));
//! ```

use rand::Rng;

/// Generate a string from a format pattern.
///
/// Pattern characters:
/// - `#` - random digit (0-9)
/// - `?` - random lowercase letter (a-z)
/// - `A` - random uppercase letter (A-Z)
/// - `*` - random alphanumeric character
/// - `\\` - escape the next character (use it literally)
///
/// All other characters are kept as-is.
pub fn from_pattern<R: ?Sized + Rng>(rng: &mut R, pattern: &str) -> String {
    let mut result = String::with_capacity(pattern.len());
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '#' => {
                result.push(char::from_digit(rng.random_range(0..10), 10).unwrap());
            }
            '?' => {
                result.push((b'a' + rng.random_range(0..26)) as char);
            }
            'A' => {
                result.push((b'A' + rng.random_range(0..26)) as char);
            }
            '*' => {
                const ALPHANUM: &[u8] =
                    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                result.push(ALPHANUM[rng.random_range(0..ALPHANUM.len())] as char);
            }
            '\\' => {
                // Escape: take the next character literally
                if let Some(next) = chars.next() {
                    result.push(next);
                }
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}

/// Generate a string matching a simple character class pattern.
///
/// This is a simpler alternative to `from_pattern` using repetition syntax:
/// - `d{n}` - n digits
/// - `a{n}` - n lowercase letters
/// - `A{n}` - n uppercase letters
/// - `w{n}` - n alphanumeric characters
///
/// # Example
///
/// ```
/// use dx_datagen::text::patterns::from_class_pattern;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
///
/// // 3 letters followed by 4 digits
/// let code = from_class_pattern(&mut rng, "A{3}d{4}");
/// ```
pub fn from_class_pattern<R: ?Sized + Rng>(rng: &mut R, pattern: &str) -> String {
    let mut result = String::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        if let Some(&'{') = chars.peek() {
            // Parse repetition count
            chars.next(); // consume '{'
            let mut count_str = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == '}' {
                    chars.next();
                    break;
                }
                count_str.push(chars.next().unwrap());
            }
            let count: usize = count_str.parse().unwrap_or(1);

            // Generate based on class
            for _ in 0..count {
                match c {
                    'd' => result.push(char::from_digit(rng.random_range(0..10), 10).unwrap()),
                    'a' => result.push((b'a' + rng.random_range(0..26)) as char),
                    'A' => result.push((b'A' + rng.random_range(0..26)) as char),
                    'w' => {
                        const ALPHANUM: &[u8] =
                            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                        result.push(ALPHANUM[rng.random_range(0..ALPHANUM.len())] as char);
                    }
                    _ => result.push(c),
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_digit_pattern() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = from_pattern(&mut rng, "###");
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_lowercase_pattern() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = from_pattern(&mut rng, "???");
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_uppercase_pattern() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = from_pattern(&mut rng, "AAA");
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_alphanumeric_pattern() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = from_pattern(&mut rng, "***");
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_mixed_pattern() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = from_pattern(&mut rng, "(###) ###-####");
        assert_eq!(result.len(), 14);
        assert_eq!(&result[0..1], "(");
        assert_eq!(&result[4..6], ") ");
        assert_eq!(&result[9..10], "-");
    }

    #[test]
    fn test_escape_pattern() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = from_pattern(&mut rng, "Price: \\#100");
        assert!(result.contains("#100"));
    }

    #[test]
    fn test_literal_chars() {
        let mut rng = StdRng::seed_from_u64(42);
        // Use escaped A to test literal characters (A is a special character)
        let result = from_pattern(&mut rng, "\\A\\BC-123");
        assert_eq!(result, "ABC-123");
    }

    #[test]
    fn test_class_pattern() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = from_class_pattern(&mut rng, "A{3}d{4}");
        assert_eq!(result.len(), 7);
        assert!(result[0..3].chars().all(|c| c.is_ascii_uppercase()));
        assert!(result[3..7].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        let result1 = from_pattern(&mut rng1, "###-###");
        let result2 = from_pattern(&mut rng2, "###-###");

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let result = from_pattern(&mut *rng, "###");
        assert_eq!(result.len(), 3);
    }
}
