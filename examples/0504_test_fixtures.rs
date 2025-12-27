//! # Test Fixtures with tempfile
//!
//! This example shows how to manage test fixtures and temp files.
//!
//! Run with: `cargo run --example 0504_test_fixtures`

#![allow(dead_code)]

use std::io::Write;
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir, tempdir};

fn main() {
    println!("=== Test Fixtures with tempfile ===\n");

    // =========================================================================
    // TEMPORARY DIRECTORIES
    // =========================================================================

    println!("--- Temporary Directories ---");
    {
        let dir = tempdir().unwrap();
        println!("  Created temp dir: {:?}", dir.path());

        // Create files in temp dir
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "test content").unwrap();
        println!("  Created file: {:?}", file_path);

        // Dir is automatically deleted when `dir` goes out of scope
    }
    println!("  (temp dir cleaned up automatically)");

    println!();

    // =========================================================================
    // TEMPORARY FILES
    // =========================================================================

    println!("--- Temporary Files ---");
    {
        // Named temp file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "line 1").unwrap();
        writeln!(file, "line 2").unwrap();

        println!("  Temp file path: {:?}", file.path());

        // Read it back
        let content = std::fs::read_to_string(file.path()).unwrap();
        println!("  Content: {:?}", content.trim());
    }

    println!();

    // =========================================================================
    // FIXTURE PATTERNS
    // =========================================================================

    println!("--- Fixture Patterns ---");
    println!(
        r##"
Pattern 1: Test Context Struct

struct TestContext {{
    dir: TempDir,
    config_path: PathBuf,
    data_path: PathBuf,
}}

impl TestContext {{
    fn new() -> Self {{
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");
        let data_path = dir.path().join("data");
        std::fs::create_dir_all(&data_path).unwrap();

        Self {{ dir, config_path, data_path }}
    }}

    fn write_config(&self, content: &str) {{
        std::fs::write(&self.config_path, content).unwrap();
    }}

    fn write_data(&self, name: &str, content: &str) {{
        std::fs::write(self.data_path.join(name), content).unwrap();
    }}
}}

#[test]
fn test_with_context() {{
    let ctx = TestContext::new();
    ctx.write_config(r#"key = "value""#);
    ctx.write_data("input.json", r#"{{"a": 1}}"#);

    // Run test with files...
}}
"##
    );

    // Demo
    struct TestContext {
        #[allow(dead_code)]
        dir: TempDir,
        config_path: PathBuf,
    }

    impl TestContext {
        fn new() -> Self {
            let dir = tempdir().unwrap();
            let config_path = dir.path().join("config.toml");
            Self { dir, config_path }
        }

        fn write_config(&self, content: &str) {
            std::fs::write(&self.config_path, content).unwrap();
        }
    }

    let ctx = TestContext::new();
    ctx.write_config("key = 'value'");
    println!("  Demo context created: {:?}", ctx.config_path);

    println!();

    // =========================================================================
    // STATIC FIXTURES
    // =========================================================================

    println!("--- Static Fixtures ---");
    println!(
        r##"
Pattern 2: Static Test Data

// In tests/fixtures/mod.rs
pub const VALID_JSON: &str = r#"{{"name": "test", "value": 42}}"#;
pub const INVALID_JSON: &str = r#"{{not valid}}"#;
pub const SAMPLE_CONFIG: &str = r#"
[server]
host = "localhost"
port = 8080
"#;

// Include files from disk
pub const LARGE_JSON: &str = include_str!("fixtures/large.json");
pub const BINARY_DATA: &[u8] = include_bytes!("fixtures/test.bin");

// In tests
use crate::fixtures;

#[test]
fn test_parse_valid() {{
    let result = parse_json(fixtures::VALID_JSON);
    assert!(result.is_ok());
}}
"##
    );

    println!();

    // =========================================================================
    // FIXTURE GENERATORS
    // =========================================================================

    println!("--- Fixture Generators ---");
    println!(
        r#"
Pattern 3: Generate Test Data

fn generate_test_files(dir: &Path, count: usize) {{
    for i in 0..count {{
        let path = dir.join(format!("file_{{}}.txt", i));
        let content = format!("Content for file {{}}", i);
        std::fs::write(path, content).unwrap();
    }}
}}

fn generate_json_file(path: &Path, records: usize) {{
    let data: Vec<_> = (0..records)
        .map(|i| serde_json::json!({{
            "id": i,
            "name": format!("Record {{}}", i),
            "active": i % 2 == 0,
        }}))
        .collect();

    let json = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write(path, json).unwrap();
}}

#[test]
fn test_batch_processing() {{
    let dir = tempdir().unwrap();
    generate_test_files(dir.path(), 100);

    let result = process_directory(dir.path());
    assert_eq!(result.processed, 100);
}}
"#
    );

    println!();

    // =========================================================================
    // CLEANUP PATTERNS
    // =========================================================================

    println!("--- Cleanup Patterns ---");
    println!(
        r#"
Automatic cleanup (preferred):

  let dir = tempdir().unwrap();  // Deleted when dropped

Keep temp files for debugging:

  let dir = tempdir().unwrap();
  // ... test fails ...
  let path = dir.into_path();  // Prevents cleanup
  println!("Debug files at: {{:?}}", path);

Explicit cleanup:

  let dir = tempdir().unwrap();
  let path = dir.path().to_path_buf();
  // ... use path ...
  drop(dir);  // Explicitly cleanup
  assert!(!path.exists());

Persist on failure:

  let dir = tempdir().unwrap();
  let result = run_test(&dir);

  if result.is_err() {{
      let kept = dir.into_path();
      eprintln!("Test failed. Files at: {{:?}}", kept);
  }}
  // Otherwise, dir is dropped and cleaned up
"#
    );

    println!();

    // =========================================================================
    // CROSS-PLATFORM CONSIDERATIONS
    // =========================================================================

    println!("--- Cross-Platform Considerations ---");
    println!(
        r#"
tempfile handles platform differences:

  // Works on all platforms
  let dir = tempdir().unwrap();

  // Uses correct path separators
  let file = dir.path().join("subdir").join("file.txt");

  // Creates parent directories
  std::fs::create_dir_all(file.parent().unwrap()).unwrap();

  // Line endings (be careful with text comparisons!)
  #[cfg(windows)]
  const NEWLINE: &str = "\r\n";
  #[cfg(not(windows))]
  const NEWLINE: &str = "\n";

  // Or use normalize functions
  fn normalize_newlines(s: &str) -> String {{
      s.replace("\r\n", "\n")
  }}
"#
    );

    println!();

    // =========================================================================
    // SHARED FIXTURES
    // =========================================================================

    println!("--- Shared Fixtures ---");
    println!(
        r#"
Share expensive fixtures across tests:

use std::sync::OnceLock;

static SHARED_FIXTURE: OnceLock<SharedFixture> = OnceLock::new();

struct SharedFixture {{
    dir: TempDir,
    // Large pre-computed data...
}}

impl SharedFixture {{
    fn get() -> &'static Self {{
        SHARED_FIXTURE.get_or_init(|| {{
            let dir = tempdir().unwrap();
            // Expensive setup...
            Self {{ dir }}
        }})
    }}
}}

#[test]
fn test_using_shared() {{
    let fixture = SharedFixture::get();
    // Use fixture.dir.path()...
}}

// Note: Shared fixtures are NOT cleaned up until process exit
// Use for read-only fixtures only
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Test fixture patterns:");
    println!("  1. Use tempdir() for isolated test directories");
    println!("  2. Create TestContext structs for complex setup");
    println!("  3. Use include_str!/include_bytes! for static data");
    println!("  4. Generate fixtures for large datasets");
    println!("  5. Fixtures auto-cleanup when dropped");
}
