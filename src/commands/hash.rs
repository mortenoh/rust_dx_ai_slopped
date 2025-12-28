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
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use colored::Colorize;
use md5::Md5;
use rand::Rng;
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

    // Step 2: Either verify or compute and display the hash
    if let Some(expected) = &args.verify {
        // Verification mode: use algorithm-specific verification
        verify_hash(&data, expected, args.algorithm)?;
    } else {
        // Compute the hash using the selected algorithm
        let hash = compute_hash(&data, args.algorithm, args.cost)?;

        if args.quiet {
            // Quiet mode: just print the hash (useful for scripts)
            println!("{}", hash);
        } else {
            // Normal mode: print with colors and source info
            // Format: "source (algorithm) = hash"
            println!("{} ({}) = {}", source.cyan(), args.algorithm, hash.green());
        }
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
/// | Bcrypt    | Variable    | Slow     | Password hashing  |
/// | Argon2    | Variable    | Slow     | Modern password hash |
///
/// # When to Use Each
///
/// - **MD5**: Only for non-security checksums (file integrity, cache keys)
/// - **SHA-256**: General purpose, file verification, most applications
/// - **SHA-512**: When you need extra security margin
/// - **Bcrypt**: Password hashing with configurable cost
/// - **Argon2**: Modern password hashing, memory-hard
///
/// # Arguments
/// * `data` - The bytes to hash
/// * `algorithm` - Which hash algorithm to use
/// * `cost` - Cost factor for bcrypt/argon2 (ignored for other algorithms)
///
/// # Returns
/// The hash as a string (hex for MD5/SHA, PHC format for bcrypt/argon2)
pub fn compute_hash(data: &[u8], algorithm: Algorithm, cost: u32) -> Result<String> {
    match algorithm {
        Algorithm::Md5 => {
            // MD5 produces a 128-bit (16 byte) hash
            // Note: MD5 is cryptographically broken - don't use for security!
            // See: https://www.kb.cert.org/vuls/id/836068
            let mut hasher = Md5::new();
            hasher.update(data);
            // finalize() consumes the hasher and returns the digest
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Sha256 => {
            // SHA-256 produces a 256-bit (32 byte) hash
            // Part of the SHA-2 family, designed by NSA
            // See: https://en.wikipedia.org/wiki/SHA-2
            let mut hasher = Sha256::new();
            hasher.update(data);
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Sha512 => {
            // SHA-512 produces a 512-bit (64 byte) hash
            // Slower but provides larger security margin
            let mut hasher = Sha512::new();
            hasher.update(data);
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Bcrypt => {
            // Bcrypt password hash with configurable cost (4-31)
            // Note: bcrypt has a 72 byte password limit
            let hash = bcrypt::hash(data, cost).context("Failed to compute bcrypt hash")?;
            Ok(hash)
        }
        Algorithm::Argon2 => {
            // Argon2id password hash (memory-hard, recommended for passwords)
            // Generate 16 random bytes for salt
            let mut salt_bytes = [0u8; 16];
            rand::rng().fill(&mut salt_bytes);
            let salt = SaltString::encode_b64(&salt_bytes)
                .map_err(|e| anyhow::anyhow!("Failed to encode salt: {}", e))?;
            let argon2 = Argon2::default();
            let hash = argon2
                .hash_password(data, &salt)
                .map_err(|e| anyhow::anyhow!("Failed to compute argon2 hash: {}", e))?
                .to_string();
            Ok(hash)
        }
    }
}

/// Verify that input data matches an expected hash value.
///
/// # Algorithm-Specific Verification
/// - MD5/SHA: Compute hash and compare (case-insensitive hex comparison)
/// - Bcrypt/Argon2: Use built-in verification (hash contains salt)
///
/// # Exit Behavior
/// On mismatch, this function calls `std::process::exit(1)` rather than
/// returning an error. This ensures the CLI returns a non-zero exit code
/// that scripts can check.
///
/// # Arguments
/// * `data` - The original input data
/// * `expected` - The hash we're comparing against
/// * `algorithm` - The algorithm used
///
/// # Returns
/// * `Ok(())` if verification passes
/// * Exits with code 1 if verification fails
fn verify_hash(data: &[u8], expected: &str, algorithm: Algorithm) -> Result<()> {
    let verified = match algorithm {
        Algorithm::Md5 | Algorithm::Sha256 | Algorithm::Sha512 => {
            // For standard hash algorithms, compute and compare
            let computed = compute_hash(data, algorithm, 0)?;
            let expected_lower = expected.to_lowercase();
            if computed == expected_lower {
                true
            } else {
                eprintln!("{} Hash mismatch!", "✗".red().bold());
                eprintln!("  Expected: {}", expected_lower.yellow());
                eprintln!("  Got:      {}", computed.red());
                false
            }
        }
        Algorithm::Bcrypt => {
            // Bcrypt has built-in verification
            match bcrypt::verify(data, expected) {
                Ok(true) => true,
                Ok(false) => {
                    eprintln!("{} Bcrypt verification failed!", "✗".red().bold());
                    false
                }
                Err(e) => {
                    eprintln!("{} Bcrypt verification error: {}", "✗".red().bold(), e);
                    false
                }
            }
        }
        Algorithm::Argon2 => {
            // Argon2 has built-in verification
            match PasswordHash::new(expected) {
                Ok(parsed_hash) => {
                    if Argon2::default()
                        .verify_password(data, &parsed_hash)
                        .is_ok()
                    {
                        true
                    } else {
                        eprintln!("{} Argon2 verification failed!", "✗".red().bold());
                        false
                    }
                }
                Err(e) => {
                    eprintln!("{} Invalid Argon2 hash format: {}", "✗".red().bold(), e);
                    false
                }
            }
        }
    };

    if verified {
        println!(
            "{} {} hash verified",
            "✓".green().bold(),
            algorithm.to_string().cyan()
        );
        Ok(())
    } else {
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
        let hash = compute_hash(data, Algorithm::Md5, 0).unwrap();
        assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    /// Test SHA-256 hash computation against known value.
    /// You can verify with: echo -n "hello world" | sha256sum
    #[test]
    fn test_sha256_hash() {
        let data = b"hello world";
        let hash = compute_hash(data, Algorithm::Sha256, 0).unwrap();
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    /// Test SHA-512 hash computation (just check prefix due to length).
    #[test]
    fn test_sha512_hash() {
        let data = b"hello world";
        let hash = compute_hash(data, Algorithm::Sha512, 0).unwrap();
        // SHA-512 produces 128 hex characters, just check the start
        assert!(hash.starts_with("309ecc489c12d6eb"));
    }

    /// Test bcrypt hash generation and verification.
    #[test]
    fn test_bcrypt_hash() {
        let data = b"password123";
        // Use low cost for fast tests
        let hash = compute_hash(data, Algorithm::Bcrypt, 4).unwrap();
        // Bcrypt hashes start with $2b$ (or $2a$, $2y$)
        assert!(hash.starts_with("$2"));
        // Verify the hash works
        assert!(bcrypt::verify(data, &hash).unwrap());
    }

    /// Test argon2 hash generation and verification.
    #[test]
    fn test_argon2_hash() {
        let data = b"password123";
        let hash = compute_hash(data, Algorithm::Argon2, 0).unwrap();
        // Argon2 hashes start with $argon2
        assert!(hash.starts_with("$argon2"));
        // Verify the hash works
        let parsed = PasswordHash::new(&hash).unwrap();
        assert!(Argon2::default().verify_password(data, &parsed).is_ok());
    }
}
