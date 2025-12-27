//! # Encode Command Implementation
//!
//! This module provides encoding and decoding functionality for common formats.
//! It's useful for working with APIs, data interchange, and debugging.
//!
//! ## Supported Formats
//!
//! - **Base64**: Binary-to-text encoding using 64 ASCII characters
//! - **Hex**: Binary-to-text encoding using hexadecimal (0-9, a-f)
//!
//! ## Base64 Variants
//!
//! Base64 has multiple variants for different use cases:
//!
//! | Variant | Characters | Padding | Use Case |
//! |---------|------------|---------|----------|
//! | Standard | A-Z, a-z, 0-9, +, / | = | Email, MIME |
//! | URL-safe | A-Z, a-z, 0-9, -, _ | = | URLs, filenames |
//! | No-padding | (either alphabet) | none | JWTs, compact storage |
//!
//! ### Why Different Alphabets?
//! - `+` and `/` have special meaning in URLs, so URL-safe uses `-` and `_`
//! - Padding (`=`) can be omitted when length is known (saves bytes in JWTs)
//!
//! ## Example Usage
//! ```bash
//! dx encode "hello world"              # Base64 encode
//! dx encode --hex "hello"              # Hex encode
//! dx encode --decode "aGVsbG8="        # Decode base64
//! dx encode --url-safe "hello"         # URL-safe base64
//! dx encode --no-padding "hello"       # No padding
//! ```
//!
//! ## External Documentation
//! - Base64 crate: <https://docs.rs/base64>
//! - Hex crate: <https://docs.rs/hex>
//! - Base64 RFC 4648: <https://datatracker.ietf.org/doc/html/rfc4648>

use crate::cli::commands::encode::{EncodeArgs, EncodingFormat};
use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose, Engine};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Run the encode/decode command with the provided arguments.
///
/// This function handles both encoding and decoding based on the `--decode` flag.
/// The encode/decode distinction is controlled by a single flag to keep the CLI simple.
///
/// # Arguments
/// * `args` - Parsed command-line arguments
///
/// # Returns
/// * `Ok(())` on success, prints result to stdout
/// * `Err` if input cannot be read or decoded data is invalid
pub fn run(args: EncodeArgs) -> Result<()> {
    // Get input data from string, file, or stdin
    let data = get_input(&args)?;

    // Encode or decode based on the --decode flag
    let result = if args.decode {
        // Decoding: convert encoded text back to original
        decode(&data, args.format)?
    } else {
        // Encoding: convert binary data to text representation
        encode(&data, args.format, args.url_safe, args.no_padding)
    };

    // Print result (no trailing newline issues since println adds one)
    println!("{}", result);
    Ok(())
}

/// Get input data from one of three sources.
///
/// Priority order:
/// 1. `--string` argument (literal string to encode/decode)
/// 2. File path (or "-" for stdin)
/// 3. Default to stdin
///
/// # Note on String vs Binary
/// When encoding, the string's UTF-8 bytes are encoded.
/// When decoding, the input is treated as encoded text to decode.
fn get_input(args: &EncodeArgs) -> Result<Vec<u8>> {
    if let Some(s) = &args.string {
        // Direct string input
        Ok(s.as_bytes().to_vec())
    } else if let Some(path) = &args.input {
        if path.to_string_lossy() == "-" {
            // "-" is the Unix convention for stdin
            let mut data = Vec::new();
            io::stdin()
                .read_to_end(&mut data)
                .context("Failed to read from stdin")?;
            Ok(data)
        } else {
            // Read from file
            read_file(path)
        }
    } else {
        // Default: read from stdin (allows piping)
        let mut data = Vec::new();
        io::stdin()
            .read_to_end(&mut data)
            .context("Failed to read from stdin")?;
        Ok(data)
    }
}

/// Read file contents into a byte vector.
fn read_file(path: &Path) -> Result<Vec<u8>> {
    let mut file =
        File::open(path).with_context(|| format!("Failed to open {}", path.display()))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    Ok(data)
}

