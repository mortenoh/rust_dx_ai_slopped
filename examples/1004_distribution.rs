//! # Distribution Channels
//!
//! This example shows how to distribute your CLI tool.
//!
//! Run with: `cargo run --example 1004_distribution`

#![allow(dead_code)]

fn main() {
    println!("=== Distribution Channels ===\n");

    // =========================================================================
    // CRATES.IO
    // =========================================================================

    println!("--- Crates.io ---");
    println!(
        r#"
Publish to crates.io:

# Cargo.toml
[package]
name = "dx"
version = "1.0.0"
edition = "2021"
description = "Developer toolkit CLI"
license = "MIT"
repository = "https://github.com/user/dx"
homepage = "https://dx.example.com"
readme = "README.md"
keywords = ["cli", "developer", "tools"]
categories = ["command-line-utilities"]

# Exclude files from package
exclude = [
    "tests/*",
    "benches/*",
    ".github/*",
]

Publish:
  cargo login <token>
  cargo publish --dry-run  # Test first
  cargo publish

Install:
  cargo install dx
"#
    );

    println!();

    // =========================================================================
    // CARGO BINSTALL
    // =========================================================================

    println!("--- Cargo Binstall ---");
    println!(
        r#"
Support pre-built binary installation:

# Cargo.toml
[package.metadata.binstall]
pkg-url = "{{ repo }}/releases/download/v{{ version }}/dx-{{ target }}{{ archive-suffix }}"
pkg-fmt = "tgz"

[package.metadata.binstall.overrides.x86_64-pc-windows-msvc]
pkg-fmt = "zip"

[package.metadata.binstall.overrides.aarch64-apple-darwin]
pkg-url = "{{ repo }}/releases/download/v{{ version }}/dx-aarch64-apple-darwin.tar.gz"

Users install with:
  cargo binstall dx

Benefits:
  - Downloads pre-built binary (fast!)
  - Falls back to source build if no binary
  - Works with existing cargo workflow
"#
    );

    println!();

    // =========================================================================
    // HOMEBREW
    // =========================================================================

    println!("--- Homebrew ---");
    println!(
        r##"
Create a Homebrew tap:

1. Create tap repository:
   github.com/user/homebrew-tap

2. Add formula:

# Formula/dx.rb
class Dx < Formula
  desc "Developer toolkit CLI"
  homepage "https://github.com/user/dx"
  version "1.0.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/user/dx/releases/download/v1.0.0/dx-aarch64-apple-darwin.tar.gz"
      sha256 "..."
    end
    on_intel do
      url "https://github.com/user/dx/releases/download/v1.0.0/dx-x86_64-apple-darwin.tar.gz"
      sha256 "..."
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/user/dx/releases/download/v1.0.0/dx-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "..."
    end
    on_intel do
      url "https://github.com/user/dx/releases/download/v1.0.0/dx-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "..."
    end
  end

  def install
    bin.install "dx"
    generate_completions_from_executable(bin/"dx", "completions")
  end

  test do
    assert_match "dx #{{version}}", shell_output("#{{bin}}/dx --version")
  end
end

Install:
  brew tap user/tap
  brew install dx
"##
    );

    println!();

    // =========================================================================
    // APT/DEBIAN
    // =========================================================================

    println!("--- APT/Debian ---");
    println!(
        r#"
Create Debian packages and repository:

1. Build .deb with cargo-deb:
   cargo install cargo-deb
   cargo deb

2. Create APT repository:

# Setup GPG key
gpg --armor --export you@example.com > dx.gpg.key

# Create repository structure
mkdir -p repo/pool/main
mkdir -p repo/dists/stable/main/binary-amd64

# Add packages
cp dx_1.0.0_amd64.deb repo/pool/main/

# Generate metadata
cd repo
dpkg-scanpackages pool/main > dists/stable/main/binary-amd64/Packages
gzip -k dists/stable/main/binary-amd64/Packages

3. Create Release file (sign with GPG)

4. Host on GitHub Pages or S3

Install:
  curl -fsSL https://dx.example.com/dx.gpg.key | sudo apt-key add -
  echo "deb https://dx.example.com/repo stable main" | sudo tee /etc/apt/sources.list.d/dx.list
  sudo apt update && sudo apt install dx
"#
    );

    println!();

    // =========================================================================
    // CONTAINER IMAGES
    // =========================================================================

    println!("--- Container Images ---");
    println!(
        r#"
Publish Docker images:

# Dockerfile
FROM scratch
COPY target/x86_64-unknown-linux-musl/release/dx /dx
ENTRYPOINT ["/dx"]

# Multi-arch with buildx
docker buildx create --use
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  --tag ghcr.io/user/dx:1.0.0 \
  --tag ghcr.io/user/dx:latest \
  --push .

# GitHub Actions
- name: Build and push
  uses: docker/build-push-action@v5
  with:
    context: .
    platforms: linux/amd64,linux/arm64
    push: true
    tags: |
      ghcr.io/${{{{ github.repository }}}}:${{{{ github.ref_name }}}}
      ghcr.io/${{{{ github.repository }}}}:latest

Use:
  docker run --rm ghcr.io/user/dx:latest hash file.txt
"#
    );

    println!();

    // =========================================================================
    // WINDOWS
    // =========================================================================

    println!("--- Windows Distribution ---");
    println!(
        r#"
Windows-specific distribution:

1. WINGET (Windows Package Manager)

# manifests/u/user/dx/1.0.0/user.dx.yaml
PackageIdentifier: user.dx
PackageVersion: 1.0.0
PackageName: dx
Publisher: Your Name
License: MIT
ShortDescription: Developer toolkit CLI
Installers:
  - Architecture: x64
    InstallerUrl: https://github.com/user/dx/releases/download/v1.0.0/dx-x86_64-pc-windows-msvc.zip
    InstallerSha256: ...
    InstallerType: zip
ManifestType: singleton
ManifestVersion: 1.0.0

Submit PR to microsoft/winget-pkgs

Install:
  winget install user.dx

2. SCOOP

# bucket/dx.json
{{
  "version": "1.0.0",
  "url": "https://github.com/user/dx/releases/download/v1.0.0/dx-x86_64-pc-windows-msvc.zip",
  "hash": "...",
  "bin": "dx.exe"
}}

Install:
  scoop bucket add user https://github.com/user/scoop-bucket
  scoop install dx

3. CHOCOLATEY
   Create .nuspec package
   Submit to community repository
"#
    );

    println!();

    // =========================================================================
    // INSTALL SCRIPT
    // =========================================================================

    println!("--- Universal Install Script ---");
    println!(
        r#"
One-liner installation:

# Unix
curl -sSfL https://dx.example.com/install.sh | sh

# Windows PowerShell
iwr https://dx.example.com/install.ps1 -useb | iex

The script should:
  - Detect OS and architecture
  - Download appropriate binary
  - Verify checksum
  - Install to appropriate location
  - Add to PATH if needed
  - Show next steps

Example output:
  $ curl -sSfL https://dx.example.com/install.sh | sh

  Downloading dx v1.0.0 for linux-x64...
  Verifying checksum... OK
  Installing to /usr/local/bin/dx... OK

  dx has been installed!

  To get started:
    dx --help

  To enable shell completions:
    dx completions bash >> ~/.bashrc
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Distribution channels:");
    println!("  1. crates.io for Rust users");
    println!("  2. cargo-binstall for fast binary install");
    println!("  3. Homebrew for macOS/Linux");
    println!("  4. APT/Debian for Linux distros");
    println!("  5. WinGet/Scoop for Windows");
    println!("  6. Docker for containerized use");
    println!("  7. Install script for universal access");
}
