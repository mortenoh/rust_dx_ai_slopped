//! # Artifact Packaging
//!
//! This example shows how to package release artifacts.
//!
//! Run with: `cargo run --example 0904_artifact_packaging`

#![allow(dead_code)]

fn main() {
    println!("=== Artifact Packaging ===\n");

    // =========================================================================
    // ARCHIVE FORMATS
    // =========================================================================

    println!("--- Archive Formats ---");
    println!(
        r#"
Standard archive formats:

UNIX (tar.gz):
  tar czvf dx-v1.0.0-linux-x64.tar.gz \
    -C target/release \
    dx \
    -C ../../ \
    README.md LICENSE

WINDOWS (zip):
  7z a dx-v1.0.0-windows-x64.zip \
    target/release/dx.exe \
    README.md LICENSE

Archive contents:
  dx-v1.0.0-linux-x64/
  ├── dx           # Binary
  ├── README.md    # Documentation
  ├── LICENSE      # License file
  └── completions/ # Shell completions (optional)
      ├── dx.bash
      ├── dx.zsh
      ├── dx.fish
      └── _dx.ps1
"#
    );

    println!();

    // =========================================================================
    // PACKAGING SCRIPT
    // =========================================================================

    println!("--- Packaging Script ---");
    println!(
        r##"
#!/bin/bash
# scripts/package.sh

set -euo pipefail

VERSION="${{1:-$(cargo metadata --format-version 1 | jq -r '.packages[0].version')}}"
TARGET="${{2:-x86_64-unknown-linux-gnu}}"
BINARY="dx"
ARCHIVE_DIR="dist"

# Determine archive format
case "$TARGET" in
    *-windows-*) EXT="zip" ;;
    *) EXT="tar.gz" ;;
esac

# Create staging directory
STAGING="$ARCHIVE_DIR/staging/$BINARY-v$VERSION-$TARGET"
mkdir -p "$STAGING"

# Copy binary
if [[ "$TARGET" == *-windows-* ]]; then
    cp "target/$TARGET/release/$BINARY.exe" "$STAGING/"
else
    cp "target/$TARGET/release/$BINARY" "$STAGING/"
fi

# Copy documentation
cp README.md LICENSE CHANGELOG.md "$STAGING/"

# Generate completions
mkdir -p "$STAGING/completions"
"$STAGING/$BINARY" completions bash > "$STAGING/completions/$BINARY.bash"
"$STAGING/$BINARY" completions zsh > "$STAGING/completions/$BINARY.zsh"
"$STAGING/$BINARY" completions fish > "$STAGING/completions/$BINARY.fish"

# Create archive
cd "$ARCHIVE_DIR"
ARCHIVE_NAME="$BINARY-v$VERSION-$TARGET.$EXT"

if [[ "$EXT" == "zip" ]]; then
    7z a "$ARCHIVE_NAME" "staging/$BINARY-v$VERSION-$TARGET"
else
    tar czvf "$ARCHIVE_NAME" -C staging "$BINARY-v$VERSION-$TARGET"
fi

# Generate checksum
sha256sum "$ARCHIVE_NAME" > "$ARCHIVE_NAME.sha256"

