//! # JSON Command Implementation
//!
//! This example shows how to implement JSON formatting and validation.
//!
//! Run with: `cargo run --example 0205_json_command`

#![allow(dead_code)]

use serde_json::Value;

// =========================================================================
// FORMATTING
// =========================================================================

/// Pretty print JSON with custom indentation
pub fn format_json(value: &Value, indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_str.as_bytes());
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    serde::Serialize::serialize(value, &mut ser).unwrap();
    String::from_utf8(buf).unwrap()
}

/// Minify JSON (remove whitespace)
pub fn minify_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap()
}

/// Sort JSON object keys recursively
pub fn sort_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted.insert(key.clone(), sort_keys(&map[key]));
            }
            Value::Object(sorted)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_keys).collect()),
        _ => value.clone(),
    }
}

// =========================================================================
// VALIDATION
// =========================================================================

/// Validate JSON string
pub fn validate_json(input: &str) -> Result<(), String> {
    serde_json::from_str::<Value>(input)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// =========================================================================
// QUERYING
// =========================================================================

/// Simple JSON path query (e.g., ".foo.bar[0]")
pub fn query_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;

    for part in path.split('.').filter(|s| !s.is_empty()) {
        if let Some(bracket_pos) = part.find('[') {
            let key = &part[..bracket_pos];
            let index_str = &part[bracket_pos + 1..part.len() - 1];
            let index: usize = index_str.parse().ok()?;

            if !key.is_empty() {
                current = current.get(key)?;
            }
            current = current.get(index)?;
        } else {
            current = current.get(part)?;
        }
    }

    Some(current)
}

fn main() {
    println!("=== JSON Command Implementation ===\n");

    // =========================================================================
    // SAMPLE DATA
    // =========================================================================

    let sample = r#"{
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com",
        "active": true,
        "tags": ["rust", "cli", "json"],
        "address": {
            "city": "New York",
            "zip": "10001"
        }
    }"#;

    let value: Value = serde_json::from_str(sample).unwrap();

    // =========================================================================
    // FORMATTING
    // =========================================================================

    println!("--- Pretty Print (2 spaces) ---");
    println!("{}", format_json(&value, 2));

    println!("--- Pretty Print (4 spaces) ---");
    println!("{}", format_json(&value, 4));

    println!();

    // =========================================================================
    // MINIFICATION
    // =========================================================================

    println!("--- Minified ---");
    println!("{}", minify_json(&value));

    println!();

    // =========================================================================
    // SORTING KEYS
    // =========================================================================

    println!("--- Sorted Keys ---");
    let sorted = sort_keys(&value);
    println!("{}", format_json(&sorted, 2));

    println!();

    // =========================================================================
    // VALIDATION
    // =========================================================================

    println!("--- Validation ---");

    let valid = r#"{"key": "value"}"#;
    let invalid = r#"{"key": value}"#;

    match validate_json(valid) {
        Ok(()) => println!("  Valid JSON: ✓"),
        Err(e) => println!("  Valid JSON: ✗ {}", e),
    }

    match validate_json(invalid) {
        Ok(()) => println!("  Invalid JSON: ✓"),
        Err(e) => println!("  Invalid JSON: ✗ {}", e),
    }

    println!();

    // =========================================================================
    // QUERYING
    // =========================================================================

    println!("--- JSON Path Query ---");

    let queries = [".name", ".age", ".tags[0]", ".address.city", ".missing"];

    for query in queries {
        match query_path(&value, query) {
            Some(v) => println!("  {} = {}", query, v),
            None => println!("  {} = (not found)", query),
        }
    }

    println!();

    // =========================================================================
    // CLI USAGE
    // =========================================================================

    println!("--- CLI Usage ---");
    println!(
        r#"
# Pretty print (default 2-space indent)
dx json format file.json
echo '{{"a":1}}' | dx json format

# Custom indent
dx json format file.json -i 4
dx json format file.json --tabs

# Minify
dx json minify file.json

# Sort keys
dx json format file.json -s

# Compact (minify alias)
dx json format file.json -c

# Validate
dx json validate file.json
dx json validate file.json -q  # quiet (exit code only)

# Query
dx json query file.json -p ".users[0].name"
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("JSON command features:");
    println!("  1. Pretty print with custom indentation");
    println!("  2. Minification");
    println!("  3. Key sorting");
    println!("  4. Validation with error messages");
    println!("  5. Simple path queries");
}
