//! # Cross-Platform Test Handling
//!
//! This example shows how to handle platform differences in tests.
//!
//! Run with: `cargo run --example 0603_cross_platform_tests`

#![allow(dead_code)]

fn main() {
    println!("=== Cross-Platform Test Handling ===\n");

    // =========================================================================
    // PLATFORM-SPECIFIC TESTS
    // =========================================================================

    println!("--- Platform-Specific Tests ---");
    println!(
        r#"
Skip or customize tests per platform:

// Run only on Unix
#[test]
#[cfg(unix)]
fn test_unix_permissions() {{
    use std::os::unix::fs::PermissionsExt;
    // ...
}}

// Run only on Windows
#[test]
#[cfg(windows)]
fn test_windows_registry() {{
    // ...
}}

// Run only on macOS
#[test]
#[cfg(target_os = "macos")]
fn test_macos_keychain() {{
    // ...
}}

// Skip on specific platform
#[test]
#[cfg(not(windows))]
fn test_symlinks() {{
    // Symlinks can be tricky on Windows
}}
"#
    );

    println!();

    // =========================================================================
    // RUNTIME PLATFORM CHECKS
    // =========================================================================

    println!("--- Runtime Platform Checks ---");
    println!(
        r#"
Check platform at runtime for conditional logic:

#[test]
fn test_platform_behavior() {{
    let expected = if cfg!(windows) {{
        "C:\\Users\\..."
    }} else {{
        "/home/..."
    }};

    let home = get_home_dir();
    assert!(home.starts_with(expected.split("...").next().unwrap()));
}}

#[test]
fn test_line_endings() {{
    let content = read_file("test.txt");

    if cfg!(windows) {{
        // Windows might have CRLF
        assert!(content.contains('\n'));
    }} else {{
        // Unix should have LF only
        assert!(!content.contains("\r\n"));
    }}
}}

#[test]
fn test_path_separators() {{
    let path = build_path(&["foo", "bar", "baz"]);

    if cfg!(windows) {{
        assert!(path.contains('\\'));
    }} else {{
        assert!(path.contains('/'));
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // NORMALIZING OUTPUT
    // =========================================================================

    println!("--- Normalizing Output ---");
    println!(
        r#"
Normalize platform differences for comparison:

fn normalize_path(path: &str) -> String {{
    path.replace('\\', "/")
}}

fn normalize_newlines(text: &str) -> String {{
    text.replace("\r\n", "\n")
}}

#[test]
fn test_output_normalized() {{
    let output = run_command();
    let normalized = normalize_newlines(&output);

    assert_eq!(normalized, "line1\nline2\nline3\n");
}}

#[test]
fn test_path_output() {{
    let output = get_path_output();
    let normalized = normalize_path(&output);

    assert_eq!(normalized, "foo/bar/baz");
}}

// For snapshots, normalize before comparing
#[test]
fn test_snapshot_normalized() {{
    let output = run_command();
    let normalized = normalize_newlines(&output);
    insta::assert_snapshot!(normalized);
}}
"#
    );

    println!();

    // =========================================================================
    // TEMP FILE HANDLING
    // =========================================================================

    println!("--- Temp File Handling ---");
    println!(
        r#"
tempfile crate handles platform differences:

use tempfile::{{tempdir, NamedTempFile}};

#[test]
fn test_with_temp_dir() {{
    let dir = tempdir().unwrap();
    // Works on all platforms

    // Path handling
    let file = dir.path().join("subdir").join("file.txt");
    // Correct separator on each platform

    // Create parent dirs
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "content").unwrap();

    assert!(file.exists());
}}

// Named temp files
#[test]
fn test_with_temp_file() {{
    let file = NamedTempFile::new().unwrap();
    // File is in OS temp directory
    // Works on Windows, macOS, Linux
}}
"#
    );

    println!();

    // =========================================================================
    // EXECUTABLE EXTENSION
    // =========================================================================

    println!("--- Executable Extension ---");
    println!(
        r#"
Handle .exe extension on Windows:

const EXE_SUFFIX: &str = if cfg!(windows) {{ ".exe" }} else {{ "" }};

#[test]
fn test_find_executable() {{
    let name = format!("myapp{{}}", EXE_SUFFIX);
    let path = find_in_path(&name);
    assert!(path.is_some());
}}

// Or use std::env::consts
use std::env::consts::EXE_SUFFIX;

#[test]
fn test_executable_name() {{
    let exe = format!("cargo{{}}", EXE_SUFFIX);
    // "cargo" on Unix, "cargo.exe" on Windows
}}
"#
    );

    println!();

    // =========================================================================
    // ENVIRONMENT VARIABLE TESTS
    // =========================================================================

    println!("--- Environment Variable Tests ---");
    println!(
        r#"
Environment differences:

#[test]
fn test_home_dir() {{
    let home = if cfg!(windows) {{
        std::env::var("USERPROFILE")
    }} else {{
        std::env::var("HOME")
    }};

    assert!(home.is_ok());
}}

// Use directories crate for cross-platform paths
use directories::ProjectDirs;

#[test]
fn test_config_dir() {{
    let dirs = ProjectDirs::from("com", "example", "myapp").unwrap();
    let config = dirs.config_dir();

    // Correct location on each platform:
    // Linux: ~/.config/myapp
    // macOS: ~/Library/Application Support/com.example.myapp
    // Windows: C:\Users\X\AppData\Roaming\example\myapp

    assert!(config.to_str().is_some());
}}
"#
    );

    println!();

    // =========================================================================
    // CI PLATFORM MATRIX
    // =========================================================================

    println!("--- CI Platform Matrix ---");
    println!(
        r#"
Test on multiple platforms in CI:

# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]

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

Some tests may need platform-specific handling in CI:

      - name: Test with sudo (Linux only)
        if: runner.os == 'Linux'
        run: sudo cargo test --test privileged_tests

      - name: Test registry (Windows only)
        if: runner.os == 'Windows'
        run: cargo test --test registry_tests
"#
    );

    println!();

    // =========================================================================
    // HANDLING FLAKY TESTS
    // =========================================================================

    println!("--- Handling Flaky Platform Tests ---");
    println!(
        r#"
Some tests are flaky on certain platforms:

// Skip if known flaky
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn test_flaky_on_windows() {{
    // This test has timing issues on Windows CI
}}

// Add retries for flaky tests
#[test]
fn test_network_operation() {{
    let mut attempts = 0;
    let max_attempts = 3;

    loop {{
        attempts += 1;
        match try_network_test() {{
            Ok(_) => break,
            Err(e) if attempts < max_attempts => {{
                eprintln!("Attempt {{}} failed: {{}}, retrying...", attempts, e);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }}
            Err(e) => panic!("Test failed after {{}} attempts: {{}}", attempts, e),
        }}
    }}
}}
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Cross-platform testing:");
    println!("  1. Use #[cfg] for platform-specific tests");
    println!("  2. Normalize paths and newlines for comparison");
    println!("  3. Use tempfile for cross-platform temp files");
    println!("  4. Test on matrix of OS/Rust versions in CI");
    println!("  5. Handle executable extensions (.exe)");
}
