# Cross Compilation

Build binaries for other platforms.

## Rust Targets

```bash
# List available targets
rustup target list

# Common targets
x86_64-unknown-linux-gnu      # Linux x86_64
x86_64-unknown-linux-musl     # Linux static binary
aarch64-unknown-linux-gnu     # Linux ARM64
x86_64-apple-darwin           # macOS Intel
aarch64-apple-darwin          # macOS Apple Silicon
x86_64-pc-windows-msvc        # Windows x86_64
x86_64-pc-windows-gnu         # Windows (MinGW)
```

## Adding Targets

```bash
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-apple-darwin
```

## Building for Target

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

Output: `target/x86_64-unknown-linux-musl/release/dx`

## Using cross

For complex cross-compilation:

```bash
cargo install cross
```

```bash
# Build Linux binary from macOS
cross build --release --target x86_64-unknown-linux-gnu

# Build Windows binary from Linux
cross build --release --target x86_64-pc-windows-gnu
```

## Cross Configuration

```toml
# Cross.toml
[target.x86_64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main"

[target.aarch64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:main"
```

## Static Binaries (musl)

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Build static binary
cargo build --release --target x86_64-unknown-linux-musl

# Verify it's static
ldd target/x86_64-unknown-linux-musl/release/dx
# Output: not a dynamic executable
```

## macOS Universal Binary

```bash
# Add both targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for both
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Combine into universal binary
lipo -create \
  target/x86_64-apple-darwin/release/dx \
  target/aarch64-apple-darwin/release/dx \
  -output dx-universal
```

## CI Release Workflow

```yaml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            artifact: dx-linux-x86_64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: dx-macos-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: dx-macos-aarch64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: dx-windows-x86_64.exe

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install target
        run: rustup target add ${{ matrix.target }}

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Package
        run: |
          if [ "${{ matrix.os }}" = "windows-latest" ]; then
            cp target/${{ matrix.target }}/release/dx.exe ${{ matrix.artifact }}
          else
            cp target/${{ matrix.target }}/release/dx ${{ matrix.artifact }}
          fi
        shell: bash

      - uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: ${{ matrix.artifact }}

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4

      - uses: softprops/action-gh-release@v1
        with:
          files: |
            dx-linux-x86_64/*
            dx-macos-x86_64/*
            dx-macos-aarch64/*
            dx-windows-x86_64.exe/*
```

## Cargo Config

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```
