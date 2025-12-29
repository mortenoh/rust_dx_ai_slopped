//! IP address generation.
//!
//! Generate IPv4 and IPv6 addresses in various formats.
//!
//! # Example
//!
//! ```
//! use dx_datagen::network::ip::{ipv4, ipv4_private, ipv6};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let ip = ipv4(&mut rng);
//! let private = ipv4_private(&mut rng);
//! let ip6 = ipv6(&mut rng);
//! ```

use rand::Rng;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Generate a random IPv4 address.
pub fn ipv4<R: ?Sized + Rng>(rng: &mut R) -> Ipv4Addr {
    Ipv4Addr::new(
        rng.random_range(1..255),
        rng.random_range(0..256) as u8,
        rng.random_range(0..256) as u8,
        rng.random_range(1..255),
    )
}

/// Generate a random private IPv4 address.
///
/// Uses one of:
/// - 10.0.0.0/8
/// - 172.16.0.0/12
/// - 192.168.0.0/16
pub fn ipv4_private<R: ?Sized + Rng>(rng: &mut R) -> Ipv4Addr {
    let range = rng.random_range(0..3);
    match range {
        0 => {
            // 10.0.0.0/8
            Ipv4Addr::new(
                10,
                rng.random_range(0..256) as u8,
                rng.random_range(0..256) as u8,
                rng.random_range(1..255),
            )
        }
        1 => {
            // 172.16.0.0/12
            Ipv4Addr::new(
                172,
                rng.random_range(16..32) as u8,
                rng.random_range(0..256) as u8,
                rng.random_range(1..255),
            )
        }
        _ => {
            // 192.168.0.0/16
            Ipv4Addr::new(
                192,
                168,
                rng.random_range(0..256) as u8,
                rng.random_range(1..255),
            )
        }
    }
}

/// Generate a random public (non-private, non-reserved) IPv4 address.
pub fn ipv4_public<R: ?Sized + Rng>(rng: &mut R) -> Ipv4Addr {
    loop {
        let ip = ipv4(rng);
        if !is_private_ipv4(&ip) && !is_reserved_ipv4(&ip) {
            return ip;
        }
    }
}

/// Generate a random localhost IPv4 address (127.x.x.x).
pub fn ipv4_localhost<R: ?Sized + Rng>(rng: &mut R) -> Ipv4Addr {
    Ipv4Addr::new(
        127,
        rng.random_range(0..256) as u8,
        rng.random_range(0..256) as u8,
        rng.random_range(1..255),
    )
}

/// Generate an IPv4 address within a subnet.
///
/// # Arguments
///
/// * `network` - The network address (e.g., "192.168.1.0")
/// * `prefix` - The subnet prefix length (e.g., 24 for /24)
pub fn ipv4_in_subnet<R: ?Sized + Rng>(rng: &mut R, network: Ipv4Addr, prefix: u8) -> Ipv4Addr {
    let mask = if prefix >= 32 {
        u32::MAX
    } else {
        u32::MAX << (32 - prefix)
    };
    let network_bits = u32::from(network) & mask;
    let host_bits = if prefix >= 32 {
        0
    } else {
        rng.random_range(1..(1u32 << (32 - prefix)) - 1)
    };
    Ipv4Addr::from(network_bits | host_bits)
}

/// Generate a random IPv6 address.
pub fn ipv6<R: ?Sized + Rng>(rng: &mut R) -> Ipv6Addr {
    Ipv6Addr::new(
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    )
}

/// Generate a random link-local IPv6 address (fe80::/10).
pub fn ipv6_link_local<R: ?Sized + Rng>(rng: &mut R) -> Ipv6Addr {
    Ipv6Addr::new(
        0xfe80,
        0,
        0,
        0,
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    )
}

/// Generate a random unique local IPv6 address (fc00::/7).
pub fn ipv6_unique_local<R: ?Sized + Rng>(rng: &mut R) -> Ipv6Addr {
    let first_segment = 0xfc00 | (rng.random::<u16>() & 0x01ff);
    Ipv6Addr::new(
        first_segment,
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    )
}

/// Generate an IPv4 address as a string.
pub fn ipv4_string<R: ?Sized + Rng>(rng: &mut R) -> String {
    ipv4(rng).to_string()
}

/// Generate an IPv6 address as a string.
pub fn ipv6_string<R: ?Sized + Rng>(rng: &mut R) -> String {
    ipv6(rng).to_string()
}

/// Check if an IPv4 address is private.
fn is_private_ipv4(ip: &Ipv4Addr) -> bool {
    ip.is_private()
}

/// Check if an IPv4 address is reserved.
fn is_reserved_ipv4(ip: &Ipv4Addr) -> bool {
    ip.is_loopback()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_documentation()
        || ip.is_unspecified()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_ipv4() {
        let mut rng = StdRng::seed_from_u64(42);
        let ip = ipv4(&mut rng);
        assert!(!ip.is_unspecified());
        assert!(!ip.is_broadcast());
    }

    #[test]
    fn test_ipv4_private() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let ip = ipv4_private(&mut rng);
            assert!(ip.is_private(), "Expected private, got: {}", ip);
        }
    }

    #[test]
    fn test_ipv4_public() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10 {
            let ip = ipv4_public(&mut rng);
            assert!(!ip.is_private(), "Expected public, got: {}", ip);
        }
    }

    #[test]
    fn test_ipv4_localhost() {
        let mut rng = StdRng::seed_from_u64(42);
        let ip = ipv4_localhost(&mut rng);
        assert!(ip.is_loopback());
    }

    #[test]
    fn test_ipv4_in_subnet() {
        let mut rng = StdRng::seed_from_u64(42);
        let network = Ipv4Addr::new(192, 168, 1, 0);
        for _ in 0..10 {
            let ip = ipv4_in_subnet(&mut rng, network, 24);
            assert_eq!(ip.octets()[0], 192);
            assert_eq!(ip.octets()[1], 168);
            assert_eq!(ip.octets()[2], 1);
        }
    }

    #[test]
    fn test_ipv6() {
        let mut rng = StdRng::seed_from_u64(42);
        let ip = ipv6(&mut rng);
        assert!(!ip.is_unspecified());
    }

    #[test]
    fn test_ipv6_link_local() {
        let mut rng = StdRng::seed_from_u64(42);
        let ip = ipv6_link_local(&mut rng);
        // Link-local starts with fe80
        assert_eq!(ip.segments()[0], 0xfe80);
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(ipv4(&mut rng1), ipv4(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let ip = ipv4(&mut *rng);
        assert!(!ip.is_unspecified());
    }
}
