# Installation

## From crates.io

```bash
cargo install dx
```

## From Source

```bash
git clone https://github.com/user/dx
cd dx
cargo install --path .
```

## Pre-built Binaries

Download from [GitHub Releases](https://github.com/user/dx/releases):

| Platform | Download |
|----------|----------|
| Linux x64 | `dx-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `dx-aarch64-unknown-linux-gnu.tar.gz` |
| macOS x64 | `dx-x86_64-apple-darwin.tar.gz` |
| macOS ARM64 | `dx-aarch64-apple-darwin.tar.gz` |
| Windows x64 | `dx-x86_64-pc-windows-msvc.zip` |

## Homebrew (macOS/Linux)

```bash
brew tap user/tap
brew install dx
```

## Shell Completions

```bash
# Bash
dx completions bash >> ~/.bashrc

# Zsh
dx completions zsh > ~/.zfunc/_dx

# Fish
dx completions fish > ~/.config/fish/completions/dx.fish
```
