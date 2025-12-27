//! # UUID Command Implementation
//!
//! This example shows how to implement UUID generation with format options.
//!
//! Run with: `cargo run --example 0203_uuid_command`

#![allow(dead_code)]

use uuid::Uuid;

// =========================================================================
// UUID VERSIONS
// =========================================================================

#[derive(Debug, Clone, Copy)]
pub enum Version {
    /// Random UUID (version 4)
    V4,
    /// Time-ordered UUID (version 7)
    V7,
}

// =========================================================================
// OUTPUT FORMATS
// =========================================================================

#[derive(Debug, Clone, Copy)]
pub enum Format {
    /// Standard: 8-4-4-4-12
    Standard,
    /// Simple: 32 hex chars, no dashes
    Simple,
    /// URN: urn:uuid:...
    Urn,
    /// Braced: {uuid}
    Braced,
}

// =========================================================================
// GENERATION
// =========================================================================

/// Generate a UUID
pub fn generate(version: Version) -> Uuid {
    match version {
        Version::V4 => Uuid::new_v4(),
        Version::V7 => Uuid::now_v7(),
    }
}

/// Format a UUID
pub fn format_uuid(uuid: &Uuid, format: Format, uppercase: bool) -> String {
    let s = match format {
        Format::Standard => uuid.hyphenated().to_string(),
        Format::Simple => uuid.simple().to_string(),
        Format::Urn => uuid.urn().to_string(),
        Format::Braced => uuid.braced().to_string(),
    };

    if uppercase {
        s.to_uppercase()
    } else {
        s
    }
}

/// Parse a UUID string
pub fn parse(s: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(s.trim())
}

fn main() {
    println!("=== UUID Command Implementation ===\n");

    // =========================================================================
    // UUID V4 (RANDOM)
    // =========================================================================

    println!("--- UUID v4 (Random) ---");
    println!("UUIDs v4 are randomly generated:");
    for i in 1..=3 {
        let uuid = generate(Version::V4);
        println!("  {}: {}", i, format_uuid(&uuid, Format::Standard, false));
    }

    println!();

    // =========================================================================
    // UUID V7 (TIME-ORDERED)
    // =========================================================================

    println!("--- UUID v7 (Time-ordered) ---");
    println!("UUIDs v7 are time-ordered (sortable):");
    for i in 1..=3 {
        let uuid = generate(Version::V7);
        println!("  {}: {}", i, format_uuid(&uuid, Format::Standard, false));
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    println!("  Note: v7 UUIDs sort chronologically!");

    println!();

    // =========================================================================
    // OUTPUT FORMATS
    // =========================================================================

    println!("--- Output Formats ---");
    let uuid = generate(Version::V4);
    println!(
        "  Standard: {}",
        format_uuid(&uuid, Format::Standard, false)
    );
    println!("  Simple:   {}", format_uuid(&uuid, Format::Simple, false));
    println!("  URN:      {}", format_uuid(&uuid, Format::Urn, false));
    println!("  Braced:   {}", format_uuid(&uuid, Format::Braced, false));
    println!("  Upper:    {}", format_uuid(&uuid, Format::Standard, true));

    println!();

    // =========================================================================
    // PARSING
    // =========================================================================

    println!("--- Parsing UUIDs ---");
    let valid = "550e8400-e29b-41d4-a716-446655440000";
    let parsed = parse(valid);
    println!("  Input:  {}", valid);
    println!("  Parsed: {:?}", parsed);

    let invalid = "not-a-uuid";
    let parsed = parse(invalid);
    println!("  Invalid: {:?}", parsed);

    println!();

    // =========================================================================
    // UUID COMPONENTS
    // =========================================================================

    println!("--- UUID Components ---");
    let uuid = generate(Version::V4);
    println!("  UUID: {}", uuid);
    println!("  Version: {}", uuid.get_version_num());
    println!("  Variant: {:?}", uuid.get_variant());
    println!("  Bytes: {:02x?}", uuid.as_bytes());

    println!();

    // =========================================================================
    // CLI USAGE
    // =========================================================================

    println!("--- CLI Usage ---");
    println!(
        r#"
# Generate single UUID (v4 default)
dx uuid

# Generate multiple UUIDs
dx uuid -c 5

# Generate v7 (time-ordered)
dx uuid -T v7

# Different formats
dx uuid -f simple
dx uuid -f urn
dx uuid -f braced

# Uppercase
dx uuid -U

# Combined
dx uuid -T v7 -c 10 -f simple -U
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("UUID command features:");
    println!("  1. Version 4 (random) and 7 (time-ordered)");
    println!("  2. Multiple output formats");
    println!("  3. Batch generation");
    println!("  4. Uppercase option");
    println!("  5. v7 for sortable/time-based use cases");
}
