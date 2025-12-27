//! # Snapshot Testing with Insta
//!
//! This example shows how to use insta for snapshot testing.
//!
//! Run with: `cargo run --example 0503_snapshot_testing`

#![allow(dead_code)]

fn main() {
    println!("=== Snapshot Testing with Insta ===\n");

    // =========================================================================
    // WHAT IS SNAPSHOT TESTING?
    // =========================================================================

    println!("--- What is Snapshot Testing? ---");
    println!(
        r#"
Snapshot testing captures output and compares against stored "snapshots".
Perfect for:
  - CLI output formatting
  - Error messages
  - Complex data structures
  - Anything that's tedious to manually assert

When output changes:
  1. Test fails with a diff
  2. Review the diff (cargo insta review)
  3. Accept if correct, fix if bug
"#
    );

    println!();

    // =========================================================================
    // BASIC SNAPSHOTS
    // =========================================================================

    println!("--- Basic Snapshots ---");
    println!(
        r#"
Add insta to dev-dependencies:
  [dev-dependencies]
  insta = {{ version = "1", features = ["yaml"] }}

Basic usage:

use insta::assert_snapshot;

#[test]
fn test_help_output() {{
    let output = get_help_text();
    assert_snapshot!(output);
}}

// Snapshot saved to: snapshots/module__test_help_output.snap
// Contents:
// ---
// source: tests/cli_tests.rs
// expression: output
// ---
// dx - Developer toolkit
//
// Usage: dx <COMMAND>
// ...
"#
    );

    println!();

    // =========================================================================
    // NAMED SNAPSHOTS
    // =========================================================================

    println!("--- Named Snapshots ---");
    println!(
        r#"
Give snapshots meaningful names:

#[test]
fn test_error_messages() {{
    // Multiple snapshots in one test
    assert_snapshot!("file_not_found", get_error("missing.txt"));
    assert_snapshot!("permission_denied", get_error("/root/secret"));
    assert_snapshot!("invalid_format", get_error("bad.json"));
}}

// Creates:
//   snapshots/module__file_not_found.snap
//   snapshots/module__permission_denied.snap
//   snapshots/module__invalid_format.snap
"#
    );

    println!();

    // =========================================================================
    // INLINE SNAPSHOTS
    // =========================================================================

    println!("--- Inline Snapshots ---");
    println!(
        r##"
Keep snapshots in the test file:

#[test]
fn test_uuid_format() {{
    let uuid = generate_uuid_v4();
    // Empty string gets filled in by `cargo insta review`
    assert_snapshot!(uuid, @"");
}}

// After running `cargo insta review`:
#[test]
fn test_uuid_format() {{
    let uuid = generate_uuid_v4();
    assert_snapshot!(uuid, @"550e8400-e29b-41d4-a716-446655440000");
}}

// Great for small, stable outputs
"##
    );

    println!();

    // =========================================================================
    // YAML SNAPSHOTS
    // =========================================================================

    println!("--- YAML Snapshots ---");
    println!(
        r#"
Serialize complex types as YAML:

use insta::assert_yaml_snapshot;

#[derive(Debug, Serialize)]
struct Config {{
    name: String,
    port: u16,
    features: Vec<String>,
}}

#[test]
fn test_default_config() {{
    let config = Config::default();
    assert_yaml_snapshot!(config);
}}

// Snapshot:
// ---
// source: tests/config_tests.rs
// expression: config
// ---
// name: myapp
// port: 8080
// features:
//   - logging
//   - metrics
"#
    );

    println!();

    // =========================================================================
    // JSON SNAPSHOTS
    // =========================================================================

    println!("--- JSON Snapshots ---");
    println!(
        r#"
For JSON output:

use insta::assert_json_snapshot;

#[test]
fn test_json_output() {{
    let result = process_data();
    assert_json_snapshot!(result);
}}

// Or with custom settings:
use insta::{{with_settings, assert_json_snapshot}};

#[test]
fn test_with_settings() {{
    with_settings!({{
        sort_maps => true,
        redactions => {{
            "[].timestamp" => "[timestamp]",
            "[].id" => "[uuid]",
        }}
    }}, {{
        assert_json_snapshot!(get_records());
    }});
}}
"#
    );

    println!();

    // =========================================================================
    // REDACTIONS
    // =========================================================================

    println!("--- Redactions ---");
    println!(
        r#"
Redact dynamic values:

use insta::{{assert_yaml_snapshot, with_settings}};

#[test]
fn test_with_redactions() {{
    let output = run_command();

    with_settings!({{
        redactions => {{
            // Redact specific paths
            ".timestamp" => "[timestamp]",
            ".uuid" => "[uuid]",
            ".duration_ms" => "[duration]",

            // Redact in arrays
            "[].created_at" => "[time]",

            // Regex redaction
            ".version" => insta::dynamic_redaction(|value, _| {{
                if value.as_str().map(|s| s.starts_with("1.")).unwrap_or(false) {{
                    "[version 1.x]".into()
                }} else {{
                    value
                }}
            }}),
        }}
    }}, {{
        assert_yaml_snapshot!(output);
    }});
}}

// Output in snapshot:
// timestamp: "[timestamp]"
// uuid: "[uuid]"
// version: "[version 1.x]"
"#
    );

    println!();

    // =========================================================================
    // CLI INTEGRATION
    // =========================================================================

    println!("--- CLI Integration ---");
    println!(
        r#"
Combine with assert_cmd:

use assert_cmd::Command;
use insta::assert_snapshot;

#[test]
fn test_help_snapshot() {{
    let output = Command::cargo_bin("dx")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_snapshot!(stdout);
}}

#[test]
fn test_hash_output_snapshot() {{
    let output = Command::cargo_bin("dx")
        .unwrap()
        .args(["hash", "-a", "sha256", "-"])
        .write_stdin("hello")
        .output()
        .unwrap();

    assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}}
"#
    );

    println!();

    // =========================================================================
    // WORKFLOW
    // =========================================================================

    println!("--- Snapshot Workflow ---");
    println!(
        r#"
Typical workflow:

1. WRITE TEST (snapshot empty or outdated)
   cargo test
   → Test fails, shows diff

2. REVIEW CHANGES
   cargo insta review
   → Interactive review of all pending snapshots
   → Accept (a), Reject (r), Skip (s)

3. ACCEPT/REJECT
   - Accept: Snapshot file updated
   - Reject: Keeps old snapshot, fix your code

Alternative commands:
  cargo insta test           # Run tests, review pending
  cargo insta test --review  # Run tests then auto-open review
  cargo insta accept         # Accept all pending without review
  cargo insta reject         # Reject all pending
"#
    );

    println!();

    // =========================================================================
    // BEST PRACTICES
    // =========================================================================

    println!("--- Best Practices ---");
    println!(
        r#"
1. COMMIT SNAPSHOTS to version control
   - They're test fixtures
   - Changes show in PRs for review

2. USE REDACTIONS for dynamic values
   - Timestamps, UUIDs, durations
   - Prevents flaky tests

3. NAME SNAPSHOTS clearly
   - Especially with multiple in one test
   - Makes diffs easier to understand

4. REVIEW CAREFULLY before accepting
   - Don't just accept-all blindly
   - Changes might indicate bugs

5. KEEP SNAPSHOTS SMALL
   - Large snapshots are hard to review
   - Split into multiple focused tests
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Snapshot testing with insta:");
    println!("  1. Capture complex output automatically");
    println!("  2. Use assert_snapshot! for strings");
    println!("  3. Use assert_yaml_snapshot! for structs");
    println!("  4. Redact dynamic values (timestamps, IDs)");
    println!("  5. Review with `cargo insta review`");
}
