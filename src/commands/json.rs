//! # JSON Command Implementation
//!
//! This module provides JSON utilities: formatting, validation, minification,
//! and path-based querying.
//!
//! ## Key Concepts
//!
//! ### Serde and serde_json
//! - **Serde**: Rust's serialization/deserialization framework
//! - **serde_json**: Serde implementation for JSON
//! - **Value**: A dynamic JSON type that can hold any JSON data
//!
//! ### JSON Value Types
//! ```text
//! Value::Null       -> null
//! Value::Bool       -> true, false
//! Value::Number     -> 42, 3.14
//! Value::String     -> "hello"
//! Value::Array      -> [1, 2, 3]
//! Value::Object     -> {"key": "value"}
//! ```
//!
//! ## Example Usage
//! ```bash
//! dx json format data.json              # Pretty-print JSON
//! dx json format --compact data.json    # Minify JSON
//! dx json format --tabs data.json       # Use tabs for indentation
//! dx json format --sort-keys data.json  # Sort object keys
//! dx json validate data.json            # Check if valid JSON
//! dx json minify data.json              # Remove whitespace
//! dx json query data.json ".foo.bar[0]" # Extract value at path
//! ```
//!
//! ## External Documentation
//! - Serde JSON: <https://docs.rs/serde_json>
//! - Serde: <https://serde.rs/>
//! - JSON specification: <https://www.json.org/>

use crate::cli::commands::json::{JsonArgs, JsonCommand};
use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Run the JSON command, dispatching to the appropriate subcommand.
///
/// JSON has several subcommands:
/// - `format`: Pretty-print with customizable indentation
/// - `validate`: Check if input is valid JSON
/// - `minify`: Remove all unnecessary whitespace
/// - `query`: Extract a value using a path expression
pub fn run(args: JsonArgs) -> Result<()> {
    match args.command {
        JsonCommand::Format {
            input,
            indent,
            tabs,
            sort_keys,
            compact,
        } => cmd_format(input.as_deref(), indent, tabs, sort_keys, compact),
        JsonCommand::Validate { input, quiet } => cmd_validate(input.as_deref(), quiet),
        JsonCommand::Minify { input } => cmd_minify(input.as_deref()),
        JsonCommand::Query { input, path } => cmd_query(input.as_deref(), &path),
    }
}

/// Read JSON input from a file, stdin, or "-" (explicit stdin).
///
/// # Input Sources
/// - `None`: Read from stdin (allows piping)
/// - `Some("-")`: Explicitly read from stdin
/// - `Some(path)`: Read from the specified file
///
/// # Why read_to_string instead of from_reader?
/// For error messages! If we parse directly from a reader, serde_json
/// can't show the problematic line. By reading to string first, we get
/// better error context and can potentially show the invalid portion.
fn read_input(path: Option<&Path>) -> Result<String> {
    if let Some(p) = path {
        if p.to_string_lossy() == "-" {
            // "-" is Unix convention for stdin
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .context("Failed to read from stdin")?;
            Ok(input)
        } else {
            // Read from file
            let mut file =
                File::open(p).with_context(|| format!("Failed to open {}", p.display()))?;
            let mut input = String::new();
            file.read_to_string(&mut input)
                .with_context(|| format!("Failed to read {}", p.display()))?;
            Ok(input)
        }
    } else {
        // No path specified: default to stdin
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .context("Failed to read from stdin")?;
        Ok(input)
    }
}

