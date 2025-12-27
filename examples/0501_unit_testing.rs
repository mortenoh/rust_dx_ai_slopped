//! # Unit Testing Patterns
//!
//! This example shows unit testing patterns for CLI applications.
//!
//! Run with: `cargo run --example 0501_unit_testing`

#![allow(dead_code)]

fn main() {
    println!("=== Unit Testing Patterns ===\n");

    // =========================================================================
    // TESTING PURE FUNCTIONS
    // =========================================================================

    println!("--- Testing Pure Functions ---");
    println!(
        r#"
Pure functions are easiest to test - no side effects:

// src/utils/hash.rs
pub fn calculate_hash(data: &[u8]) -> String {{
    use sha2::{{Sha256, Digest}};
    let hash = Sha256::digest(data);
    hex::encode(hash)
}}

// Tests
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_hash_empty() {{
        let result = calculate_hash(b"");
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }}

    #[test]
    fn test_hash_hello() {{
        let result = calculate_hash(b"hello");
        assert!(result.starts_with("2cf24dba"));
    }}
}}
"#
    );

    // Demo
    fn calculate_hash(data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let hash = Sha256::digest(data);
        hex::encode(hash)
    }

    println!("  Hash of 'hello': {}", &calculate_hash(b"hello")[..16]);

    println!();

    // =========================================================================
    // TESTING WITH SETUP/TEARDOWN
    // =========================================================================

    println!("--- Setup and Teardown ---");
    println!(
        r#"
Use helper functions for common setup:

#[cfg(test)]
mod tests {{
    struct TestContext {{
        temp_dir: tempfile::TempDir,
        config_path: std::path::PathBuf,
    }}

    impl TestContext {{
        fn new() -> Self {{
            let temp_dir = tempfile::tempdir().unwrap();
            let config_path = temp_dir.path().join("config.toml");
            Self {{ temp_dir, config_path }}
        }}

        fn write_config(&self, content: &str) {{
            std::fs::write(&self.config_path, content).unwrap();
        }}
    }}

    #[test]
    fn test_load_config() {{
        let ctx = TestContext::new();
        ctx.write_config("key = \"value\"");
        // ... test logic ...
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING ERROR CONDITIONS
    // =========================================================================

    println!("--- Testing Error Conditions ---");
    println!(
        r#"
Test both success and error paths:

#[test]
fn test_parse_valid_input() {{
    let result = parse_port("8080");
    assert_eq!(result.unwrap(), 8080);
}}

#[test]
fn test_parse_invalid_input() {{
    let result = parse_port("not_a_number");
    assert!(result.is_err());
}}

#[test]
fn test_parse_out_of_range() {{
    let result = parse_port("99999");
    assert!(matches!(result, Err(ParseError::OutOfRange(_))));
}}

// Use #[should_panic] for expected panics
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_condition() {{
    let v: Vec<i32> = vec![];
    let _ = v[0];
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING WITH MOCKS
    // =========================================================================

    println!("--- Testing with Mocks ---");
    println!(
        r#"
Use traits and mock implementations:

// Define a trait for the behavior
pub trait FileSystem {{
    fn read_file(&self, path: &str) -> std::io::Result<String>;
    fn write_file(&self, path: &str, content: &str) -> std::io::Result<()>;
}}

// Real implementation
pub struct RealFileSystem;
impl FileSystem for RealFileSystem {{
    fn read_file(&self, path: &str) -> std::io::Result<String> {{
        std::fs::read_to_string(path)
    }}
    fn write_file(&self, path: &str, content: &str) -> std::io::Result<()> {{
        std::fs::write(path, content)
    }}
}}

// Mock implementation for tests
#[cfg(test)]
struct MockFileSystem {{
    files: std::collections::HashMap<String, String>,
}}

#[cfg(test)]
impl FileSystem for MockFileSystem {{
    fn read_file(&self, path: &str) -> std::io::Result<String> {{
        self.files.get(path)
            .cloned()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found"
            ))
    }}
    fn write_file(&self, path: &str, content: &str) -> std::io::Result<()> {{
        // Track writes for assertions
        Ok(())
    }}
}}

// Function that uses the trait
pub fn process_config<F: FileSystem>(fs: &F, path: &str) -> Result<Config, Error> {{
    let content = fs.read_file(path)?;
    // ... parse and return config ...
}}

#[test]
fn test_process_config() {{
    let mut fs = MockFileSystem {{ files: HashMap::new() }};
    fs.files.insert("config.toml".into(), "key = 'value'".into());

    let config = process_config(&fs, "config.toml").unwrap();
    assert_eq!(config.key, "value");
}}
"#
    );

    println!();

    // =========================================================================
    // TEST ORGANIZATION
    // =========================================================================

    println!("--- Test Organization ---");
    println!(
        r#"
Organize tests with modules:

#[cfg(test)]
mod tests {{
    use super::*;

    // Group related tests
    mod hash_tests {{
        use super::*;

        #[test]
        fn test_sha256() {{ ... }}

        #[test]
        fn test_md5() {{ ... }}
    }}

    mod encode_tests {{
        use super::*;

        #[test]
        fn test_base64_encode() {{ ... }}

        #[test]
        fn test_base64_decode() {{ ... }}
    }}

    // Helper functions for tests only
    fn create_test_data() -> Vec<u8> {{
        vec![1, 2, 3, 4, 5]
    }}
}}

// Run specific test module:
// cargo test hash_tests

// Run tests matching pattern:
// cargo test test_sha
"#
    );

    println!();

    // =========================================================================
    // TEST ATTRIBUTES
    // =========================================================================

    println!("--- Test Attributes ---");
    println!(
        r#"
Useful test attributes:

#[test]
fn basic_test() {{ }}

#[test]
#[ignore]  // Skip unless --ignored flag
fn slow_test() {{ }}

#[test]
#[should_panic]
fn test_panic() {{ panic!("expected"); }}

#[test]
#[should_panic(expected = "specific message")]
fn test_specific_panic() {{ }}

// Run ignored tests:
// cargo test -- --ignored

// Run all tests including ignored:
// cargo test -- --include-ignored
"#
    );

    println!();

    // =========================================================================
    // ASSERTIONS
    // =========================================================================

    println!("--- Assertion Macros ---");
    println!(
        r#"
Standard assertions:

assert!(condition);
assert!(condition, "custom message");
assert_eq!(left, right);
assert_eq!(left, right, "values: {{:?}} vs {{:?}}", left, right);
assert_ne!(a, b);

// For floating point
assert!((a - b).abs() < f64::EPSILON);

// Pattern matching
assert!(matches!(result, Ok(_)));
assert!(matches!(result, Err(Error::NotFound)));

// With predicates crate (in dev-dependencies)
use predicates::prelude::*;

let predicate = predicate::str::contains("error")
    .and(predicate::str::contains("not found"));
assert!(predicate.eval(&error_message));
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Unit testing patterns:");
    println!("  1. Test pure functions directly");
    println!("  2. Use helper structs for setup/teardown");
    println!("  3. Test both success and error paths");
    println!("  4. Use traits and mocks for dependencies");
    println!("  5. Organize with nested modules");
}
