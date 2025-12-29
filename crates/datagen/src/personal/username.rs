//! Username generation.
//!
//! Generate realistic usernames in various formats.
//!
//! # Example
//!
//! ```
//! use dx_datagen::personal::username::{username, username_simple};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let user = username(&mut rng);
//! println!("Username: {}", user);
//! ```

use super::names::{first_name, last_name};
use crate::text::words::{adjective, noun};
use rand::Rng;

/// Generate a random username.
///
/// Returns usernames in various formats:
/// - first_last (e.g., "john_doe")
/// - first.last (e.g., "john.doe")
/// - adjective_noun (e.g., "happy_tiger")
/// - word + number (e.g., "player123")
pub fn username<R: ?Sized + Rng>(rng: &mut R) -> String {
    let format = rng.random_range(0..5);
    match format {
        0 => {
            // first_last
            format!(
                "{}_{}",
                first_name(rng).to_lowercase(),
                last_name(rng).to_lowercase()
            )
        }
        1 => {
            // first.last
            format!(
                "{}.{}",
                first_name(rng).to_lowercase(),
                last_name(rng).to_lowercase()
            )
        }
        2 => {
            // adjective_noun
            format!("{}_{}", adjective(rng), noun(rng))
        }
        3 => {
            // firstNNN
            format!(
                "{}{}",
                first_name(rng).to_lowercase(),
                rng.random_range(1..1000)
            )
        }
        _ => {
            // wordNNN
            format!("{}{}", noun(rng), rng.random_range(1..1000))
        }
    }
}

/// Generate a simple username (just first name + number).
pub fn username_simple<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{}{}",
        first_name(rng).to_lowercase(),
        rng.random_range(1..10000)
    )
}

/// Generate a username with specific length constraints.
pub fn username_with_length<R: ?Sized + Rng>(
    rng: &mut R,
    min_len: usize,
    max_len: usize,
) -> String {
    loop {
        let user = username(rng);
        if user.len() >= min_len && user.len() <= max_len {
            return user;
        }
        // If too long, try simpler format
        if user.len() > max_len {
            let short = format!(
                "{}{}",
                &noun(rng)[..3.min(noun(rng).len())],
                rng.random_range(1..100)
            );
            if short.len() >= min_len && short.len() <= max_len {
                return short;
            }
        }
    }
}

/// Generate a gaming-style username (e.g., "xXDarkLordXx", "ProGamer99").
pub fn gaming_username<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = &[
        "xx", "xX", "Pro", "Epic", "Dark", "Shadow", "Ninja", "Cyber", "Ultra",
    ];
    let suffixes = &[
        "xx", "Xx", "99", "Pro", "Master", "King", "Legend", "Gaming",
    ];

    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let suffix = suffixes[rng.random_range(0..suffixes.len())];
    let word = noun(rng);
    let capitalized = format!("{}{}", word[..1].to_uppercase(), &word[1..]);

    format!("{}{}{}", prefix, capitalized, suffix)
}

/// Generate a professional username (e.g., "jsmith", "johnd").
pub fn professional_username<R: ?Sized + Rng>(rng: &mut R) -> String {
    let first = first_name(rng).to_lowercase();
    let last = last_name(rng).to_lowercase();

    let format = rng.random_range(0..4);
    match format {
        0 => format!("{}{}", &first[..1], last), // jsmith
        1 => format!("{}{}", first, &last[..1]), // johns
        2 => format!("{}_{}", first, last),      // john_smith
        _ => format!("{}{}", first, last),       // johnsmith
    }
}

/// Generate a handle-style username (with @ prefix).
pub fn handle<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("@{}", username(rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_username() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let user = username(&mut rng);
            assert!(!user.is_empty());
            assert!(user
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.'));
        }
    }

    #[test]
    fn test_username_simple() {
        let mut rng = StdRng::seed_from_u64(42);
        let user = username_simple(&mut rng);
        assert!(!user.is_empty());
        // Should end with a number
        assert!(user.chars().last().unwrap().is_ascii_digit());
    }

    #[test]
    fn test_username_with_length() {
        let mut rng = StdRng::seed_from_u64(42);
        let user = username_with_length(&mut rng, 5, 15);
        assert!(user.len() >= 5);
        assert!(user.len() <= 15);
    }

    #[test]
    fn test_gaming_username() {
        let mut rng = StdRng::seed_from_u64(42);
        let user = gaming_username(&mut rng);
        assert!(!user.is_empty());
    }

    #[test]
    fn test_professional_username() {
        let mut rng = StdRng::seed_from_u64(42);
        let user = professional_username(&mut rng);
        assert!(!user.is_empty());
        assert!(user.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
    }

    #[test]
    fn test_handle() {
        let mut rng = StdRng::seed_from_u64(42);
        let h = handle(&mut rng);
        assert!(h.starts_with('@'));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(username(&mut rng1), username(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let user = username(&mut *rng);
        assert!(!user.is_empty());
    }
}
