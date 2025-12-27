//! # Encode Command Implementation
//!
//! This example shows how to implement encoding/decoding with stdin support.
//!
//! Run with: `cargo run --example 0202_encode_command`

#![allow(dead_code)]

use base64::{engine::general_purpose, Engine};

// =========================================================================
// ENCODING FORMATS
// =========================================================================

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Base64,
    Base64UrlSafe,
    Hex,
}

// =========================================================================
// ENCODING
// =========================================================================

/// Encode bytes to string
pub fn encode(data: &[u8], format: Format) -> String {
    match format {
        Format::Base64 => general_purpose::STANDARD.encode(data),
        Format::Base64UrlSafe => general_purpose::URL_SAFE.encode(data),
        Format::Hex => hex::encode(data),
    }
}

/// Decode string to bytes
pub fn decode(input: &str, format: Format) -> Result<Vec<u8>, String> {
    let input = input.trim();
    match format {
        Format::Base64 => general_purpose::STANDARD
            .decode(input)
            .or_else(|_| general_purpose::STANDARD_NO_PAD.decode(input))
            .map_err(|e| format!("Invalid base64: {}", e)),
        Format::Base64UrlSafe => general_purpose::URL_SAFE
            .decode(input)
            .or_else(|_| general_purpose::URL_SAFE_NO_PAD.decode(input))
            .map_err(|e| format!("Invalid URL-safe base64: {}", e)),
        Format::Hex => hex::decode(input).map_err(|e| format!("Invalid hex: {}", e)),
    }
}

fn main() {
    println!("=== Encode Command Implementation ===\n");

    // =========================================================================
    // BASE64 ENCODING
    // =========================================================================

    println!("--- Base64 Encoding ---");
    let data = b"Hello, World!";

    let standard = encode(data, Format::Base64);
    let url_safe = encode(data, Format::Base64UrlSafe);

    println!("Input: \"Hello, World!\"");
    println!("  Standard Base64: {}", standard);
    println!("  URL-safe Base64: {}", url_safe);

    println!();

    // =========================================================================
    // HEX ENCODING
    // =========================================================================

    println!("--- Hex Encoding ---");
    let hex_encoded = encode(data, Format::Hex);
    println!("  Hex: {}", hex_encoded);

    println!();

    // =========================================================================
    // DECODING
    // =========================================================================

    println!("--- Decoding ---");
    let decoded = decode(&standard, Format::Base64).unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();
    println!("  Decoded: \"{}\"", decoded_str);

    let hex_decoded = decode(&hex_encoded, Format::Hex).unwrap();
    let hex_decoded_str = String::from_utf8(hex_decoded).unwrap();
    println!("  Hex decoded: \"{}\"", hex_decoded_str);

    println!();

    // =========================================================================
    // STDIN SUPPORT
    // =========================================================================

    println!("--- Stdin Support Pattern ---");
    println!(
        r#"
Reading from stdin:

use std::io::{{self, Read}};

fn read_stdin() -> io::Result<Vec<u8>> {{
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    Ok(buffer)
}}

// Usage:
// echo "hello" | dx encode
// cat file.bin | dx encode -f hex
"#
    );

    println!();

    // =========================================================================
    // BINARY DATA
    // =========================================================================

    println!("--- Binary Data Handling ---");
    let binary_data: &[u8] = &[0x00, 0xFF, 0x7F, 0x80, 0xDE, 0xAD, 0xBE, 0xEF];
    let hex = encode(binary_data, Format::Hex);
    let b64 = encode(binary_data, Format::Base64);
    println!("  Binary: {:02x?}", binary_data);
    println!("  Hex:    {}", hex);
    println!("  Base64: {}", b64);

    println!();

    // =========================================================================
    // CLI USAGE
    // =========================================================================

    println!("--- CLI Usage ---");
    println!(
        r#"
# Encode string to base64
dx encode -s "hello world"

# Encode file to hex
dx encode -f hex file.bin

# Decode base64
dx encode -d -s "aGVsbG8gd29ybGQ="

# Decode hex
dx encode -d -f hex -s "68656c6c6f"

# Pipe from stdin
echo -n "hello" | dx encode
cat binary.dat | dx encode -f hex

# URL-safe base64
dx encode -s "hello" --url-safe
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Encode command features:");
    println!("  1. Base64 standard and URL-safe");
    println!("  2. Hexadecimal encoding");
    println!("  3. Encode and decode modes");
    println!("  4. Stdin support for piping");
    println!("  5. Handles binary data safely");
}
