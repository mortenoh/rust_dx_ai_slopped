//! # Platform Testing
//!
//! This example shows how to test across platforms.
//!
//! Run with: `cargo run --example 0803_platform_testing`

#![allow(dead_code)]

fn main() {
    println!("=== Platform Testing ===\n");

    // =========================================================================
    // CI MATRIX
    // =========================================================================

    println!("--- CI Platform Matrix ---");
    println!(
        r#"
Test on multiple platforms in GitHub Actions:

# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
        exclude:
          # Skip beta on Windows to save CI time
          - os: windows-latest
            rust: beta

    runs-on: ${{{{ matrix.os }}}}

    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{{{ matrix.rust }}}}

      - name: Run tests
        run: cargo test --all-features

      - name: Run clippy
        run: cargo clippy -- -D warnings

  # Platform-specific jobs
  linux-specific:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --features unix-extras

  windows-specific:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --features windows-extras
"#
    );

    println!();

    // =========================================================================
    // PLATFORM-SPECIFIC TESTS
    // =========================================================================

    println!("--- Platform-Specific Tests ---");
    println!(
        r#"
Write tests for specific platforms:

#[cfg(test)]
mod tests {{
    // Unix-only tests
    #[test]
    #[cfg(unix)]
    fn test_unix_permissions() {{
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::NamedTempFile::new().unwrap();
        let perms = std::fs::metadata(temp.path())
            .unwrap()
            .permissions();
        assert!(perms.mode() & 0o600 != 0);
    }}

    #[test]
    #[cfg(unix)]
    fn test_symlink() {{
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("target");
        let link = dir.path().join("link");

        std::fs::write(&target, "content").unwrap();
        std::os::unix::fs::symlink(&target, &link).unwrap();

        assert!(link.is_symlink());
    }}

    // Windows-only tests
    #[test]
    #[cfg(windows)]
    fn test_windows_path() {{
        let path = std::path::Path::new(r"C:\Users\test\file.txt");
        assert!(path.is_absolute());
    }}

    #[test]
    #[cfg(windows)]
    fn test_executable_extension() {{
        let exe = "myapp.exe";
        assert!(exe.ends_with(".exe"));
    }}

    // Cross-platform tests with platform expectations
    #[test]
    fn test_path_separator() {{
        let path = std::path::Path::new("a").join("b").join("c");
        let s = path.to_string_lossy();

        #[cfg(unix)]
        assert_eq!(s, "a/b/c");

        #[cfg(windows)]
        assert_eq!(s, r"a\b\c");
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // MOCKING PLATFORM BEHAVIOR
    // =========================================================================

    println!("--- Mocking Platform Behavior ---");
    println!(
        r#"
Abstract platform behavior for testing:

// Trait for platform-specific operations
pub trait PlatformOps {{
    fn home_dir(&self) -> Option<std::path::PathBuf>;
    fn executable_extension(&self) -> &str;
    fn path_separator(&self) -> char;
}}

// Real implementation
pub struct RealPlatform;

impl PlatformOps for RealPlatform {{
    fn home_dir(&self) -> Option<std::path::PathBuf> {{
        directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf())
    }}

    fn executable_extension(&self) -> &str {{
        std::env::consts::EXE_SUFFIX
    }}

    fn path_separator(&self) -> char {{
        std::path::MAIN_SEPARATOR
    }}
}}

// Mock for testing
#[cfg(test)]
pub struct MockPlatform {{
    pub home: Option<std::path::PathBuf>,
    pub exe_ext: String,
    pub separator: char,
}}

#[cfg(test)]
impl PlatformOps for MockPlatform {{
    fn home_dir(&self) -> Option<std::path::PathBuf> {{
        self.home.clone()
    }}
    fn executable_extension(&self) -> &str {{
        &self.exe_ext
    }}
    fn path_separator(&self) -> char {{
        self.separator
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_with_mock_linux() {{
        let platform = MockPlatform {{
            home: Some("/home/user".into()),
            exe_ext: "".to_string(),
            separator: '/',
        }};

        let config = get_config(&platform);
        assert!(config.path.starts_with("/home/user"));
    }}

    #[test]
    fn test_with_mock_windows() {{
        let platform = MockPlatform {{
            home: Some(r"C:\Users\test".into()),
            exe_ext: ".exe".to_string(),
            separator: '\\',
        }};

        let exe = format!("myapp{{}}", platform.executable_extension());
        assert_eq!(exe, "myapp.exe");
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // OUTPUT NORMALIZATION
    // =========================================================================

    println!("--- Output Normalization ---");
    println!(
        r#"
Normalize output for cross-platform comparison:

#[cfg(test)]
mod tests {{
    fn normalize_output(s: &str) -> String {{
        s.replace("\r\n", "\n")      // Normalize line endings
         .replace('\\', "/")          // Normalize path separators
         .trim()
         .to_string()
    }}

    #[test]
    fn test_output() {{
        let output = run_command();
        let normalized = normalize_output(&output);

        assert_eq!(normalized, "path/to/file\nline 2");
    }}

    // For snapshot testing
    #[test]
    fn test_snapshot() {{
        let output = run_command();
        let normalized = normalize_output(&output);

        insta::with_settings!({{
            // Redact platform-specific paths
            redactions => {{
                "[].path" => "[PATH]",
            }}
        }}, {{
            insta::assert_yaml_snapshot!(normalized);
        }});
    }}
}}

// For paths specifically
fn normalize_path(path: &std::path::Path) -> String {{
    path.to_string_lossy().replace('\\', "/")
}}
"#
    );

    println!();

    // =========================================================================
    // SKIP CONDITIONS
    // =========================================================================

    println!("--- Conditional Test Skipping ---");
    println!(
        r#"
Skip tests under certain conditions:

// Skip on specific platform
#[test]
#[cfg_attr(windows, ignore = "symlinks require admin on Windows")]
fn test_symlinks() {{
    // ...
}}

// Skip in CI
#[test]
#[cfg_attr(
    any(
        target_os = "windows",
        all(target_os = "linux", not(feature = "full-tests"))
    ),
    ignore
)]
fn test_slow_operation() {{
    // ...
}}

// Runtime skip
#[test]
fn test_requires_network() {{
    if std::env::var("CI").is_ok() {{
        eprintln!("Skipping network test in CI");
        return;
    }}

    // ... network test ...
}}

// Check for required tools
#[test]
fn test_requires_docker() {{
    if std::process::Command::new("docker")
        .arg("--version")
        .output()
        .is_err()
    {{
        eprintln!("Docker not available, skipping");
        return;
    }}

    // ... test with docker ...
}}
"#
    );

    println!();

    // =========================================================================
    // TEMP FILES IN TESTS
    // =========================================================================

    println!("--- Temp Files in Tests ---");
    println!(
        r#"
Use tempfile crate for cross-platform temp files:

use tempfile::{{tempdir, NamedTempFile}};

#[test]
fn test_with_temp_file() {{
    // Works on all platforms
    let temp = NamedTempFile::new().unwrap();
    std::fs::write(temp.path(), "content").unwrap();

    let result = process_file(temp.path());
    assert!(result.is_ok());
    // File cleaned up when temp goes out of scope
}}

#[test]
fn test_with_temp_dir() {{
    let dir = tempdir().unwrap();

    // Create test structure
    let sub = dir.path().join("subdir");
    std::fs::create_dir_all(&sub).unwrap();

    let file = sub.join("test.txt");
    std::fs::write(&file, "test").unwrap();

    // Test
    let result = process_directory(dir.path());
    assert!(result.is_ok());
}}
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Platform testing:");
    println!("  1. Use CI matrix for multi-platform testing");
    println!("  2. Use #[cfg] to run platform-specific tests");
    println!("  3. Mock platform behavior for unit tests");
    println!("  4. Normalize output for comparison");
    println!("  5. Use tempfile for cross-platform temp files");
}
