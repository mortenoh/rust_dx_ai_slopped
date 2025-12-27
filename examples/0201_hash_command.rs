//! # Hash Command Implementation
//!
//! This example shows how to implement file hashing with multiple algorithms.
//!
//! Run with: `cargo run --example 0201_hash_command`

#![allow(dead_code)]

use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

// =========================================================================
// ALGORITHM SELECTION
// =========================================================================

/// Supported hash algorithms
#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    Md5,
    Sha256,
    Sha512,
}

impl Algorithm {
    /// Get the output length in bytes
    pub fn output_len(&self) -> usize {
        match self {
            Algorithm::Md5 => 16,
            Algorithm::Sha256 => 32,
            Algorithm::Sha512 => 64,
        }
    }
}

// =========================================================================
// HASHING IMPLEMENTATION
// =========================================================================

/// Hash bytes using the specified algorithm
pub fn hash_bytes(data: &[u8], algorithm: Algorithm) -> String {
    match algorithm {
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
    }
}

/// Hash a file using streaming (memory efficient)
pub fn hash_file(path: &Path, algorithm: Algorithm) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    match algorithm {
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            let mut buffer = [0u8; 8192];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            let mut buffer = [0u8; 8192];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
    }
}

/// Verify a hash matches expected value
pub fn verify_hash(computed: &str, expected: &str) -> bool {
    computed.eq_ignore_ascii_case(expected)
}

fn main() {
    println!("=== Hash Command Implementation ===\n");

    // =========================================================================
    // BASIC HASHING
    // =========================================================================

    println!("--- Basic String Hashing ---");
    let data = b"hello world";

    let md5 = hash_bytes(data, Algorithm::Md5);
    let sha256 = hash_bytes(data, Algorithm::Sha256);
    let sha512 = hash_bytes(data, Algorithm::Sha512);

    println!("Input: \"hello world\"");
    println!("  MD5:    {}", md5);
    println!("  SHA256: {}", sha256);
    println!("  SHA512: {}...", &sha512[..32]);

    println!();

    // =========================================================================
    // VERIFICATION
    // =========================================================================

    println!("--- Hash Verification ---");
    let expected = "5eb63bbbe01eeed093cb22bb8f5acdc3";
    let matches = verify_hash(&md5, expected);
    println!("  Expected MD5: {}", expected);
    println!("  Computed MD5: {}", md5);
    println!("  Match: {}", matches);

    println!();

    // =========================================================================
    // IMPLEMENTATION DETAILS
    // =========================================================================

    println!("--- Implementation Pattern ---");
    println!(
        r#"
The hash command implementation:

1. ALGORITHM SELECTION via enum:
   pub enum Algorithm {{ Md5, Sha256, Sha512 }}

2. STREAMING for large files:
   let mut buffer = [0u8; 8192];
   loop {{
       let n = reader.read(&mut buffer)?;
       if n == 0 {{ break; }}
       hasher.update(&buffer[..n]);
   }}

3. HEX ENCODING output:
   hex::encode(hasher.finalize())

4. VERIFICATION (case-insensitive):
   computed.eq_ignore_ascii_case(expected)
"#
    );

    println!();

    // =========================================================================
    // CLI INTEGRATION
    // =========================================================================

    println!("--- CLI Usage ---");
    println!(
        r#"
# Hash a string
dx hash -s "hello world"
dx hash -s "hello world" -a sha512

# Hash a file
dx hash path/to/file.txt
dx hash path/to/file.txt -a md5

# Verify a hash
dx hash file.txt --verify abc123...

# Output only hash (for scripting)
dx hash -q -s "hello"
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Hash command features:");
    println!("  1. Multiple algorithms: MD5, SHA256, SHA512");
    println!("  2. String or file input");
    println!("  3. Streaming for memory efficiency");
    println!("  4. Verification mode");
    println!("  5. Quiet output for scripting");
}