echo "Created: $ARCHIVE_DIR/$ARCHIVE_NAME"
"##
    );

    println!();

    // =========================================================================
    // INSTALL SCRIPTS
    // =========================================================================

    println!("--- Install Scripts ---");
    println!(
        r##"
Generate installation scripts:

# install.sh (Unix)
#!/bin/sh
set -e

REPO="user/dx"
VERSION="${{DX_VERSION:-latest}}"
INSTALL_DIR="${{DX_INSTALL_DIR:-/usr/local/bin}}"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${{ARCH}}-unknown-${{OS}}-gnu"
if [ "$OS" = "darwin" ]; then
    TARGET="${{ARCH}}-apple-darwin"
fi

# Download and install
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/dx-$TARGET.tar.gz"
echo "Downloading dx from $DOWNLOAD_URL..."

curl -sSL "$DOWNLOAD_URL" | tar xz -C /tmp
install -m 755 /tmp/dx "$INSTALL_DIR/dx"

echo "Installed dx to $INSTALL_DIR/dx"
dx --version

# Usage:
# curl -sSL https://raw.githubusercontent.com/user/dx/main/install.sh | sh
"##
    );

    println!();

    // =========================================================================
    // DEBIAN PACKAGE
    // =========================================================================

    println!("--- Debian Package ---");
    println!(
        r#"
Create .deb package:

# Use cargo-deb
cargo install cargo-deb

# Cargo.toml
[package.metadata.deb]
maintainer = "Your Name <you@example.com>"
copyright = "2024, Your Name"
license-file = ["LICENSE", "0"]
extended-description = """\
A comprehensive developer toolkit CLI."""
section = "utils"
priority = "optional"
assets = [
    ["target/release/dx", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/dx/", "644"],
    ["completions/dx.bash", "usr/share/bash-completion/completions/dx", "644"],
    ["completions/dx.zsh", "usr/share/zsh/site-functions/_dx", "644"],
]

Build:
  cargo deb --target x86_64-unknown-linux-gnu

Output: target/debian/dx_1.0.0_amd64.deb

Install:
  sudo dpkg -i dx_1.0.0_amd64.deb
"#
    );

    println!();

    // =========================================================================
    // RPM PACKAGE
    // =========================================================================

    println!("--- RPM Package ---");
    println!(
        r#"
Create .rpm package:

# Use cargo-rpm
cargo install cargo-generate-rpm

# Cargo.toml
[package.metadata.generate-rpm]
assets = [
    {{ source = "target/release/dx", dest = "/usr/bin/dx", mode = "755" }},
    {{ source = "README.md", dest = "/usr/share/doc/dx/README.md", mode = "644" }},
]

Build:
  cargo build --release
  cargo generate-rpm

Output: target/generate-rpm/dx-1.0.0-1.x86_64.rpm

Install:
  sudo rpm -i dx-1.0.0-1.x86_64.rpm
"#
    );

    println!();

    // =========================================================================
    // HOMEBREW
    // =========================================================================

    println!("--- Homebrew Formula ---");
    println!(
        r##"
Create Homebrew formula:

# Formula/dx.rb
class Dx < Formula
  desc "Developer toolkit CLI"
  homepage "https://github.com/user/dx"
  version "1.0.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/user/dx/releases/download/v1.0.0/dx-aarch64-apple-darwin.tar.gz"
      sha256 "abc123..."
    end
    on_intel do
      url "https://github.com/user/dx/releases/download/v1.0.0/dx-x86_64-apple-darwin.tar.gz"
      sha256 "def456..."
    end
  end

  on_linux do
    url "https://github.com/user/dx/releases/download/v1.0.0/dx-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "ghi789..."
  end

  def install
    bin.install "dx"
    generate_completions_from_executable(bin/"dx", "completions")
  end

  test do
    assert_match "dx #{{version}}", shell_output("#{{bin}}/dx --version")
  end
end

Publish:
  1. Create tap repository: github.com/user/homebrew-tap
  2. Add formula to tap
  3. Install: brew install user/tap/dx
"##
    );

    println!();

    // =========================================================================
    // CARGO BINSTALL
    // =========================================================================

    println!("--- Cargo Binstall ---");
    println!(
        r#"
Support cargo-binstall:

# Cargo.toml
[package.metadata.binstall]
pkg-url = "{{ repo }}/releases/download/v{{ version }}/dx-{{ target }}{{ archive-suffix }}"
bin-dir = "dx-v{{ version }}-{{ target }}/dx{{ binary-ext }}"
pkg-fmt = "tgz"

[package.metadata.binstall.overrides.x86_64-pc-windows-msvc]
pkg-fmt = "zip"

Users can install with:
  cargo binstall dx

This downloads pre-built binary instead of compiling.
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Artifact packaging:");
    println!("  1. Use tar.gz for Unix, zip for Windows");
    println!("  2. Include README, LICENSE, completions");
    println!("  3. Generate install scripts");
    println!("  4. Create native packages (deb, rpm)");
    println!("  5. Support Homebrew and cargo-binstall");
}
