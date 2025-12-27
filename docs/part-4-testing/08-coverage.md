# Code Coverage

Measure how much code is exercised by tests.

## Using cargo-llvm-cov

```bash
cargo install cargo-llvm-cov

cargo llvm-cov           # Run with coverage
cargo llvm-cov --html    # HTML report
```

## Output

```
Filename             Regions   Cover   Lines   Cover
----------------------------------------------------
src/commands/hash.rs      45  93.33%    120  93.33%
src/config.rs             28 100.00%     75 100.00%
----------------------------------------------------
TOTAL                     73  95.89%    195  95.38%
```

## Thresholds

```bash
cargo llvm-cov --fail-under-lines 80
```

## Excluding Code

```rust
#[cfg(not(tarpaulin_include))]
fn debug_only() { }
```

## CI Integration

```yaml
- name: Coverage
  run: cargo llvm-cov --lcov --output-path lcov.info

- uses: codecov/codecov-action@v3
  with:
    files: lcov.info
```

## Best Practices

1. Don't chase 100%
2. Cover critical paths
3. Ignore generated code
4. Track trends over time
