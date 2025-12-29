//! UUID generation utilities.

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
}
