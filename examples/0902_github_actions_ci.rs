//! # GitHub Actions CI
//!
//! This example shows how to set up CI with GitHub Actions.
//!
//! Run with: `cargo run --example 0902_github_actions_ci`

#![allow(dead_code)]

fn main() {
    println!("=== GitHub Actions CI ===\n");

    // =========================================================================
    // BASIC CI WORKFLOW
    // =========================================================================

    println!("--- Basic CI Workflow ---");
    println!(
        r#"
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2

      - name: Run tests
        run: cargo test --all-features

  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Check formatting
        run: cargo fmt --check

      - name: Clippy
        run: cargo clippy -- -D warnings
"#
    );

    println!();

    // =========================================================================
    // MULTI-PLATFORM TESTING
    // =========================================================================

    println!("--- Multi-Platform Testing ---");
    println!(
        r#"
# .github/workflows/ci.yml (continued)

jobs:
  test:
    name: Test ${{{{ matrix.os }}}} / ${{{{ matrix.rust }}}}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
        exclude:
          - os: windows-latest
            rust: beta

    runs-on: ${{{{ matrix.os }}}}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust ${{{{ matrix.rust }}}}
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{{{ matrix.rust }}}}

      - uses: Swatinem/rust-cache@v2
        with:
          key: ${{{{ matrix.os }}}}-${{{{ matrix.rust }}}}

      - name: Build
        run: cargo build --all-features

      - name: Test
        run: cargo test --all-features

      - name: Doc tests
        run: cargo test --doc
"#
    );

    println!();

    // =========================================================================
    // CACHING
    // =========================================================================

    println!("--- Caching ---");
    println!(
        r#"
Efficient caching with rust-cache:

- uses: Swatinem/rust-cache@v2
  with:
    # Cache key prefix
    prefix-key: "v1-rust"

    # Additional cache key
    key: ${{{{ matrix.os }}}}-${{{{ hashFiles('**/Cargo.lock') }}}}

    # Directories to cache
    cache-directories: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      target/

    # Save cache even on failure
    save-if: ${{{{ github.ref == 'refs/heads/main' }}}}

Alternative: manual caching

- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/
      ~/.cargo/git/
      target/
    key: ${{{{ runner.os }}}}-cargo-${{{{ hashFiles('**/Cargo.lock') }}}}
    restore-keys: |
      ${{{{ runner.os }}}}-cargo-
"#
    );

    println!();

    // =========================================================================
    // ADVANCED CHECKS
    // =========================================================================

    println!("--- Advanced Checks ---");
    println!(
        r#"
Additional CI checks:

jobs:
  # Security audit
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v2
        with:
          token: ${{{{ secrets.GITHUB_TOKEN }}}}

  # Code coverage
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-llvm-cov
        uses: taiki-e/install-action@cargo-llvm-cov

      - name: Generate coverage
        run: cargo llvm-cov --all-features --lcov --output-path lcov.info

      - name: Upload to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: lcov.info
          fail_ci_if_error: true

  # Minimum supported Rust version
  msrv:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.70.0  # MSRV
      - run: cargo build --all-features

  # Documentation
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo doc --no-deps --all-features
        env:
          RUSTDOCFLAGS: -D warnings
"#
    );

    println!();

    // =========================================================================
    // ARTIFACTS
    // =========================================================================

    println!("--- Build Artifacts ---");
    println!(
        r#"
Upload build artifacts:

jobs:
  build:
    runs-on: ${{{{ matrix.os }}}}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: dx-linux-x64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: dx-macos-x64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: dx-windows-x64.exe

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Build
        run: cargo build --release

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{{{ matrix.artifact }}}}
          path: |
            target/release/dx
            target/release/dx.exe
          if-no-files-found: error
"#
    );

    println!();

    // =========================================================================
    // TRIGGERS AND FILTERS
    // =========================================================================

    println!("--- Triggers and Filters ---");
    println!(
        r#"
Control when workflows run:

on:
  push:
    branches: [main]
    tags:
      - 'v*'
    paths:
      - 'src/**'
      - 'Cargo.toml'
      - 'Cargo.lock'
    paths-ignore:
      - '**.md'
      - 'docs/**'

  pull_request:
    types: [opened, synchronize, reopened]

  schedule:
    # Run weekly security audit
    - cron: '0 0 * * 0'

  workflow_dispatch:
    # Manual trigger with inputs
    inputs:
      debug:
        description: 'Enable debug mode'
        required: false
        default: 'false'

# Use in job
jobs:
  build:
    if: github.event_name != 'schedule' || github.repository == 'owner/repo'
    runs-on: ubuntu-latest
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("GitHub Actions CI:");
    println!("  1. Test on multiple OS and Rust versions");
    println!("  2. Use rust-cache for fast builds");
    println!("  3. Add security audits and coverage");
    println!("  4. Upload build artifacts");
    println!("  5. Control triggers with paths/branches");
}
