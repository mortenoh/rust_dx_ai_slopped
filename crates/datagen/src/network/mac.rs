//! MAC address generation.
//!
//! Generate random MAC (Media Access Control) addresses.
//!
//! # Example
//!
//! ```
//! use dx_datagen::network::mac::{mac_address, mac_address_colon};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let mac = mac_address(&mut rng);
//! println!("MAC: {}", mac);  // e.g., "00:1A:2B:3C:4D:5E"
//! ```

use rand::Rng;

/// Generate a random MAC address with colons (XX:XX:XX:XX:XX:XX).
pub fn mac_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    mac_address_colon(rng)
}

/// Generate a MAC address with colon separators.
pub fn mac_address_colon<R: ?Sized + Rng>(rng: &mut R) -> String {
    let bytes: [u8; 6] = [
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ];
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Generate a MAC address with dash separators.
pub fn mac_address_dash<R: ?Sized + Rng>(rng: &mut R) -> String {
    let bytes: [u8; 6] = [
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ];
    format!(
        "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Generate a MAC address without separators.
pub fn mac_address_plain<R: ?Sized + Rng>(rng: &mut R) -> String {
    let bytes: [u8; 6] = [
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ];
    format!(
        "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Generate a MAC address in Cisco format (XXXX.XXXX.XXXX).
pub fn mac_address_cisco<R: ?Sized + Rng>(rng: &mut R) -> String {
    let bytes: [u8; 6] = [
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ];
    format!(
        "{:02X}{:02X}.{:02X}{:02X}.{:02X}{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Generate a locally administered MAC address.
///
/// The second least significant bit of the first octet is set to 1.
pub fn mac_address_local<R: ?Sized + Rng>(rng: &mut R) -> String {
    let first_byte = (rng.random::<u8>() | 0x02) & 0xFE; // Set local bit, clear multicast bit
    let bytes: [u8; 6] = [
        first_byte,
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ];
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Generate a universally administered MAC address.
///
/// The second least significant bit of the first octet is cleared.
pub fn mac_address_universal<R: ?Sized + Rng>(rng: &mut R) -> String {
    let first_byte = rng.random::<u8>() & 0xFC; // Clear local and multicast bits
    let bytes: [u8; 6] = [
        first_byte,
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ];
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Generate a MAC address with a specific OUI (Organizationally Unique Identifier).
///
/// The OUI is the first 3 bytes of the MAC address.
pub fn mac_address_with_oui<R: ?Sized + Rng>(rng: &mut R, oui: [u8; 3]) -> String {
    let bytes: [u8; 6] = [
        oui[0],
        oui[1],
        oui[2],
        rng.random(),
        rng.random(),
        rng.random(),
    ];
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Common vendor OUIs.
pub const VENDOR_OUIS: &[([u8; 3], &str)] = &[
    ([0x00, 0x1A, 0x2B], "Apple"),
    ([0x00, 0x50, 0x56], "VMware"),
    ([0x00, 0x0C, 0x29], "VMware"),
    ([0x00, 0x15, 0x5D], "Microsoft"),
    ([0x52, 0x54, 0x00], "QEMU/KVM"),
    ([0x08, 0x00, 0x27], "VirtualBox"),
    ([0x00, 0x16, 0x3E], "Xen"),
];

/// Generate a MAC address with a random known vendor OUI.
pub fn mac_address_vendor<R: ?Sized + Rng>(rng: &mut R) -> String {
    let (oui, _vendor) = VENDOR_OUIS[rng.random_range(0..VENDOR_OUIS.len())];
    mac_address_with_oui(rng, oui)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_mac_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let mac = mac_address(&mut rng);
        assert_eq!(mac.len(), 17); // XX:XX:XX:XX:XX:XX
        assert_eq!(mac.matches(':').count(), 5);
    }

    #[test]
    fn test_mac_address_dash() {
        let mut rng = StdRng::seed_from_u64(42);
        let mac = mac_address_dash(&mut rng);
        assert_eq!(mac.len(), 17);
        assert_eq!(mac.matches('-').count(), 5);
    }

    #[test]
    fn test_mac_address_plain() {
        let mut rng = StdRng::seed_from_u64(42);
        let mac = mac_address_plain(&mut rng);
        assert_eq!(mac.len(), 12);
        assert!(mac.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_mac_address_cisco() {
        let mut rng = StdRng::seed_from_u64(42);
        let mac = mac_address_cisco(&mut rng);
        assert_eq!(mac.len(), 14); // XXXX.XXXX.XXXX
        assert_eq!(mac.matches('.').count(), 2);
    }

    #[test]
    fn test_mac_address_local() {
        let mut rng = StdRng::seed_from_u64(42);
        let mac = mac_address_local(&mut rng);
        // Second bit of first byte should be set
        let first_byte = u8::from_str_radix(&mac[0..2], 16).unwrap();
        assert_eq!(first_byte & 0x02, 0x02);
    }

    #[test]
    fn test_mac_address_with_oui() {
        let mut rng = StdRng::seed_from_u64(42);
        let oui = [0x00, 0x50, 0x56];
        let mac = mac_address_with_oui(&mut rng, oui);
        assert!(mac.starts_with("00:50:56:"));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(mac_address(&mut rng1), mac_address(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let mac = mac_address(&mut *rng);
        assert_eq!(mac.len(), 17);
    }
}
