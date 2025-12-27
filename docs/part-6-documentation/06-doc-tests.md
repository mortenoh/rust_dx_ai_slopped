# Doc Tests

Ensure documentation examples always work.

## Basic Doc Test

```rust
/// Adds two numbers.
///
/// ```
/// assert_eq!(dx::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run with:
```bash
cargo test --doc
```

## Hiding Setup Code

```rust
/// Parse a config file.
///
/// ```
/// # use std::io::Write;
/// # let mut file = tempfile::NamedTempFile::new().unwrap();
/// # write!(file, "key=value").unwrap();
/// # let path = file.path();
/// let config = dx::parse_config(path);
/// assert!(config.is_ok());
/// ```
pub fn parse_config(path: &Path) -> Result<Config> {
    // Lines starting with # are hidden but still run
}
```

## Test Attributes

### should_panic

```rust
/// Panics on empty input.
///
/// ```should_panic
/// dx::process("");  // This will panic
/// ```
pub fn process(s: &str) {
    assert!(!s.is_empty());
}
```

### no_run

```rust
/// Connects to a server.
///
/// ```no_run
/// // Compiles but doesn't execute
/// dx::connect("localhost:8080")?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn connect(addr: &str) -> std::io::Result<()> {
    todo!()
}
```

### ignore

```rust
/// Expensive operation.
///
/// ```ignore
/// // Not compiled or run
/// dx::slow_operation();
/// ```
pub fn slow_operation() {}
```

### compile_fail

```rust
/// This requires a mutable reference.
///
/// ```compile_fail
/// let x = 5;
/// dx::modify(&x);  // Error: expected mutable reference
/// ```
pub fn modify(x: &mut i32) {
    *x += 1;
}
```

## Handling Results

```rust
/// Reads a file.
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// let content = std::fs::read_to_string("Cargo.toml")?;
/// assert!(content.contains("[package]"));
/// # Ok(())
/// # }
/// ```
pub fn read_file() {}
```

Or with the question mark shorthand:
```rust
/// ```
/// let content = std::fs::read_to_string("Cargo.toml")?;
/// # Ok::<(), std::io::Error>(())
/// ```
```

## Testing Private Items

```rust
/// ```
/// // Access private items in doc tests
/// use my_crate::internal::private_fn;
/// ```
#[doc(hidden)]
pub mod internal {
    pub fn private_fn() {}
}
```

## Combining with Unit Tests

```rust
/// Encodes to base64.
///
/// ```
/// assert_eq!(dx::encode::base64(b"hello"), "aGVsbG8=");
/// ```
pub fn base64(data: &[u8]) -> String {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_edge_cases() {
        // More thorough testing here
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"a"), "YQ==");
    }
}
```

## Running Doc Tests

```bash
# All doc tests
cargo test --doc

# Specific module
cargo test --doc hash

# With output
cargo test --doc -- --nocapture

# List doc tests
cargo test --doc -- --list
```
