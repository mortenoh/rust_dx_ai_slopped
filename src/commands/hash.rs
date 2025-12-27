//! # Hash Command Implementation
//!
//! This module implements cryptographic hashing functionality for the CLI.
//! It supports multiple hash algorithms and can hash files, strings, or stdin.
//!
//! ## Supported Algorithms
//!
//! - **MD5**: 128-bit hash, fast but cryptographically broken. Use only for checksums.
//! - **SHA-256**: 256-bit hash from the SHA-2 family. Recommended for most use cases.
//! - **SHA-512**: 512-bit hash, more secure but slower than SHA-256.
//!
//! ## Key Concepts
//!
//! ### Cryptographic Hash Functions
//! A hash function takes arbitrary input and produces a fixed-size output (digest).
//! Properties: deterministic, fast, one-way (can't reverse), collision-resistant.
//!
//! ### The Digest Trait
//! The `sha2` and `md5` crates use the `Digest` trait from the `digest` crate.
//! This provides a common interface: `new()`, `update()`, `finalize()`.
//!
//! ## Example Usage
//! ```bash
//! dx hash file.txt                    # SHA-256 (default)
//! dx hash -a md5 file.txt             # MD5
//! dx hash -s "hello world"            # Hash a string
//! echo "data" | dx hash -             # Hash from stdin
//! dx hash --verify abc123 file.txt    # Verify against expected hash
//! ```
//!
//! ## External Documentation
//! - SHA-2: <https://docs.rs/sha2>
//! - MD5: <https://docs.rs/md5>
//! - Digest trait: <https://docs.rs/digest>

use crate::cli::commands::hash::{Algorithm, HashArgs};
use anyhow::{Context, Result};
use colored::Colorize;
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

/// Run the hash command with the provided arguments.
///
/// This is the main entry point for the hash subcommand. It:
/// 1. Reads input data from file, string, or stdin
/// 2. Computes the hash using the specified algorithm
/// 3. Either verifies against an expected hash or prints the result
///
/// # Arguments
/// * `args` - Parsed command-line arguments containing input source, algorithm, etc.
///
/// # Returns
/// * `Ok(())` on success
/// * `Err` if reading input fails or verification fails
pub fn run(args: HashArgs) -> Result<()> {
    // Step 1: Get input data and its source description (for display)
    let (data, source) = get_input(&args)?;

    // Step 2: Compute the hash using the selected algorithm
    let hash = compute_hash(&data, args.algorithm);

    // Step 3: Either verify or display the hash
    if let Some(expected) = &args.verify {
        // Verification mode: compare computed hash with expected
        verify_hash(&hash, expected, args.algorithm)?;
    } else if args.quiet {
        // Quiet mode: just print the hash (useful for scripts)
        println!("{}", hash);
    } else {
        // Normal mode: print with colors and source info
        // Format: "source (algorithm) = hash"
        println!("{} ({}) = {}", source.cyan(), args.algorithm, hash.green());
    }

    Ok(())
}

/// Get input data from one of three sources: string, file, or stdin.
///
/// The priority order is:
/// 1. `--string` flag (hash the literal string)
/// 2. File path argument (or "-" for stdin)
/// 3. Default to stdin if nothing specified
///
/// # Returns
/// A tuple of (data bytes, source description for display)
///
/// # Why Vec<u8> instead of &[u8]?
/// We need to own the data because it might come from different sources
/// with different lifetimes (stdin buffer, file contents, string clone).
fn get_input(args: &HashArgs) -> Result<(Vec<u8>, String)> {
    if let Some(s) = &args.string {
        // --string "text": Convert string to bytes
        // as_bytes() gives us a UTF-8 byte representation
        Ok((s.as_bytes().to_vec(), "string".to_string()))
    } else if let Some(path) = &args.input {
        // File path provided
        if path.to_string_lossy() == "-" {
            // Special case: "-" means read from stdin (Unix convention)
            let mut data = Vec::new();
            io::stdin()
                .read_to_end(&mut data)
                .context("Failed to read from stdin")?;
            Ok((data, "stdin".to_string()))
        } else {
            // Normal file path
            let data = read_file(path)?;
            Ok((data, path.display().to_string()))
        }
    } else {
        // No input specified: default to reading stdin
        // This allows: echo "data" | dx hash
        let mut data = Vec::new();
        io::stdin()
            .read_to_end(&mut data)
            .context("Failed to read from stdin")?;
        Ok((data, "stdin".to_string()))
    }
}

