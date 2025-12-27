//! # UUID Command Implementation
//!
//! This module generates Universally Unique Identifiers (UUIDs).
//! UUIDs are 128-bit identifiers that are practically guaranteed to be unique.
//!
//! ## UUID Versions
//!
//! | Version | Method | Use Case |
//! |---------|--------|----------|
//! | V4 | Random | Most common, good for general use |
//! | V7 | Timestamp + Random | Sortable, good for databases |
//!
//! ### V4 (Random)
//! - 122 bits of randomness (6 bits reserved for version/variant)
//! - No ordering guarantee
//! - Best for: session IDs, correlation IDs, general identifiers
//!
//! ### V7 (Timestamp-based)
//! - Unix timestamp in milliseconds + random bits
//! - Naturally sortable (newer UUIDs sort after older ones)
//! - Best for: database primary keys, event ordering
//!
//! ## UUID Formats
//!
//! ```text
//! Standard: 550e8400-e29b-41d4-a716-446655440000
//! Simple:   550e8400e29b41d4a716446655440000
//! URN:      urn:uuid:550e8400-e29b-41d4-a716-446655440000
//! Braced:   {550e8400-e29b-41d4-a716-446655440000}
//! ```
//!
//! ## Example Usage
//! ```bash
//! dx uuid                        # Generate one V4 UUID
//! dx uuid --count 5              # Generate 5 UUIDs
//! dx uuid --version v7           # Generate V7 (sortable) UUID
//! dx uuid --format simple        # No hyphens
//! dx uuid --uppercase            # Uppercase hex digits
//! ```
//!
//! ## External Documentation
//! - UUID crate: <https://docs.rs/uuid>
//! - RFC 4122: <https://www.rfc-editor.org/rfc/rfc4122>
//! - UUID v7 draft: <https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-04.html>

use crate::cli::commands::uuid::{UuidArgs, UuidFormat, UuidVersion};
use anyhow::Result;
use uuid::Uuid;

/// Run the UUID command to generate one or more UUIDs.
///
/// # Features
/// - Generate multiple UUIDs with `--count`
/// - Choose version (V4 random or V7 timestamp)
/// - Format output (standard, simple, URN, braced)
/// - Uppercase option for hex digits
pub fn run(args: UuidArgs) -> Result<()> {
    // Generate the requested number of UUIDs
    for _ in 0..args.count {
        let uuid = generate_uuid(args.uuid_version);
        let formatted = format_uuid(&uuid, args.format, args.uppercase);
        println!("{}", formatted);
    }
    Ok(())
}

/// Generate a UUID of the specified version.
///
/// # V4 vs V7
///
/// **V4 (Random)**:
/// ```text
/// xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
///               ^    ^
///               |    variant (8, 9, a, or b)
///               version (always 4)
/// ```
/// Uses cryptographically secure random number generator.
///
/// **V7 (Timestamp)**:
/// ```text
/// TTTTTTTT-TTTT-7RRR-yRRR-RRRRRRRRRRRR
/// ^^^^^^^^ ^^^^      ^^^^ ^^^^^^^^^^^^
/// |        |         random bits
/// |        more timestamp + version
/// Unix ms timestamp (48 bits)
/// ```
/// Embeds Unix timestamp, making UUIDs naturally sortable.
fn generate_uuid(version: UuidVersion) -> Uuid {
    match version {
        UuidVersion::V4 => {
            // V4: Pure random UUID
            // Internally uses getrandom for cryptographic randomness
            Uuid::new_v4()
        }
        UuidVersion::V7 => {
            // V7: Timestamp-based UUID
            // now_v7() uses current system time
            // Great for database keys where ordering matters
            Uuid::now_v7()
        }
    }
}

/// Format a UUID according to the specified format.
///
/// # Format Options
///
/// | Format | Example | Length |
/// |--------|---------|--------|
/// | Standard | `550e8400-e29b-41d4-a716-446655440000` | 36 chars |
/// | Simple | `550e8400e29b41d4a716446655440000` | 32 chars |
/// | URN | `urn:uuid:550e8400-e29b-41d4-a716-446655440000` | 45 chars |
/// | Braced | `{550e8400-e29b-41d4-a716-446655440000}` | 38 chars |
///
/// # Uppercase
/// By default, hex digits are lowercase (a-f). The `uppercase` flag
/// converts them to A-F. Note: UUIDs are case-insensitive per RFC 4122.
fn format_uuid(uuid: &Uuid, format: UuidFormat, uppercase: bool) -> String {
    // Each format method returns a type that implements Display
    // We call to_string() to get the formatted string
    let formatted = match format {
        // Standard: hyphenated format (most common)
        UuidFormat::Standard => uuid.hyphenated().to_string(),

        // Simple: no hyphens (compact, good for filenames)
        UuidFormat::Simple => uuid.simple().to_string(),

        // URN: Uniform Resource Name format
        // Used in XML and some protocols
        UuidFormat::Urn => uuid.urn().to_string(),

        // Braced: Microsoft/Windows style
        // Common in Windows registry and COM
        UuidFormat::Braced => uuid.braced().to_string(),
    };

    // Apply uppercase transformation if requested
    if uppercase {
        formatted.to_uppercase()
    } else {
        formatted
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify V4 UUIDs have version number 4.
    #[test]
    fn test_uuid_v4_generation() {
        let uuid = generate_uuid(UuidVersion::V4);
        assert_eq!(uuid.get_version_num(), 4);
    }

    /// Verify V7 UUIDs have version number 7.
    #[test]
    fn test_uuid_v7_generation() {
        let uuid = generate_uuid(UuidVersion::V7);
        assert_eq!(uuid.get_version_num(), 7);
    }

    /// Test standard (hyphenated) format.
    #[test]
    fn test_format_standard() {
        let uuid = Uuid::nil(); // All zeros for predictable output
        let formatted = format_uuid(&uuid, UuidFormat::Standard, false);
        assert_eq!(formatted, "00000000-0000-0000-0000-000000000000");
    }

    /// Test simple (no hyphens) format.
    #[test]
    fn test_format_simple() {
        let uuid = Uuid::nil();
        let formatted = format_uuid(&uuid, UuidFormat::Simple, false);
        assert_eq!(formatted, "00000000000000000000000000000000");
    }

    /// Test uppercase formatting.
    #[test]
    fn test_format_uppercase() {
        let uuid = Uuid::parse_str("a1b2c3d4-e5f6-4789-abcd-ef0123456789").unwrap();
        let formatted = format_uuid(&uuid, UuidFormat::Standard, true);
        assert_eq!(formatted, "A1B2C3D4-E5F6-4789-ABCD-EF0123456789");
    }
}
