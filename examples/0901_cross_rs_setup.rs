//! # Cross-Compilation with cross-rs
//!
//! This example shows how to set up cross-compilation.
//!
//! Run with: `cargo run --example 0901_cross_rs_setup`

#![allow(dead_code)]

fn main() {
    println!("=== Cross-Compilation with cross-rs ===\n");

    // =========================================================================
    // INSTALLATION
    // =========================================================================

    println!("--- Installation ---");
    println!(
        r#"
Install cross-rs:

  cargo install cross --git https://github.com/cross-rs/cross

Requirements:
  - Docker (or Podman)
  - Rust toolchain

Verify installation:
  cross --version
  docker --version
"#
    );

    println!();

    // =========================================================================
    // BASIC USAGE
    // =========================================================================

    println!("--- Basic Usage ---");
    println!(
        r#"
Cross compiles your project in a Docker container:

# Build for Linux ARM64 (from any host)
cross build --target aarch64-unknown-linux-gnu --release

# Build for Linux musl (static binary)
cross build --target x86_64-unknown-linux-musl --release

# Build for Windows (from Linux/macOS)
cross build --target x86_64-pc-windows-gnu --release

# Run tests on target architecture
cross test --target aarch64-unknown-linux-gnu

Output is in: target/<target>/release/dx
"#
    );

    println!();

    // =========================================================================
    // CROSS.TOML
    // =========================================================================

    println!("--- Cross.toml Configuration ---");
    println!(
        r#"
Create Cross.toml in project root:

# Cross.toml

[build.env]
passthrough = [
    "RUST_LOG",
    "DX_VERSION",
]

[target.x86_64-unknown-linux-gnu]
# Use pre-built image
image = "ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main"

[target.x86_64-unknown-linux-musl]
image = "ghcr.io/cross-rs/x86_64-unknown-linux-musl:main"

[target.aarch64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:main"

[target.aarch64-unknown-linux-musl]
image = "ghcr.io/cross-rs/aarch64-unknown-linux-musl:main"

# Custom image with extra dependencies
[target.x86_64-unknown-linux-gnu.env]
passthrough = ["OPENSSL_DIR"]

# Build settings
[build]
# Default target if none specified
default-target = "x86_64-unknown-linux-gnu"
"#
    );

    println!();

    // =========================================================================
    // COMMON TARGETS
    // =========================================================================

    println!("--- Common Targets ---");
    println!(
        r#"
Popular cross-compilation targets:

| Target                        | OS      | Arch   | Notes              |
|-------------------------------|---------|--------|--------------------|
| x86_64-unknown-linux-gnu      | Linux   | x64    | Standard Linux     |
| x86_64-unknown-linux-musl     | Linux   | x64    | Static, portable   |
| aarch64-unknown-linux-gnu     | Linux   | ARM64  | Raspberry Pi, ARM  |
| aarch64-unknown-linux-musl    | Linux   | ARM64  | Static ARM         |
| x86_64-apple-darwin           | macOS   | x64    | Intel Mac*         |
| aarch64-apple-darwin          | macOS   | ARM64  | Apple Silicon*     |
| x86_64-pc-windows-gnu         | Windows | x64    | MinGW toolchain    |
| x86_64-pc-windows-msvc        | Windows | x64    | MSVC (Windows CI)* |

* = Cannot cross-compile with cross-rs, need native CI runner

Add targets to rustup:
  rustup target add x86_64-unknown-linux-musl
  rustup target add aarch64-unknown-linux-gnu
"#
    );

    println!();

    // =========================================================================
    // STATIC BINARIES
    // =========================================================================

    println!("--- Static Binaries (musl) ---");
    println!(
        r#"
Build fully static binaries with musl:

# Build static binary
cross build --target x86_64-unknown-linux-musl --release

# Verify it's static
file target/x86_64-unknown-linux-musl/release/dx
# Output: ELF 64-bit LSB executable, x86-64, statically linked

ldd target/x86_64-unknown-linux-musl/release/dx
# Output: not a dynamic executable

Benefits of static binaries:
  - No runtime dependencies
  - Works on any Linux distribution
  - Smaller container images (FROM scratch)
  - Easier deployment

Downsides:
  - Larger binary size
  - Some crates don't work with musl
  - No dynamic loading
"#
    );

    println!();

    // =========================================================================
    // CUSTOM DOCKER IMAGES
    // =========================================================================

    println!("--- Custom Docker Images ---");
    println!(
        r##"
Create custom cross images for special dependencies:

# Dockerfile.cross
FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main

# Install additional dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set environment
ENV OPENSSL_DIR=/usr

Build and use:

  docker build -t my-cross-image -f Dockerfile.cross .

# Cross.toml
[target.x86_64-unknown-linux-gnu]
image = "my-cross-image"

Or use pre-build hook:

[target.x86_64-unknown-linux-gnu]
pre-build = [
    "apt-get update",
    "apt-get install -y libssl-dev"
]
"##
    );

    println!();

    // =========================================================================
    // TROUBLESHOOTING
    // =========================================================================

    println!("--- Troubleshooting ---");
    println!(
        r#"
Common issues:

1. OPENSSL ERRORS
   Solution: Use vendored OpenSSL
   # Cargo.toml
   [dependencies]
   openssl = {{ version = "0.10", features = ["vendored"] }}

2. LINKING ERRORS
   Solution: Check if library is installed in container
   Or use Rust-native alternatives (rustls instead of openssl)

3. DOCKER PERMISSION ERRORS
   Solution: Add user to docker group
   sudo usermod -aG docker $USER

4. SLOW BUILDS
   Solution: Use Docker volume caching
   # Cross.toml
   [build]
   # Mount cargo registry as volume
   volumes = ["~/.cargo/registry:/root/.cargo/registry"]

5. BINARY TOO LARGE
   Solution: Strip and compress
   cross build --release
   strip target/<target>/release/dx
   upx target/<target>/release/dx  # Optional compression
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Cross-compilation with cross-rs:");
    println!("  1. Install: cargo install cross");
    println!("  2. Configure: Cross.toml");
    println!("  3. Build: cross build --target <target>");
    println!("  4. Use musl for static binaries");
    println!("  5. Custom images for special deps");
}