/// Read an entire file into memory.
///
/// # Why BufReader?
/// `BufReader` wraps the file with an internal buffer, reducing the number
/// of system calls. For small files this doesn't matter much, but for larger
/// files it improves performance significantly.
///
/// # Memory Considerations
/// This reads the entire file into memory. For very large files (multiple GB),
/// you might want to use streaming hash computation instead. See the `Digest`
/// trait's `update()` method which can be called multiple times with chunks.
///
/// # Arguments
/// * `path` - Path to the file to read
///
/// # Returns
/// * `Ok(Vec<u8>)` - The file contents as bytes
/// * `Err` - If the file cannot be opened or read
fn read_file(path: &Path) -> Result<Vec<u8>> {
    // Open the file, providing context on failure
    let file = File::open(path).with_context(|| format!("Failed to open {}", path.display()))?;

    // Wrap in BufReader for buffered reading (typically 8KB buffer)
    // See: https://doc.rust-lang.org/std/io/struct.BufReader.html
    let mut reader = BufReader::new(file);

    // Pre-allocate vector (could use file metadata for capacity hint)
    let mut data = Vec::new();

    // Read entire file into the vector
    reader
        .read_to_end(&mut data)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    Ok(data)
}

/// Compute the cryptographic hash of data using the specified algorithm.
///
/// # How Hashing Works
///
/// 1. Create a new hasher instance for the algorithm
/// 2. Feed data into the hasher with `update()` (can be called multiple times)
/// 3. Finalize to get the hash digest
/// 4. Encode the binary digest as hexadecimal string
///
/// # Algorithm Comparison
///
/// | Algorithm | Output Size | Speed    | Security           |
/// |-----------|-------------|----------|-------------------|
/// | MD5       | 128 bits    | Fastest  | Broken (collisions found) |
/// | SHA-256   | 256 bits    | Fast     | Secure            |
/// | SHA-512   | 512 bits    | Slower   | Most secure       |
///
/// # When to Use Each
///
/// - **MD5**: Only for non-security checksums (file integrity, cache keys)
/// - **SHA-256**: General purpose, file verification, most applications
/// - **SHA-512**: When you need extra security margin
///
/// # Arguments
/// * `data` - The bytes to hash
/// * `algorithm` - Which hash algorithm to use
///
/// # Returns
/// The hash as a lowercase hexadecimal string
pub fn compute_hash(data: &[u8], algorithm: Algorithm) -> String {
    match algorithm {
        Algorithm::Md5 => {
            // MD5 produces a 128-bit (16 byte) hash
            // Note: MD5 is cryptographically broken - don't use for security!
            // See: https://www.kb.cert.org/vuls/id/836068
            let mut hasher = Md5::new();
            hasher.update(data);
            // finalize() consumes the hasher and returns the digest
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha256 => {
            // SHA-256 produces a 256-bit (32 byte) hash
            // Part of the SHA-2 family, designed by NSA
            // See: https://en.wikipedia.org/wiki/SHA-2
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha512 => {
            // SHA-512 produces a 512-bit (64 byte) hash
            // Slower but provides larger security margin
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
    }
}

/// Verify that a computed hash matches an expected value.
///
/// # Case Insensitivity
/// Hash comparison is case-insensitive because hex digits can be uppercase
/// or lowercase (e.g., "ABC123" == "abc123"). We normalize to lowercase.
///
/// # Exit Behavior
/// On mismatch, this function calls `std::process::exit(1)` rather than
/// returning an error. This ensures the CLI returns a non-zero exit code
/// that scripts can check.
///
/// # Arguments
/// * `computed` - The hash we calculated
/// * `expected` - The hash we're comparing against
/// * `algorithm` - The algorithm used (for display purposes)
///
/// # Returns
/// * `Ok(())` if hashes match
/// * Exits with code 1 if hashes don't match
fn verify_hash(computed: &str, expected: &str, algorithm: Algorithm) -> Result<()> {
    // Normalize to lowercase for case-insensitive comparison
    let expected_lower = expected.to_lowercase();

    if computed == expected_lower {
        // Success: print confirmation with green checkmark
        println!(
            "{} {} hash verified",
            "✓".green().bold(),
            algorithm.to_string().cyan()
        );
        Ok(())
    } else {
        // Failure: print detailed mismatch info and exit
        eprintln!("{} Hash mismatch!", "✗".red().bold());
        eprintln!("  Expected: {}", expected_lower.yellow());
        eprintln!("  Got:      {}", computed.red());
        // Exit with non-zero code so scripts can detect failure
        // This is intentional: verification failure is a "hard" error
        std::process::exit(1);
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test MD5 hash computation against known value.
    /// MD5("hello world") = 5eb63bbbe01eeed093cb22bb8f5acdc3
    #[test]
    fn test_md5_hash() {
        let data = b"hello world";
        let hash = compute_hash(data, Algorithm::Md5);
        assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    /// Test SHA-256 hash computation against known value.
    /// You can verify with: echo -n "hello world" | sha256sum
    #[test]
    fn test_sha256_hash() {
        let data = b"hello world";
        let hash = compute_hash(data, Algorithm::Sha256);
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    /// Test SHA-512 hash computation (just check prefix due to length).
    #[test]
    fn test_sha512_hash() {
        let data = b"hello world";
        let hash = compute_hash(data, Algorithm::Sha512);
        // SHA-512 produces 128 hex characters, just check the start
        assert!(hash.starts_with("309ecc489c12d6eb"));
    }
}