/// Format/pretty-print JSON with customizable options.
///
/// # Options
/// - `indent`: Number of spaces per indentation level (default: 2)
/// - `tabs`: Use tabs instead of spaces
/// - `sort_keys`: Alphabetically sort object keys
/// - `compact`: Output on a single line (opposite of pretty-print)
///
/// # Custom Indentation
/// serde_json's default pretty-printer uses 2 spaces. For custom indentation,
/// we use `PrettyFormatter::with_indent()` which accepts any byte slice.
///
/// # Tab Indentation Workaround
/// serde_json doesn't directly support tabs, so we:
/// 1. Pretty-print with 2-space indentation
/// 2. Replace all "  " (2 spaces) with "\t"
///
/// This is a simple hack but works for most cases.
fn cmd_format(
    input: Option<&Path>,
    indent: usize,
    tabs: bool,
    sort_keys: bool,
    compact: bool,
) -> Result<()> {
    // Parse JSON into a dynamic Value type
    let json_str = read_input(input)?;
    let value: Value = serde_json::from_str(&json_str).context("Invalid JSON")?;

    // Optionally sort keys (creates a new Value with sorted keys)
    let value = if sort_keys { sort_json(&value) } else { value };

    // Format according to options
    let output = if compact {
        // Compact: no whitespace, single line
        // to_string() produces minimal JSON
        serde_json::to_string(&value)?
    } else if tabs {
        // Tab indentation: use the workaround
        let spaces = serde_json::to_string_pretty(&value)?;
        // Replace 2-space indentation with tabs
        spaces.replace("  ", "\t")
    } else {
        // Custom space indentation using PrettyFormatter
        // This is more complex but gives us full control
        let indent_str = " ".repeat(indent);
        let indent_bytes = indent_str.as_bytes();

        // Create a formatter with our custom indentation
        // See: https://docs.rs/serde_json/latest/serde_json/ser/struct.PrettyFormatter.html
        let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_bytes);

        // Serialize to a byte buffer using our custom formatter
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);

        // Use Serde's Serialize trait to write the value
        serde::Serialize::serialize(&value, &mut ser)?;

        // Convert bytes back to string (JSON is always UTF-8)
        String::from_utf8(buf)?
    };

    println!("{}", output);
    Ok(())
}

/// Validate that input is valid JSON.
///
/// # Exit Codes
/// - 0: Valid JSON
/// - 1: Invalid JSON
///
/// This follows Unix conventions where non-zero exit indicates failure.
/// Scripts can use: `if dx json validate file.json; then ...`
///
/// # Quiet Mode
/// With `--quiet`, no output is produced - only the exit code matters.
/// Useful for scripting where you only care about success/failure.
fn cmd_validate(input: Option<&Path>, quiet: bool) -> Result<()> {
    let json_str = read_input(input)?;

    // Try to parse as JSON
    // We parse into Value (not a specific type) to accept any valid JSON
    match serde_json::from_str::<Value>(&json_str) {
        Ok(_) => {
            if !quiet {
                println!("{} Valid JSON", "✓".green().bold());
            }
            Ok(())
        }
        Err(e) => {
            if quiet {
                // Silent failure: just exit with error code
                std::process::exit(1);
            } else {
                // Show the error message (includes line/column info)
                eprintln!("{} Invalid JSON: {}", "✗".red().bold(), e);
                std::process::exit(1);
            }
        }
    }
}

/// Minify JSON by removing all unnecessary whitespace.
///
/// This is equivalent to `format --compact` but more discoverable.
/// Minified JSON is useful for:
/// - Reducing file/payload size
/// - Single-line log entries
/// - Embedding in URLs or other space-constrained contexts
fn cmd_minify(input: Option<&Path>) -> Result<()> {
    let json_str = read_input(input)?;
    let value: Value = serde_json::from_str(&json_str).context("Invalid JSON")?;
    // to_string() produces compact JSON without pretty-printing
    println!("{}", serde_json::to_string(&value)?);
    Ok(())
}

/// Query JSON using a simple path expression.
///
/// # Path Syntax
/// - `.foo`: Access object key "foo"
/// - `.foo.bar`: Nested object access
/// - `[0]`: Array index access
/// - `.foo[0]`: Combined object and array access
/// - `.foo.bar[0].baz`: Deep nesting
///
/// # Examples
/// ```text
/// Input: {"users": [{"name": "Alice"}, {"name": "Bob"}]}
/// Path: .users[0].name
/// Output: "Alice"
/// ```
///
/// # Limitations
/// This is a simple implementation. For complex queries, consider:
/// - jq: <https://stedolan.github.io/jq/>
/// - JSONPath: <https://goessner.net/articles/JsonPath/>
fn cmd_query(input: Option<&Path>, path: &str) -> Result<()> {
    let json_str = read_input(input)?;
    let value: Value = serde_json::from_str(&json_str).context("Invalid JSON")?;

    // Parse and execute the path query
    let result = query_path(&value, path)?;

    // Pretty-print the result (could be any JSON type)
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}

