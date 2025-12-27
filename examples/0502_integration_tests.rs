//! # Integration Testing with assert_cmd
//!
//! This example shows how to test CLI applications end-to-end.
//!
//! Run with: `cargo run --example 0502_integration_tests`

#![allow(dead_code)]

fn main() {
    println!("=== Integration Testing with assert_cmd ===\n");

    // =========================================================================
    // BASIC CLI TESTING
    // =========================================================================

    println!("--- Basic CLI Testing ---");
    println!(
        r#"
Tests go in tests/ directory (not examples/):

// tests/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_flag() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Developer toolkit"));
}}

#[test]
fn test_version_flag() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING SUBCOMMANDS
    // =========================================================================

    println!("--- Testing Subcommands ---");
    println!(
        r#"
Test each subcommand:

#[test]
fn test_hash_command() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "-a", "sha256", "-"])
        .write_stdin("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2cf24dba"));
}}

#[test]
fn test_encode_base64() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["encode", "base64", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8="));
}}

#[test]
fn test_uuid_generation() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["uuid", "-t", "v4"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(
            r"[0-9a-f]{{8}}-[0-9a-f]{{4}}-4[0-9a-f]{{3}}-[89ab][0-9a-f]{{3}}-[0-9a-f]{{12}}"
        ).unwrap());
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING WITH FILES
    // =========================================================================

    println!("--- Testing with Files ---");
    println!(
        r##"
Use tempfile for file-based tests:

use tempfile::{{tempdir, NamedTempFile}};
use std::io::Write;

#[test]
fn test_hash_file() {{
    // Create a temp file with known content
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "test content").unwrap();

    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", file.path().to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sha256"));
}}

#[test]
fn test_json_format_file() {{
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.json");
    let output = dir.path().join("output.json");

    std::fs::write(&input, r#"{{"a":1,"b":2}}"#).unwrap();

    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args([
        "json", "format",
        input.to_str().unwrap(),
        "-o", output.to_str().unwrap()
    ]);
    cmd.assert().success();

    let formatted = std::fs::read_to_string(&output).unwrap();
    assert!(formatted.contains("  \"a\": 1"));
}}
"##
    );

    println!();

    // =========================================================================
    // TESTING STDIN/STDOUT
    // =========================================================================

    println!("--- Testing stdin/stdout ---");
    println!(
        r#"
Test piped input and output:

#[test]
fn test_stdin_pipe() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["encode", "base64", "-"])
        .write_stdin("hello world");
    cmd.assert()
        .success()
        .stdout("aGVsbG8gd29ybGQ=\n");
}}

#[test]
fn test_decode_pipe() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["encode", "base64", "-d", "-"])
        .write_stdin("aGVsbG8=");
    cmd.assert()
        .success()
        .stdout("hello");
}}

// Test binary output
#[test]
fn test_binary_output() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["encode", "hex", "-d", "-"])
        .write_stdin("48454c4c4f");
    cmd.assert()
        .success()
        .stdout(predicates::eq(b"HELLO".as_slice()));
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING ERROR CASES
    // =========================================================================

    println!("--- Testing Error Cases ---");
    println!(
        r#"
Test error handling:

#[test]
fn test_missing_file() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "/nonexistent/file"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found")
            .or(predicate::str::contains("No such file")));
}}

#[test]
fn test_invalid_algorithm() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "-a", "invalid", "-"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));
}}

#[test]
fn test_invalid_json() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["json", "validate", "-"])
        .write_stdin("{{not valid json}}");
    cmd.assert()
        .failure();
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING ENVIRONMENT VARIABLES
    // =========================================================================

    println!("--- Testing Environment Variables ---");
    println!(
        r#"
Test env var handling:

#[test]
fn test_env_override() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.env("DX_DEFAULT_ALGORITHM", "md5")
        .args(["hash", "-"])
        .write_stdin("test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("md5"));
}}

#[test]
fn test_no_color_env() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.env("NO_COLOR", "1")
        .args(["--help"]);
    // Output should not contain ANSI escape codes
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\x1b[").not());
}}

#[test]
fn test_config_dir_env() {{
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.env("DX_CONFIG_DIR", dir.path())
        .args(["config", "path"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(dir.path().to_str().unwrap()));
}}
"#
    );

    println!();

    // =========================================================================
    // PREDICATES
    // =========================================================================

    println!("--- Predicates ---");
    println!(
        r#"
Common predicates for assertions:

use predicates::prelude::*;

// String predicates
predicate::str::contains("substring")
predicate::str::starts_with("prefix")
predicate::str::ends_with("suffix")
predicate::str::is_empty()
predicate::str::is_match(r"regex pattern").unwrap()

// Combining predicates
predicate::str::contains("error")
    .and(predicate::str::contains("file"))

predicate::str::contains("success")
    .or(predicate::str::contains("ok"))

predicate::str::contains("secret").not()

// Numeric predicates
predicate::eq(42)
predicate::ne(0)
predicate::gt(10)
predicate::lt(100)

// File predicates
predicate::path::exists()
predicate::path::is_file()
predicate::path::is_dir()
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Integration testing with assert_cmd:");
    println!("  1. Test CLI with Command::cargo_bin()");
    println!("  2. Use .args() for arguments");
    println!("  3. Use .write_stdin() for piped input");
    println!("  4. Use .assert() with predicates");
    println!("  5. Test both success and failure cases");
}
