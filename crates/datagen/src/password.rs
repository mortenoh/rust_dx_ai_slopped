//! Password and charset-based string generation.

use rand::Rng;

/// Lowercase letters
pub const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

/// Uppercase letters
pub const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// All letters (upper and lower)
pub const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Digits 0-9
pub const DIGITS: &str = "0123456789";

/// Common password symbols
pub const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

/// Alphanumeric characters (letters + digits)
pub const ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Alphanumeric + symbols (full password charset)
pub const PASSWORD_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";

/// Generate a string from a custom charset.
pub fn with_charset<R: ?Sized + Rng>(rng: &mut R, len: usize, charset: &str) -> String {
    let chars: Vec<char> = charset.chars().collect();
    (0..len)
        .map(|_| {
            let idx = rng.random_range(0..chars.len());
            chars[idx]
        })
        .collect()
}

/// Generate a password with optional symbols.
pub fn password<R: ?Sized + Rng>(rng: &mut R, len: usize, include_symbols: bool) -> String {
    let charset = if include_symbols {
        PASSWORD_CHARS
    } else {
        ALPHANUMERIC
    };
    with_charset(rng, len, charset)
}

/// Generate a password with a specific set of character classes.
pub fn password_with_classes<R: ?Sized + Rng>(
    rng: &mut R,
    len: usize,
    use_upper: bool,
    use_lower: bool,
    use_digits: bool,
    use_symbols: bool,
) -> String {
    let mut charset = String::new();
    if use_lower {
        charset.push_str(ALPHA_LOWER);
    }
    if use_upper {
        charset.push_str(ALPHA_UPPER);
    }
    if use_digits {
        charset.push_str(DIGITS);
    }
    if use_symbols {
        charset.push_str(SYMBOLS);
    }
    if charset.is_empty() {
        charset.push_str(ALPHANUMERIC);
    }
    with_charset(rng, len, &charset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_with_charset() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = with_charset(&mut rng, 10, "abc");
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| "abc".contains(c)));
    }

    #[test]
    fn test_password_no_symbols() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = password(&mut rng, 16, false);
        assert_eq!(s.len(), 16);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_password_with_symbols() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = password(&mut rng, 32, true);
        assert_eq!(s.len(), 32);
        assert!(s.chars().all(|c| PASSWORD_CHARS.contains(c)));
    }

    #[test]
    fn test_password_with_classes() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = password_with_classes(&mut rng, 12, true, true, true, false);
        assert_eq!(s.len(), 12);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
