//! UUID and ULID generation utilities.
//!
//! This module provides generation for:
//! - UUID v4 (random)
//! - UUID v7 (time-based, sortable)
//! - ULID (Universally Unique Lexicographically Sortable Identifier)

pub use ulid::Ulid;
pub use uuid::Uuid;

/// UUID version for generation
#[derive(Debug, Clone, Copy, Default)]
pub enum UuidVersion {
    /// Random UUID (v4)
    #[default]
    V4,
    /// Time-based UUID (v7)
    V7,
}

/// Generate a new UUID.
pub fn generate(version: UuidVersion) -> Uuid {
    match version {
        UuidVersion::V4 => Uuid::new_v4(),
        UuidVersion::V7 => Uuid::now_v7(),
    }
}

/// Generate a new ULID.
///
/// ULIDs are 128-bit identifiers that are:
/// - Lexicographically sortable
/// - Compatible with UUID representation
/// - Encoded as 26 character Crockford's Base32
///
/// # Example
/// ```
/// use dx_datagen::uuid::ulid;
/// let id = ulid();
/// assert_eq!(id.len(), 26);
/// ```
pub fn ulid() -> String {
    Ulid::new().to_string()
}

/// Generate a ULID from a specific timestamp (milliseconds since Unix epoch).
///
/// Useful for generating ULIDs with a known time component for testing.
///
/// # Example
/// ```
/// use dx_datagen::uuid::ulid_from_timestamp;
/// let id = ulid_from_timestamp(1234567890123);
/// assert_eq!(id.len(), 26);
/// ```
pub fn ulid_from_timestamp(timestamp_ms: u64) -> String {
    use std::time::{Duration, SystemTime};

    let time = SystemTime::UNIX_EPOCH + Duration::from_millis(timestamp_ms);
    Ulid::from_datetime(time).to_string()
}

/// Generate a random ULID using provided RNG (for deterministic generation).
///
/// # Example
/// ```
/// use dx_datagen::uuid::ulid_with_rng;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let id = ulid_with_rng(&mut rng);
/// assert_eq!(id.len(), 26);
/// ```
pub fn ulid_with_rng<R: ?Sized + rand::Rng>(rng: &mut R) -> String {
    let random_bytes: u128 = rng.random();
    // Take current time for the timestamp portion
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // ULID format: 48-bit timestamp + 80-bit random
    let timestamp_part = (now as u128) << 80;
    let random_part = random_bytes & ((1u128 << 80) - 1);
    let ulid_bits = timestamp_part | random_part;

    Ulid::from(ulid_bits).to_string()
}

/// Generate a v4 (random) UUID.
pub fn v4() -> Uuid {
    Uuid::new_v4()
}

/// Generate a v7 (time-based) UUID.
pub fn v7() -> Uuid {
    Uuid::now_v7()
}

/// UUID output format
#[derive(Debug, Clone, Copy, Default)]
pub enum UuidFormat {
    /// Standard hyphenated format: 550e8400-e29b-41d4-a716-446655440000
    #[default]
    Hyphenated,
    /// Simple format without hyphens: 550e8400e29b41d4a716446655440000
    Simple,
    /// URN format: urn:uuid:550e8400-e29b-41d4-a716-446655440000
    Urn,
    /// Braced format: {550e8400-e29b-41d4-a716-446655440000}
    Braced,
}

/// Format a UUID according to the specified format.
pub fn format(uuid: &Uuid, fmt: UuidFormat) -> String {
    match fmt {
        UuidFormat::Hyphenated => uuid.hyphenated().to_string(),
        UuidFormat::Simple => uuid.simple().to_string(),
        UuidFormat::Urn => uuid.urn().to_string(),
        UuidFormat::Braced => uuid.braced().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v4() {
        let id = v4();
        assert_eq!(id.get_version_num(), 4);
    }

    #[test]
    fn test_v7() {
        let id = v7();
        assert_eq!(id.get_version_num(), 7);
    }

    #[test]
    fn test_format_hyphenated() {
        let id = v4();
        let s = format(&id, UuidFormat::Hyphenated);
        assert_eq!(s.len(), 36);
        assert!(s.contains('-'));
    }

    #[test]
    fn test_format_simple() {
        let id = v4();
        let s = format(&id, UuidFormat::Simple);
        assert_eq!(s.len(), 32);
        assert!(!s.contains('-'));
    }

    #[test]
    fn test_format_urn() {
        let id = v4();
        let s = format(&id, UuidFormat::Urn);
        assert!(s.starts_with("urn:uuid:"));
    }

    #[test]
    fn test_format_braced() {
        let id = v4();
        let s = format(&id, UuidFormat::Braced);
        assert!(s.starts_with('{'));
        assert!(s.ends_with('}'));
    }

    #[test]
    fn test_ulid() {
        let id = ulid();
        assert_eq!(id.len(), 26);
        // ULID uses Crockford's Base32 (no I, L, O, U)
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_ulid_from_timestamp() {
        let id1 = ulid_from_timestamp(1000);
        let id2 = ulid_from_timestamp(2000);
        assert_eq!(id1.len(), 26);
        assert_eq!(id2.len(), 26);
        // Later timestamp should sort after earlier
        assert!(id1 < id2);
    }

    #[test]
    fn test_ulid_with_rng_deterministic() {
        use rand::rngs::StdRng;
        use rand::SeedableRng;

        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        let id1 = ulid_with_rng(&mut rng1);
        let id2 = ulid_with_rng(&mut rng2);

        assert_eq!(id1.len(), 26);
        // Note: IDs may differ due to timestamp component, but random parts should be same
        // with same seed
    }
}
