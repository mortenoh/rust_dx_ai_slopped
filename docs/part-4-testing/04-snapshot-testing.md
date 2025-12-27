# Snapshot Testing with insta

Capture and compare output against saved snapshots.

## Setup

```bash
cargo add --dev insta
cargo install cargo-insta
```

## Basic Snapshots

```rust
use insta::assert_snapshot;

#[test]
fn test_output() {
    let output = format_data(&data);
    assert_snapshot!(output);
}
```

## Named Snapshots

```rust
#[test]
fn test_formats() {
    assert_snapshot!("json", format_json(&data));
    assert_snapshot!("yaml", format_yaml(&data));
}
```

## Debug Snapshots

```rust
use insta::assert_debug_snapshot;

#[test]
fn test_parsed() {
    let config = parse_config("input.toml").unwrap();
    assert_debug_snapshot!(config);
}
```

## Redacting Values

```rust
use insta::with_settings;

#[test]
fn test_with_timestamp() {
    with_settings!({
        filters => vec![(r"\d{4}-\d{2}-\d{2}", "[DATE]")]
    }, {
        assert_snapshot!(output);
    });
}
```

## Managing Snapshots

```bash
cargo insta review   # Review pending
cargo insta accept   # Accept all
cargo insta reject   # Reject all
```
