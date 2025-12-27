# Property-Based Testing

Generate random inputs to find edge cases.

## Setup

```bash
cargo add --dev proptest
```

## Basic Property Test

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_reverse_twice(s in "\\PC*") {
        let reversed = reverse(&reverse(&s));
        prop_assert_eq!(&reversed, &s);
    }
}
```

## Custom Strategies

```rust
fn port_strategy() -> impl Strategy<Value = u16> {
    1..=65535u16
}

proptest! {
    #[test]
    fn test_valid_port(port in port_strategy()) {
        prop_assert!(port > 0);
    }
}
```

## Testing Roundtrips

```rust
proptest! {
    #[test]
    fn test_hex_roundtrip(bytes in prop::collection::vec(any::<u8>(), 0..100)) {
        let hex = to_hex(&bytes);
        let parsed = from_hex(&hex).unwrap();
        prop_assert_eq!(parsed, bytes);
    }
}
```

## Shrinking

When a test fails, proptest automatically shrinks the input to find the minimal failing case.

## Configuration

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn thorough_test(x in 0..100i32) {
        // Runs 1000 cases
    }
}
```
