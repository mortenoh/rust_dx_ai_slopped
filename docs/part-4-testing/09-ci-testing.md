# CI Testing

Automated testing in continuous integration.

## GitHub Actions

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - run: cargo build
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

## Matrix Testing

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta]

runs-on: ${{ matrix.os }}
```

## Feature Testing

```yaml
- run: cargo test --no-default-features
- run: cargo test --all-features
```

## Coverage in CI

```yaml
- uses: taiki-e/install-action@cargo-llvm-cov
- run: cargo llvm-cov --lcov --output-path lcov.info
- uses: codecov/codecov-action@v3
```

## Caching

```yaml
- uses: Swatinem/rust-cache@v2
```

## Required Checks

Configure branch protection to require:
- `test` job passing
- `clippy` job passing
- `fmt` check passing