/// Encode binary data to a text format.
///
/// # Base64 Engine Selection
///
/// The `base64` crate uses an "engine" pattern where different engines
/// implement different Base64 variants. We select based on flags:
///
/// ```text
/// url_safe  no_padding  Engine
/// ────────  ──────────  ──────────────────────
/// false     false       STANDARD (default)
/// false     true        STANDARD_NO_PAD
/// true      false       URL_SAFE
/// true      true        URL_SAFE_NO_PAD
/// ```
///
/// # Arguments
/// * `data` - Raw bytes to encode
/// * `format` - Base64 or Hex
/// * `url_safe` - Use URL-safe alphabet (- and _ instead of + and /)
/// * `no_padding` - Omit trailing = padding characters
///
/// # Returns
/// The encoded string
fn encode(data: &[u8], format: EncodingFormat, url_safe: bool, no_padding: bool) -> String {
    match format {
        EncodingFormat::Base64 => {
            // Select the appropriate engine based on options
            // The engine encapsulates both the alphabet and padding behavior
            // See: https://docs.rs/base64/latest/base64/engine/index.html
            match (url_safe, no_padding) {
                (true, true) => general_purpose::URL_SAFE_NO_PAD.encode(data),
                (true, false) => general_purpose::URL_SAFE.encode(data),
                (false, true) => general_purpose::STANDARD_NO_PAD.encode(data),
                (false, false) => general_purpose::STANDARD.encode(data),
            }
        }
        EncodingFormat::Hex => {
            // Hex encoding is simpler: each byte becomes two hex characters
            // "hello" -> "68656c6c6f"
            // 0x68 = 'h', 0x65 = 'e', etc.
            hex::encode(data)
        }
    }
}

/// Decode text back to its original form.
///
/// # Auto-Detection Strategy
///
/// For Base64, we try multiple variants in order because:
/// 1. We can't always tell which variant was used just by looking
/// 2. URL-safe and standard differ only in a few characters
/// 3. Padding may or may not be present
///
/// The order of attempts is:
/// 1. STANDARD (most common)
/// 2. URL_SAFE (for URLs and JWTs)
/// 3. STANDARD_NO_PAD
/// 4. URL_SAFE_NO_PAD
///
/// # Why `or_else` Chain?
/// The `or_else` combinator tries the next decoder only if the previous failed.
/// This is lazy: if STANDARD works, we never try the others.
///
/// # UTF-8 Handling
/// Decoded bytes are converted to a UTF-8 string. If the bytes aren't valid
/// UTF-8 (e.g., binary data), we return an error with the raw bytes shown.
///
/// # Arguments
/// * `data` - Encoded text as bytes
/// * `format` - Base64 or Hex
///
/// # Returns
/// * `Ok(String)` - The decoded text
/// * `Err` if decoding fails or result isn't valid UTF-8
fn decode(data: &[u8], format: EncodingFormat) -> Result<String> {
    // Convert bytes to string and trim whitespace
    // from_utf8_lossy handles any non-UTF8 by replacing with �
    let input = String::from_utf8_lossy(data);
    let input = input.trim(); // Remove leading/trailing whitespace and newlines

    let bytes = match format {
        EncodingFormat::Base64 => {
            // Try multiple Base64 variants until one succeeds
            // This provides a better user experience - users don't need to
            // know which variant was used to encode the data
            general_purpose::STANDARD
                .decode(input)
                .or_else(|_| general_purpose::URL_SAFE.decode(input))
                .or_else(|_| general_purpose::STANDARD_NO_PAD.decode(input))
                .or_else(|_| general_purpose::URL_SAFE_NO_PAD.decode(input))
                .context("Invalid base64 input")?
        }
        EncodingFormat::Hex => {
            // Hex decoding: "68656c6c6f" -> [0x68, 0x65, 0x6c, 0x6c, 0x6f]
            hex::decode(input).context("Invalid hex input")?
        }
    };

    // Try to convert decoded bytes to a UTF-8 string
    String::from_utf8(bytes.clone())
        .map(|s| s.to_string())
        .or_else(|_| {
            // If not valid UTF-8, the decoded data is likely binary
            // We can't display it as a string, so show an error with raw bytes
            bail!("Decoded data is not valid UTF-8. Raw bytes: {:?}", bytes)
        })
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test standard Base64 encoding.
    /// "hello world" -> "aGVsbG8gd29ybGQ="
    #[test]
    fn test_base64_encode() {
        let data = b"hello world";
        let encoded = encode(data, EncodingFormat::Base64, false, false);
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
    }

    /// Test Base64 decoding with padding.
    #[test]
    fn test_base64_decode() {
        let data = b"aGVsbG8gd29ybGQ=";
        let decoded = decode(data, EncodingFormat::Base64).unwrap();
        assert_eq!(decoded, "hello world");
    }

    /// Test hexadecimal encoding.
    /// Each character becomes its ASCII hex value.
    #[test]
    fn test_hex_encode() {
        let data = b"hello";
        let encoded = encode(data, EncodingFormat::Hex, false, false);
        assert_eq!(encoded, "68656c6c6f");
    }

    /// Test hexadecimal decoding.
    #[test]
    fn test_hex_decode() {
        let data = b"68656c6c6f";
        let decoded = decode(data, EncodingFormat::Hex).unwrap();
        assert_eq!(decoded, "hello");
    }
}
