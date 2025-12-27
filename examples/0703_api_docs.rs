//! # API Documentation with rustdoc
//!
//! This example shows how to document library code.
//!
//! Run with: `cargo run --example 0703_api_docs`

#![allow(dead_code)]

fn main() {
    println!("=== API Documentation with rustdoc ===\n");

    // =========================================================================
    // DOCUMENTATION COMMENTS
    // =========================================================================

    println!("--- Documentation Comments ---");
    println!(
        r##"
Three types of doc comments:

/// Outer doc comment (for the following item)
/// Most common for functions, structs, etc.
pub fn my_function() {{}}

//! Inner doc comment (for the enclosing item)
//! Used at top of files for module docs
//! # My Module
//!
//! This module provides...

/** Block doc comment
 * Less common, but valid
 * for longer documentation
 */
pub struct MyStruct;
"##
    );

    println!();

    // =========================================================================
    // FUNCTION DOCUMENTATION
    // =========================================================================

    println!("--- Function Documentation ---");
    println!(
        r##"
Document functions thoroughly:

/// Calculate the SHA-256 hash of the given data.
///
/// # Arguments
///
/// * `data` - The bytes to hash
///
/// # Returns
///
/// A 64-character lowercase hex string representing the hash.
///
/// # Examples
///
/// ```
/// use dx::hash::sha256;
///
/// let hash = sha256(b"hello");
/// assert!(hash.starts_with("2cf24dba"));
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn sha256(data: &[u8]) -> String {{
    // implementation
}}
"##
    );

    println!();

    // =========================================================================
    // STRUCT DOCUMENTATION
    // =========================================================================

    println!("--- Struct Documentation ---");
    println!(
        r##"
Document structs and their fields:

/// Configuration for the hash command.
///
/// # Examples
///
/// ```
/// use dx::config::HashConfig;
///
/// let config = HashConfig::default();
/// assert_eq!(config.algorithm, "sha256");
/// ```
#[derive(Debug, Clone)]
pub struct HashConfig {{
    /// The hashing algorithm to use.
    ///
    /// Valid values: `"md5"`, `"sha256"`, `"sha512"`
    pub algorithm: String,

    /// Output format for the hash.
    ///
    /// Valid values: `"hex"`, `"base64"`
    pub format: String,

    /// Whether to verify against an expected hash.
    pub verify: Option<String>,
}}

impl HashConfig {{
    /// Create a new configuration with default values.
    ///
    /// Uses SHA-256 algorithm with hex output.
    pub fn new() -> Self {{
        Self::default()
    }}

    /// Set the hashing algorithm.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The algorithm name
    ///
    /// # Panics
    ///
    /// Panics if the algorithm is not supported.
    pub fn algorithm(mut self, algorithm: &str) -> Self {{
        self.algorithm = algorithm.to_string();
        self
    }}
}}
"##
    );

    println!();

    // =========================================================================
    // ERROR DOCUMENTATION
    // =========================================================================

    println!("--- Error Documentation ---");
    println!(
        r##"
Document error types:

/// Errors that can occur during hashing operations.
#[derive(Debug, thiserror::Error)]
pub enum HashError {{
    /// The specified file could not be found.
    ///
    /// This error occurs when the path does not exist or is not accessible.
    #[error("File not found: {{0}}")]
    FileNotFound(String),

    /// The file could not be read.
    ///
    /// This may occur due to permission issues or I/O errors.
    #[error("Failed to read file: {{0}}")]
    ReadError(#[from] std::io::Error),

    /// The specified algorithm is not supported.
    ///
    /// Supported algorithms are: md5, sha256, sha512.
    #[error("Unsupported algorithm: {{0}}")]
    UnsupportedAlgorithm(String),

    /// The hash verification failed.
    ///
    /// The computed hash does not match the expected value.
    #[error("Hash mismatch: expected {{expected}}, got {{actual}}")]
    VerificationFailed {{
        /// The expected hash value
        expected: String,
        /// The actual computed hash
        actual: String,
    }},
}}
"##
    );

    println!();

    // =========================================================================
    // MODULE DOCUMENTATION
    // =========================================================================

    println!("--- Module Documentation ---");
    println!(
        r##"
Document modules in lib.rs or mod.rs:

//! # dx - Developer Toolkit
//!
//! A comprehensive CLI toolkit for common developer tasks.
//!
//! ## Features
//!
//! - File hashing (MD5, SHA-256, SHA-512)
//! - Base64/hex encoding and decoding
//! - UUID generation (v4, v7)
//! - Timestamp conversion
//! - JSON formatting and validation
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use dx::{{hash, encode}};
//!
//! // Hash a file
//! let hash = hash::sha256_file("myfile.txt")?;
//! println!("Hash: {{}}", hash);
//!
//! // Encode data
//! let encoded = encode::base64(b"hello");
//! println!("Base64: {{}}", encoded);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Modules
//!
//! - [`hash`] - Cryptographic hashing
//! - [`encode`] - Encoding/decoding
//! - [`uuid`] - UUID generation
//! - [`time`] - Timestamp utilities
//! - [`json`] - JSON operations
"##
    );

    println!();

    // =========================================================================
    // CODE EXAMPLES IN DOCS
    // =========================================================================

    println!("--- Code Examples in Docs ---");
    println!(
        r##"
Doctest annotations:

```rust
// Normal test - compiles and runs
let x = 1 + 1;
assert_eq!(x, 2);
```

```rust,no_run
// Compiles but doesn't run (e.g., needs network)
let response = fetch_url("https://example.com")?;
```

```rust,ignore
// Completely ignored (broken or pseudo-code)
do_something_hypothetical();
```

```rust,should_panic
// Should panic
panic!("This is expected");
```

```rust,compile_fail
// Should NOT compile
let x: i32 = "not a number";
```

```text
// Not Rust code, just text
This is plain text output
```

Hidden lines (for setup):
```rust
# use std::collections::HashMap;
# fn main() {{
let map: HashMap<&str, i32> = HashMap::new();
# }}
```
"##
    );

    println!();

    // =========================================================================
    // BUILDING DOCS
    // =========================================================================

    println!("--- Building Docs ---");
    println!(
        r#"
Generate documentation:

  cargo doc                    # Build docs
  cargo doc --open            # Build and open in browser
  cargo doc --no-deps         # Skip dependency docs
  cargo doc --document-private-items  # Include private items

Doc tests:

  cargo test --doc            # Run only doc tests

Configuration in Cargo.toml:

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]

Conditional docs:

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub async fn async_function() {{}}
"#
    );

    println!();

    // =========================================================================
    // LINKING IN DOCS
    // =========================================================================

    println!("--- Linking in Docs ---");
    println!(
        r##"
Link to other items:

/// See [`HashConfig`] for configuration options.
/// Use [`sha256`] for the most common use case.
/// Related: [`crate::encode::base64`]
pub fn sha512(data: &[u8]) -> String {{ todo!() }}

/// Implements [`std::fmt::Display`].
/// See the [`hash`](crate::hash) module.
pub struct MyType;

Link syntax:
- [`Type`] - Link to type in scope
- [`module::Type`] - Qualified path
- [`crate::module`] - From crate root
- [`Self::method`] - Method on current type
- [`Type::method()`] - Method with parens
- [text][`Type`] - Custom link text
"##
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("API documentation with rustdoc:");
    println!("  1. Use /// for public items");
    println!("  2. Include Examples, Arguments, Returns sections");
    println!("  3. Document errors with # Errors");
    println!("  4. Use doc annotations (no_run, ignore, etc.)");
    println!("  5. cargo doc --open to preview");
}
