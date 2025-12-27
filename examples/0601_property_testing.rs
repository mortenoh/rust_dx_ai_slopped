//! # Property Testing with proptest
//!
//! This example shows property-based testing for edge case discovery.
//!
//! Run with: `cargo run --example 0601_property_testing`

#![allow(dead_code)]

fn main() {
    println!("=== Property Testing with proptest ===\n");

    // =========================================================================
    // WHAT IS PROPERTY TESTING?
    // =========================================================================

    println!("--- What is Property Testing? ---");
    println!(
        r#"
Property testing generates random inputs to find edge cases.

Instead of:
  #[test]
  fn test_reverse() {{
      assert_eq!(reverse("hello"), "olleh");
      assert_eq!(reverse(""), "");
      assert_eq!(reverse("a"), "a");
  }}

Property testing:
  proptest! {{
      #[test]
      fn test_reverse_twice(s in ".*") {{
          // Property: reversing twice gives original
          assert_eq!(reverse(&reverse(&s)), s);
      }}
  }}

proptest will generate hundreds of random strings automatically!
"#
    );

    println!();

    // =========================================================================
    // BASIC PROPTEST
    // =========================================================================

    println!("--- Basic proptest ---");
    println!(
        r#"
Add to Cargo.toml:
  [dev-dependencies]
  proptest = "1"

Basic usage:

use proptest::prelude::*;

proptest! {{
    #[test]
    fn test_encode_decode(data: Vec<u8>) {{
        // Property: decode(encode(x)) == x
        let encoded = base64_encode(&data);
        let decoded = base64_decode(&encoded).unwrap();
        prop_assert_eq!(decoded, data);
    }}
}}

proptest! {{
    #[test]
    fn test_hash_deterministic(data: Vec<u8>) {{
        // Property: hashing same data gives same result
        let hash1 = sha256(&data);
        let hash2 = sha256(&data);
        prop_assert_eq!(hash1, hash2);
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // STRATEGIES
    // =========================================================================

    println!("--- Strategies ---");
    println!(
        r#"
Strategies define how to generate test data:

use proptest::prelude::*;

proptest! {{
    // Built-in strategies
    #[test]
    fn test_with_string(s in ".*") {{ }}

    #[test]
    fn test_with_range(n in 0..100i32) {{ }}

    #[test]
    fn test_with_vec(v in prop::collection::vec(any::<u8>(), 0..100)) {{ }}
}}

// Custom strategies
fn port_strategy() -> impl Strategy<Value = u16> {{
    1024..65535u16
}}

fn valid_email() -> impl Strategy<Value = String> {{
    "[a-z]{{5,10}}@[a-z]{{3,8}}\\.(com|org|net)"
}}

proptest! {{
    #[test]
    fn test_valid_port(port in port_strategy()) {{
        prop_assert!(port >= 1024);
    }}

    #[test]
    fn test_email_format(email in valid_email()) {{
        prop_assert!(email.contains('@'));
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // REGEX STRATEGIES
    // =========================================================================

    println!("--- Regex Strategies ---");
    println!(
        r##"
Generate strings matching patterns:

proptest! {{
    // UUID-like strings
    #[test]
    fn test_uuid_format(
        s in "[0-9a-f]{{8}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{12}}"
    ) {{
        prop_assert_eq!(s.len(), 36);
        prop_assert!(s.chars().filter(|c| *c == '-').count() == 4);
    }}

    // Version strings
    #[test]
    fn test_version_parse(
        major in 0..100u32,
        minor in 0..100u32,
        patch in 0..100u32,
    ) {{
        let version = format!("{{}}.{{}}.{{}}", major, minor, patch);
        let parsed = parse_version(&version).unwrap();
        prop_assert_eq!(parsed, (major, minor, patch));
    }}

    // JSON-like keys
    #[test]
    fn test_json_key(key in "[a-z_][a-z0-9_]{{0,30}}") {{
        let json = format!(r#"{{"{{}}: "value"}}"#, key);
        prop_assert!(serde_json::from_str::<Value>(&json).is_ok());
    }}
}}
"##
    );

    println!();

    // =========================================================================
    // COMPOSITE STRATEGIES
    // =========================================================================

    println!("--- Composite Strategies ---");
    println!(
        r#"
Combine strategies for complex types:

use proptest::prelude::*;

// Custom struct
#[derive(Debug, Clone, PartialEq)]
struct Config {{
    name: String,
    port: u16,
    debug: bool,
}}

// Strategy for Config
fn config_strategy() -> impl Strategy<Value = Config> {{
    (
        "[a-z]{{3,10}}",      // name
        1024..65535u16,        // port
        any::<bool>(),         // debug
    )
        .prop_map(|(name, port, debug)| Config {{ name, port, debug }})
}}

proptest! {{
    #[test]
    fn test_config_roundtrip(config in config_strategy()) {{
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        prop_assert_eq!(config, deserialized);
    }}
}}

// Or using prop_compose macro
prop_compose! {{
    fn arb_config()(
        name in "[a-z]{{3,10}}",
        port in 1024..65535u16,
        debug in any::<bool>(),
    ) -> Config {{
        Config {{ name, port, debug }}
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // SHRINKING
    // =========================================================================

    println!("--- Shrinking ---");
    println!(
        r#"
When a test fails, proptest "shrinks" to find minimal failing case:

proptest! {{
    #[test]
    fn test_that_fails(v in prop::collection::vec(1..100i32, 1..100)) {{
        // Fails when sum > 500
        let sum: i32 = v.iter().sum();
        prop_assert!(sum <= 500);
    }}
}}

// Output:
// thread 'test_that_fails' panicked
// proptest: Saving this and future failures in ...
//
// Test failed for input:
//   v = [84, 84, 84, 84, 84, 84]  // Minimal failing case!
//
// The original random input might have been [23, 87, 12, 99, ...]
// but proptest shrunk it to the simplest failing example.
"#
    );

    println!();

    // =========================================================================
    // CLI TESTING WITH PROPTEST
    // =========================================================================

    println!("--- CLI Testing with proptest ---");
    println!(
        r#"
Test CLI with random inputs:

use assert_cmd::Command;

proptest! {{
    #[test]
    fn test_hash_any_input(data: Vec<u8>) {{
        let mut cmd = Command::cargo_bin("dx").unwrap();
        cmd.args(["hash", "-a", "sha256", "-"])
            .write_stdin(&data[..]);

        let result = cmd.output().unwrap();
        prop_assert!(result.status.success());

        let stdout = String::from_utf8_lossy(&result.stdout);
        prop_assert!(stdout.len() >= 64);  // SHA256 = 64 hex chars
    }}

    #[test]
    fn test_encode_decode_roundtrip(data: Vec<u8>) {{
        // Encode
        let mut encode = Command::cargo_bin("dx").unwrap();
        encode.args(["encode", "base64", "-"])
            .write_stdin(&data[..]);
        let encoded = encode.output().unwrap();
        prop_assert!(encoded.status.success());

        // Decode
        let mut decode = Command::cargo_bin("dx").unwrap();
        decode.args(["encode", "base64", "-d", "-"])
            .write_stdin(encoded.stdout);
        let decoded = decode.output().unwrap();

        prop_assert_eq!(decoded.stdout, data);
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // CONFIGURATION
    // =========================================================================

    println!("--- Configuration ---");
    println!(
        r#"
Configure proptest behavior:

// In proptest.toml or ProptestConfig
proptest! {{
    #![proptest_config(ProptestConfig {{
        cases: 1000,           // Run 1000 test cases (default: 256)
        max_shrink_iters: 100, // Shrinking iterations
        ..ProptestConfig::default()
    }})]

    #[test]
    fn thorough_test(x in any::<i32>()) {{
        // Runs 1000 times
    }}
}}

// Or via environment:
//   PROPTEST_CASES=10000 cargo test

// Disable shrinking for debugging:
proptest! {{
    #![proptest_config(ProptestConfig::with_cases(100).no_shrink())]

    #[test]
    fn test_no_shrink(x in any::<i32>()) {{ }}
}}
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Property testing with proptest:");
    println!("  1. Generate random test inputs automatically");
    println!("  2. Find edge cases you wouldn't think of");
    println!("  3. Shrinking finds minimal failing cases");
    println!("  4. Use strategies for custom types");
    println!("  5. Great for encode/decode roundtrips");
}
