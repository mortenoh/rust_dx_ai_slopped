# Rustdoc

Generate API documentation from code comments.

## Basic Documentation

```rust
/// Computes the SHA-256 hash of input data.
///
/// # Arguments
///
/// * `data` - The bytes to hash
///
/// # Returns
///
/// A 64-character hexadecimal string
///
/// # Examples
///
/// ```
/// let hash = dx::hash::sha256(b"hello");
/// assert_eq!(hash.len(), 64);
/// ```
pub fn sha256(data: &[u8]) -> String {
    // implementation
}
```

## Module Documentation

```rust
//! # Hash Module
//!
//! This module provides cryptographic hash functions.
//!
//! ## Supported Algorithms
//!
//! - SHA-256
//! - SHA-512
//! - MD5 (legacy)
//!
//! ## Example
//!
//! ```
//! use dx::hash;
//!
//! let digest = hash::sha256(b"data");
//! ```

pub mod sha256;
pub mod sha512;
```

## Common Sections

```rust
/// Brief description (first line).
///
/// Longer description with more details.
///
/// # Arguments
///
/// * `arg1` - Description of first argument
/// * `arg2` - Description of second argument
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Returns `Err` when:
/// - Condition one
/// - Condition two
///
/// # Panics
///
/// Panics if the input is invalid.
///
/// # Safety
///
/// This function is unsafe because...
///
/// # Examples
///
/// ```
/// // Example code here
/// ```
pub fn documented_function() {}
```

## Linking

```rust
/// Uses [`Config`] to load settings.
/// See [`crate::hash::sha256`] for hashing.
/// Related: [`Self::other_method`]
pub fn using_links() {}

/// Link to external docs: [std::fs::read]
pub fn external_link() {}
```

## Attributes

```rust
/// Hidden from docs
#[doc(hidden)]
pub fn internal_function() {}

/// Appears at crate root
#[doc(inline)]
pub use other_module::ImportantType;

/// Custom alias in search
#[doc(alias = "hash")]
pub fn compute_digest() {}
```

## Building Docs

```bash
# Build documentation
cargo doc

# Build and open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items

# All features
cargo doc --all-features
```

## Configuration

In `Cargo.toml`:

```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

In code:
```rust
#[cfg_attr(docsrs, doc(cfg(feature = "advanced")))]
pub fn advanced_feature() {}
```

## Lints

```rust
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
```

## docs.rs Integration

Add badges to README:

```markdown
[![docs.rs](https://docs.rs/dx/badge.svg)](https://docs.rs/dx)
```

Features shown on docs.rs:

```toml
[package.metadata.docs.rs]
features = ["full"]
targets = ["x86_64-unknown-linux-gnu"]
```
