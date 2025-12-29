//! Domain name generation.
//!
//! Generate random domain names and subdomains.
//!
//! # Example
//!
//! ```
//! use dx_datagen::network::domain::{domain, subdomain};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let d = domain(&mut rng);
//! println!("Domain: {}", d);  // e.g., "example.com"
//! ```

use crate::text::words::{adjective, noun};
use rand::Rng;

/// Common top-level domains.
pub const TLDS: &[&str] = &[
    "com", "org", "net", "edu", "gov", "io", "co", "dev", "app", "ai", "xyz", "info", "biz",
];

/// Country-code TLDs.
pub const COUNTRY_TLDS: &[&str] = &[
    "us", "uk", "de", "fr", "jp", "cn", "au", "ca", "no", "se", "dk", "nl", "es", "it", "br",
];

/// Common subdomain prefixes.
pub const SUBDOMAIN_PREFIXES: &[&str] = &[
    "www",
    "api",
    "app",
    "mail",
    "smtp",
    "pop",
    "imap",
    "ftp",
    "cdn",
    "static",
    "dev",
    "staging",
    "test",
    "beta",
    "admin",
    "portal",
    "dashboard",
    "docs",
    "blog",
    "shop",
    "store",
    "auth",
    "login",
    "m",
    "mobile",
];

/// Generate a random domain name.
pub fn domain<R: ?Sized + Rng>(rng: &mut R) -> String {
    let name = domain_name(rng);
    let tld = tld(rng);
    format!("{}.{}", name, tld)
}

/// Generate a domain name part (without TLD).
pub fn domain_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let format = rng.random_range(0..4);
    match format {
        0 => noun(rng).to_lowercase(),
        1 => format!("{}{}", adjective(rng), noun(rng)).to_lowercase(),
        2 => format!("{}-{}", adjective(rng), noun(rng)),
        _ => {
            let word = noun(rng);
            format!("{}{}", word, rng.random_range(1..100))
        }
    }
}

/// Pick a random TLD.
pub fn tld<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    TLDS[rng.random_range(0..TLDS.len())]
}

/// Pick a random country TLD.
pub fn country_tld<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    COUNTRY_TLDS[rng.random_range(0..COUNTRY_TLDS.len())]
}

/// Generate a domain with a specific TLD.
pub fn domain_with_tld<R: ?Sized + Rng>(rng: &mut R, tld: &str) -> String {
    format!("{}.{}", domain_name(rng), tld)
}

/// Generate a subdomain.
pub fn subdomain<R: ?Sized + Rng>(rng: &mut R) -> String {
    SUBDOMAIN_PREFIXES[rng.random_range(0..SUBDOMAIN_PREFIXES.len())].to_string()
}

/// Generate a full domain with subdomain.
pub fn full_domain<R: ?Sized + Rng>(rng: &mut R) -> String {
    let sub = subdomain(rng);
    let dom = domain(rng);
    format!("{}.{}", sub, dom)
}

/// Generate a random hostname (subdomain.domain.tld).
pub fn hostname<R: ?Sized + Rng>(rng: &mut R) -> String {
    full_domain(rng)
}

/// Generate a company-style domain (companyname.com).
pub fn company_domain<R: ?Sized + Rng>(rng: &mut R) -> String {
    let name = noun(rng).to_lowercase();
    format!("{}.com", name)
}

/// Generate a local hostname (not a FQDN).
pub fn local_hostname<R: ?Sized + Rng>(rng: &mut R) -> String {
    let format = rng.random_range(0..3);
    match format {
        0 => format!("{}-{}", adjective(rng), noun(rng)),
        1 => format!("{}_{}", noun(rng), rng.random_range(1..100)),
        _ => noun(rng).to_string(),
    }
}

/// Validate basic domain format.
pub fn is_valid_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }

    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() < 2 {
        return false;
    }

    for part in parts {
        if part.is_empty() || part.len() > 63 {
            return false;
        }
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_domain() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let d = domain(&mut rng);
            assert!(d.contains('.'));
            assert!(is_valid_domain(&d), "Invalid domain: {}", d);
        }
    }

    #[test]
    fn test_tld() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = tld(&mut rng);
        assert!(TLDS.contains(&t));
    }

    #[test]
    fn test_domain_with_tld() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = domain_with_tld(&mut rng, "io");
        assert!(d.ends_with(".io"));
    }

    #[test]
    fn test_subdomain() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = subdomain(&mut rng);
        assert!(SUBDOMAIN_PREFIXES.contains(&s.as_str()));
    }

    #[test]
    fn test_full_domain() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = full_domain(&mut rng);
        let parts: Vec<&str> = d.split('.').collect();
        assert!(parts.len() >= 3);
    }

    #[test]
    fn test_is_valid_domain() {
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("sub.example.com"));
        assert!(is_valid_domain("my-domain.org"));
        assert!(!is_valid_domain("example"));
        assert!(!is_valid_domain(".com"));
        assert!(!is_valid_domain("example."));
        assert!(!is_valid_domain("-example.com"));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(domain(&mut rng1), domain(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let d = domain(&mut *rng);
        assert!(d.contains('.'));
    }
}
