//! # Error Path Testing
//!
//! This example shows how to test error handling paths.
//!
//! Run with: `cargo run --example 0602_error_testing`

#![allow(dead_code)]

fn main() {
    println!("=== Error Path Testing ===\n");

    // =========================================================================
    // TESTING RESULT TYPES
    // =========================================================================

    println!("--- Testing Result Types ---");
    println!(
        r#"
Test both Ok and Err paths:

#[test]
fn test_parse_valid() {{
    let result = parse_port("8080");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 8080);
}}

#[test]
fn test_parse_invalid() {{
    let result = parse_port("not_a_number");
    assert!(result.is_err());
}}

// More specific error checks
#[test]
fn test_specific_error() {{
    let result = parse_port("99999");
    match result {{
        Err(ParseError::OutOfRange(n)) => assert_eq!(n, 99999),
        other => panic!("Expected OutOfRange, got {{:?}}", other),
    }}
}}

// Using matches! macro
#[test]
fn test_error_variant() {{
    let result = read_config("/nonexistent");
    assert!(matches!(result, Err(ConfigError::NotFound(_))));
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING ERROR MESSAGES
    // =========================================================================

    println!("--- Testing Error Messages ---");
    println!(
        r#"
Verify error messages are helpful:

#[test]
fn test_error_message_contains_path() {{
    let path = "/some/missing/file.txt";
    let result = read_file(path);

    let err = result.unwrap_err();
    let msg = err.to_string();

    assert!(
        msg.contains(path),
        "Error message should contain the path: {{}}", msg
    );
}}

#[test]
fn test_error_message_is_actionable() {{
    let result = connect_to_server("invalid://url");
    let err = result.unwrap_err();
    let msg = err.to_string();

    // Check message helps user fix the problem
    assert!(
        msg.contains("http://") || msg.contains("https://"),
        "Error should suggest valid URL schemes: {{}}", msg
    );
}}

// Snapshot test error messages
#[test]
fn test_error_messages_snapshot() {{
    let errors = vec![
        read_file("/missing").unwrap_err(),
        parse_port("-1").unwrap_err(),
        connect_to_server("").unwrap_err(),
    ];

    for (i, err) in errors.iter().enumerate() {{
        insta::assert_snapshot!(
            format!("error_message_{{}}", i),
            err.to_string()
        );
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING ERROR CHAINS
    // =========================================================================

    println!("--- Testing Error Chains ---");
    println!(
        r#"
Test error source chains (with thiserror/anyhow):

use std::error::Error;

#[test]
fn test_error_chain() {{
    let result = load_config("/bad/path");
    let err = result.unwrap_err();

    // Check immediate error
    assert!(err.to_string().contains("config"));

    // Check the cause chain
    let mut source = err.source();
    let mut found_io_error = false;

    while let Some(err) = source {{
        if err.to_string().contains("No such file") {{
            found_io_error = true;
            break;
        }}
        source = err.source();
    }}

    assert!(found_io_error, "Should have IO error in chain");
}}

// With anyhow, check the chain
#[test]
fn test_anyhow_chain() {{
    let result: anyhow::Result<()> = load_something();
    let err = result.unwrap_err();

    // Iterate the chain
    for cause in err.chain() {{
        println!("Caused by: {{}}", cause);
    }}

    // Check root cause
    let root = err.root_cause();
    assert!(root.to_string().contains("connection refused"));
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING CLI ERRORS
    // =========================================================================

    println!("--- Testing CLI Errors ---");
    println!(
        r#"
Test CLI error output:

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_missing_required_arg() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash"]);  // Missing file argument

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}}

#[test]
fn test_invalid_option_value() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "-a", "invalid_algo", "file.txt"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid")
            .or(predicate::str::contains("possible values")));
}}

#[test]
fn test_file_not_found() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "/nonexistent/file.txt"]);

    cmd.assert()
        .failure()
        .code(1)  // Check exit code
        .stderr(predicate::str::contains("not found")
            .or(predicate::str::contains("No such file")));
}}

#[test]
fn test_error_goes_to_stderr() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "/missing"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::is_empty())  // No stdout on error
        .stderr(predicate::str::is_empty().not());  // Error on stderr
}}
"#
    );

    println!();

    // =========================================================================
    // TESTING PANICS
    // =========================================================================

    println!("--- Testing Panics ---");
    println!(
        r#"
Test panic conditions (use sparingly):

#[test]
#[should_panic]
fn test_panics_on_invalid_state() {{
    let invalid = InvalidState::new();
    invalid.do_something();  // Should panic
}}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_message() {{
    let v: Vec<i32> = vec![];
    let _ = v[0];
}}

// Using std::panic::catch_unwind for more control
#[test]
fn test_panic_recovery() {{
    let result = std::panic::catch_unwind(|| {{
        dangerous_operation()
    }});

    assert!(result.is_err());

    // Check panic message
    if let Err(panic) = result {{
        if let Some(msg) = panic.downcast_ref::<&str>() {{
            assert!(msg.contains("expected"));
        }}
    }}
}}

// Prefer Result over panics in library code!
"#
    );

    println!();

    // =========================================================================
    // ERROR SIMULATION
    // =========================================================================

    println!("--- Error Simulation ---");
    println!(
        r#"
Simulate errors for testing:

// Trait-based error injection
pub trait Storage {{
    fn read(&self, key: &str) -> Result<String, StorageError>;
    fn write(&self, key: &str, value: &str) -> Result<(), StorageError>;
}}

#[cfg(test)]
struct FailingStorage {{
    fail_on: Option<String>,
    error: StorageError,
}}

#[cfg(test)]
impl Storage for FailingStorage {{
    fn read(&self, key: &str) -> Result<String, StorageError> {{
        if self.fail_on.as_deref() == Some(key) {{
            Err(self.error.clone())
        }} else {{
            Ok("value".to_string())
        }}
    }}
    // ...
}}

#[test]
fn test_handles_storage_failure() {{
    let storage = FailingStorage {{
        fail_on: Some("config".to_string()),
        error: StorageError::ConnectionLost,
    }};

    let app = App::new(storage);
    let result = app.load_config();

    assert!(result.is_err());
    // Check graceful degradation, retry logic, etc.
}}
"#
    );

    println!();

    // =========================================================================
    // EXIT CODES
    // =========================================================================

    println!("--- Exit Code Testing ---");
    println!(
        r#"
Test CLI exit codes:

use assert_cmd::Command;

#[test]
fn test_success_exit_code() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["--version"]);
    cmd.assert().code(0);
}}

#[test]
fn test_error_exit_code() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "/missing"]);
    cmd.assert().code(1);  // General error
}}

#[test]
fn test_usage_error_exit_code() {{
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["--invalid-flag"]);
    cmd.assert().code(2);  // Usage error (clap default)
}}

// Common exit codes:
// 0 - Success
// 1 - General error
// 2 - Usage/argument error
// 126 - Command not executable
// 127 - Command not found
// 130 - Interrupted (Ctrl+C)
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Error path testing:");
    println!("  1. Test both Ok and Err paths");
    println!("  2. Verify error messages are helpful");
    println!("  3. Test error chains with .source()");
    println!("  4. Use predicates for CLI error output");
    println!("  5. Test exit codes for proper signaling");
}
