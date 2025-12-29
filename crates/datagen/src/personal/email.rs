//! Email address generation.
//!
//! Generate realistic email addresses with various formats.
//!
//! # Example
//!
//! ```
//! use dx_datagen::personal::email::{email, email_with_domain};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let addr = email(&mut rng);
//! assert!(addr.contains('@'));
//!
//! let work = email_with_domain(&mut rng, "company.com");
//! assert!(work.ends_with("@company.com"));
//! ```

use super::names::{first_name, last_name};
use crate::generators::alphanumeric;
use rand::Rng;

/// Common email domains.
pub const EMAIL_DOMAINS: &[&str] = &[
    "gmail.com",
    "yahoo.com",
    "outlook.com",
    "hotmail.com",
    "icloud.com",
    "mail.com",
    "protonmail.com",
    "aol.com",
    "zoho.com",
    "fastmail.com",
];

/// Company email domains (for work emails).
pub const COMPANY_DOMAINS: &[&str] = &[
    "acme.com",
    "example.com",
    "company.com",
    "corp.com",
    "business.com",
    "enterprise.com",
    "work.com",
    "office.com",
    "team.com",
    "org.com",
];

/// Email format styles.
#[derive(Debug, Clone, Copy, Default)]
pub enum EmailFormat {
    /// firstname.lastname@domain (default)
    #[default]
    FirstDotLast,
    /// firstnamelastname@domain
    FirstLast,
    /// f.lastname@domain
    InitialDotLast,
    /// firstname_lastname@domain
    FirstUnderscoreLast,
    /// Random alphanumeric@domain
    Random,
}

/// Generate a random email address.
pub fn email<R: ?Sized + Rng>(rng: &mut R) -> String {
    let domain = EMAIL_DOMAINS[rng.random_range(0..EMAIL_DOMAINS.len())];
    email_with_domain(rng, domain)
}

/// Generate an email with a specific domain.
pub fn email_with_domain<R: ?Sized + Rng>(rng: &mut R, domain: &str) -> String {
    let first = first_name(rng).to_lowercase();
    let last = last_name(rng).to_lowercase();

    let format = rng.random_range(0..5);
    let local = match format {
        0 => format!("{}.{}", first, last),
        1 => format!("{}{}", first, last),
        2 => format!("{}_{}", first, last),
        3 => format!("{}.{}{}", first, last, rng.random_range(1..100)),
        _ => format!("{}{}", &first[..1], last),
    };

    format!("{}@{}", local, domain)
}

/// Generate an email from specific first and last names.
pub fn email_from_name<R: ?Sized + Rng>(rng: &mut R, first: &str, last: &str) -> String {
    let domain = EMAIL_DOMAINS[rng.random_range(0..EMAIL_DOMAINS.len())];
    email_from_name_with_domain(rng, first, last, domain)
}

/// Generate an email from names with a specific domain.
pub fn email_from_name_with_domain<R: ?Sized + Rng>(
    rng: &mut R,
    first: &str,
    last: &str,
    domain: &str,
) -> String {
    let first = first.to_lowercase().replace(' ', "");
    let last = last.to_lowercase().replace(' ', "");

    let format = rng.random_range(0..4);
    let local = match format {
        0 => format!("{}.{}", first, last),
        1 => format!("{}{}", first, last),
        2 => format!("{}_{}", first, last),
        _ => format!("{}.{}{}", first, last, rng.random_range(1..100)),
    };

    format!("{}@{}", local, domain)
}

/// Generate a work/corporate email.
pub fn work_email<R: ?Sized + Rng>(rng: &mut R) -> String {
    let domain = COMPANY_DOMAINS[rng.random_range(0..COMPANY_DOMAINS.len())];
    email_with_domain(rng, domain)
}

/// Generate a random email with alphanumeric local part.
pub fn random_email<R: ?Sized + Rng>(rng: &mut R) -> String {
    let len = rng.random_range(6..12);
    let local = alphanumeric(rng, len);
    let domain = EMAIL_DOMAINS[rng.random_range(0..EMAIL_DOMAINS.len())];
    format!("{}@{}", local.to_lowercase(), domain)
}

/// Generate a free email (common providers like Gmail, Yahoo).
pub fn free_email<R: ?Sized + Rng>(rng: &mut R) -> String {
    email(rng)
}

/// Validate basic email format.
pub fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let local = parts[0];
    let domain = parts[1];

    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_email() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = email(&mut rng);
        assert!(addr.contains('@'));
        assert!(is_valid_email(&addr));
    }

    #[test]
    fn test_email_with_domain() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = email_with_domain(&mut rng, "example.org");
        assert!(addr.ends_with("@example.org"));
        assert!(is_valid_email(&addr));
    }

    #[test]
    fn test_email_from_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = email_from_name(&mut rng, "John", "Doe");
        assert!(addr.contains('@'));
        assert!(addr.contains("john") || addr.contains("doe"));
        assert!(is_valid_email(&addr));
    }

    #[test]
    fn test_work_email() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = work_email(&mut rng);
        assert!(addr.contains('@'));
        // Should use one of the company domains
        let domain = addr.split('@').nth(1).unwrap();
        assert!(COMPANY_DOMAINS.contains(&domain));
    }

    #[test]
    fn test_random_email() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = random_email(&mut rng);
        assert!(addr.contains('@'));
        assert!(is_valid_email(&addr));
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name@domain.org"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@domain.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user@.com"));
        assert!(!is_valid_email("user@domain."));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(email(&mut rng1), email(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let addr = email(&mut *rng);
        assert!(addr.contains('@'));
    }
}
