# Versioning

Use semantic versioning for predictable updates.

## Semantic Versioning

```
MAJOR.MINOR.PATCH

1.0.0
│ │ │
│ │ └── Bug fixes (backwards compatible)
│ └──── New features (backwards compatible)
└────── Breaking changes
```

## Version in Code

```rust
#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

```bash
dx --version
# dx 1.2.3
```

## Extended Version Info

```rust
fn version_info() -> String {
    format!(
        "{} {}\n\
         Built: {}\n\
         Commit: {}\n\
         Target: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_DATE"),
        env!("GIT_HASH"),
        env!("TARGET"),
    )
}

// In build.rs
fn main() {
    println!("cargo:rustc-env=BUILD_DATE={}", chrono::Utc::now().format("%Y-%m-%d"));
    println!("cargo:rustc-env=GIT_HASH={}",
        std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    );
    println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());
}
```

## Changelog

Maintain a `CHANGELOG.md`:

```markdown
# Changelog

## [Unreleased]

## [1.2.0] - 2024-03-15

### Added
- New `json` command for JSON formatting
- Shell completion for fish

### Changed
- Improved error messages for file operations

### Fixed
- Hash command now handles symlinks correctly

## [1.1.0] - 2024-02-01

### Added
- Base64 URL-safe encoding option
```

## Release Automation

### cargo-release

```bash
cargo install cargo-release
```

```toml
# Cargo.toml
[package.metadata.release]
sign-commit = true
sign-tag = true
pre-release-commit-message = "chore: release {{version}}"
tag-message = "{{version}}"
```

```bash
# Bump patch version
cargo release patch

# Bump minor version
cargo release minor

# Bump major version
cargo release major
```

### Git Tags

```bash
git tag -a v1.2.3 -m "Release 1.2.3"
git push origin v1.2.3
```

## Pre-release Versions

```toml
version = "1.3.0-alpha.1"
version = "1.3.0-beta.2"
version = "1.3.0-rc.1"
```

## Version Check Command

```rust
#[derive(Subcommand)]
enum Commands {
    /// Check for updates
    Update {
        /// Check only, don't install
        #[arg(long)]
        check: bool,
    },
}

async fn check_update() -> Result<Option<String>> {
    let current = env!("CARGO_PKG_VERSION");
    let latest = fetch_latest_version().await?;

    if semver::Version::parse(&latest)? > semver::Version::parse(current)? {
        Ok(Some(latest))
    } else {
        Ok(None)
    }
}
```

## Breaking Changes

Document in changelog and consider:

```rust
#[deprecated(since = "1.2.0", note = "Use `new_function` instead")]
pub fn old_function() {}
```
