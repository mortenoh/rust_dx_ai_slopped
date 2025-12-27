# Unit Testing

Unit tests verify individual functions work correctly.

## Basic Tests

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

## Assertions

```rust
assert_eq!(1 + 1, 2);
assert_ne!(1 + 1, 3);
assert!(true);
assert!(value > 0, "Expected positive, got {}", value);
```

## Testing Results

```rust
#[test]
fn test_parse_valid() {
    assert!(parse_port("8080").is_ok());
}

#[test]
fn test_parse_invalid() {
    assert!(parse_port("abc").is_err());
}
```

## Expected Panics

```rust
#[test]
#[should_panic(expected = "divide by zero")]
fn test_panic() {
    divide(10, 0);
}
```

## Ignoring Tests

```rust
#[test]
#[ignore = "slow test"]
fn expensive_test() {
    // Run with: cargo test -- --ignored
}
```
