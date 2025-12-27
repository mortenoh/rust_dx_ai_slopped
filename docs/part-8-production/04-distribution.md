# Distribution

Get your CLI into users' hands.

## Cargo Install

Publish to crates.io:

```bash
cargo publish
```

Users install with:
```bash
cargo install dx
```

### Cargo.toml for Publishing

```toml
[package]
name = "dx"
version = "1.0.0"
edition = "2021"
description = "Developer CLI tools"
license = "MIT"
repository = "https://github.com/user/dx"
homepage = "https://github.com/user/dx"
documentation = "https://docs.rs/dx"
readme = "README.md"
keywords = ["cli", "developer-tools", "hash"]
categories = ["command-line-utilities"]

[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/v{ version }/{ name }-{ target }{ archive-suffix }"
```

## GitHub Releases

### Release Workflow

```yaml
name: Release

on:
  push:
    tags: ['v*']

permissions:
  contents: write

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Build
        run: |
          rustup target add ${{ matrix.target }}
          cargo build --release --target ${{ matrix.target }}

      - name: Package (Unix)
        if: matrix.os != 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          tar czf ../../../dx-${{ matrix.target }}.tar.gz dx

      - name: Package (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          7z a ../../../dx-${{ matrix.target }}.zip dx.exe

      - uses: softprops/action-gh-release@v1
        with:
          files: dx-*
```

## Homebrew

Create a formula:

```ruby
# Formula/dx.rb
class Dx < Formula
  desc "Developer CLI tools"
  homepage "https://github.com/user/dx"
  url "https://github.com/user/dx/archive/v1.0.0.tar.gz"
  sha256 "..."
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "1.0.0", shell_output("#{bin}/dx --version")
  end
end
```

Users install with:
```bash
brew install user/tap/dx
```

## cargo-binstall

Enable binary installation:

```toml
[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/v{ version }/{ name }-{ target }.tar.gz"
pkg-fmt = "tgz"
```

Users install with:
```bash
cargo binstall dx
```

## Install Script

```bash
#!/bin/bash
# install.sh

set -e

REPO="user/dx"
VERSION="${1:-latest}"

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  x86_64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
esac

case "$OS" in
  linux) TARGET="${ARCH}-unknown-linux-musl" ;;
  darwin) TARGET="${ARCH}-apple-darwin" ;;
esac

if [ "$VERSION" = "latest" ]; then
  URL="https://github.com/${REPO}/releases/latest/download/dx-${TARGET}.tar.gz"
else
  URL="https://github.com/${REPO}/releases/download/${VERSION}/dx-${TARGET}.tar.gz"
fi

curl -fsSL "$URL" | tar xz -C /usr/local/bin

echo "dx installed successfully!"
```

```bash
curl -fsSL https://raw.githubusercontent.com/user/dx/main/install.sh | bash
```

## Package Managers

### APT (Debian/Ubuntu)

Create `.deb` package with `cargo-deb`:

```bash
cargo install cargo-deb
cargo deb
```

### RPM (Fedora/RHEL)

Create `.rpm` with `cargo-generate-rpm`:

```bash
cargo install cargo-generate-rpm
cargo build --release
cargo generate-rpm
```