/// Execute a path query on a JSON value.
///
/// # How It Works
///
/// 1. Split the path by `.` to get individual segments
/// 2. For each segment:
///    - If it contains `[`, it's a key + array index (e.g., "items[0]")
///    - Otherwise, it's just a key (e.g., "foo")
/// 3. Navigate through the JSON structure
///
/// # Lifetime Parameter
/// The `'a` lifetime ensures the returned reference lives as long as
/// the input `value`. This avoids cloning the result.
///
/// # Error Handling
/// Returns an error if:
/// - A key doesn't exist in an object
/// - An array index is out of bounds
/// - The path syntax is invalid
fn query_path<'a>(value: &'a Value, path: &str) -> Result<&'a Value> {
    let mut current = value;

    // Split by '.' and skip empty segments (handles leading '.')
    for part in path.split('.').filter(|s| !s.is_empty()) {
        // Check if this segment includes an array index
        if let Some(bracket_pos) = part.find('[') {
            // Parse: "key[index]" -> key="key", index=number
            let key = &part[..bracket_pos];
            // Extract the number between [ and ]
            let index_str = &part[bracket_pos + 1..part.len() - 1];
            let index: usize = index_str.parse().context("Invalid array index")?;

            // First access the object key (if not empty)
            if !key.is_empty() {
                current = current
                    .get(key)
                    .with_context(|| format!("Key '{}' not found", key))?;
            }

            // Then access the array index
            current = current
                .get(index)
                .with_context(|| format!("Index {} out of bounds", index))?;
        } else {
            // Simple key access
            current = current
                .get(part)
                .with_context(|| format!("Key '{}' not found", part))?;
        }
    }

    Ok(current)
}

/// Recursively sort all object keys in a JSON value.
///
/// # Why Sort Keys?
/// - **Deterministic output**: Same data always produces same JSON
/// - **Easier diffs**: Changes are more visible when keys are ordered
/// - **Testing**: Predictable output for assertions
///
/// # Recursion
/// This function is recursive because JSON objects can contain other objects.
/// We need to sort keys at every level, not just the top level.
///
/// # How It Works
/// ```text
/// 1. If Object: collect keys, sort them, rebuild with sorted order
/// 2. If Array: recursively sort each element
/// 3. Otherwise: return value unchanged (String, Number, Bool, Null)
/// ```
fn sort_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            // Create a new map (which is a BTreeMap or IndexMap internally)
            let mut sorted: serde_json::Map<String, Value> = serde_json::Map::new();

            // Get all keys and sort them alphabetically
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();

            // Insert in sorted order, recursively sorting nested values
            for key in keys {
                sorted.insert(key.clone(), sort_json(&map[key]));
            }

            Value::Object(sorted)
        }
        Value::Array(arr) => {
            // Recursively sort objects within arrays
            Value::Array(arr.iter().map(sort_json).collect())
        }
        _ => {
            // Primitives (String, Number, Bool, Null) are returned as-is
            value.clone()
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test simple object key access.
    #[test]
    fn test_query_simple() {
        let json: Value = serde_json::json!({"foo": {"bar": 42}});
        let result = query_path(&json, ".foo.bar").unwrap();
        assert_eq!(result, &Value::Number(42.into()));
    }

    /// Test array index access.
    #[test]
    fn test_query_array() {
        let json: Value = serde_json::json!({"items": [1, 2, 3]});
        let result = query_path(&json, ".items[1]").unwrap();
        assert_eq!(result, &Value::Number(2.into()));
    }

    /// Test that sort_json orders keys alphabetically.
    #[test]
    fn test_sort_json() {
        let json: Value = serde_json::json!({"z": 1, "a": 2, "m": 3});
        let sorted = sort_json(&json);
        let keys: Vec<_> = sorted.as_object().unwrap().keys().collect();
        assert_eq!(keys, vec!["a", "m", "z"]);
    }
}
