//! # Release Workflow
//!
//! This example shows how to automate releases with GitHub Actions.
//!
//! Run with: `cargo run --example 0903_release_workflow`

#![allow(dead_code)]

fn main() {
    println!("=== Release Workflow ===\n");

    // =========================================================================
    // RELEASE WORKFLOW
    // =========================================================================

    println!("--- Release Workflow ---");
    println!(
        r#"
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v[0-9]+.*'

permissions:
  contents: write

env:
  CARGO_TERM_COLOR: always

jobs:
  create-release:
    runs-on: ubuntu-latest
    outputs:
      upload_url: ${{{{ steps.create_release.outputs.upload_url }}}}
    steps:
      - name: Create Release
        id: create_release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{{{ secrets.GITHUB_TOKEN }}}}
        with:
          tag_name: ${{{{ github.ref_name }}}}
          release_name: ${{{{ github.ref_name }}}}
          draft: true
          prerelease: ${{{{ contains(github.ref_name, '-') }}}}

  build-release:
    needs: create-release
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            archive: tar.gz
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            archive: tar.gz
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            archive: tar.gz
          - os: macos-latest
            target: x86_64-apple-darwin
            archive: tar.gz
          - os: macos-latest
            target: aarch64-apple-darwin
            archive: tar.gz
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            archive: zip

    runs-on: ${{{{ matrix.os }}}}

    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{{{ matrix.target }}}}

      - name: Build (cross)
        if: matrix.os == 'ubuntu-latest' && matrix.target != 'x86_64-unknown-linux-gnu'
        run: |
          cargo install cross --git https://github.com/cross-rs/cross
          cross build --release --target ${{{{ matrix.target }}}}

      - name: Build (native)
        if: matrix.target == 'x86_64-unknown-linux-gnu' || matrix.os != 'ubuntu-latest'
        run: cargo build --release --target ${{{{ matrix.target }}}}

      - name: Package
        shell: bash
        run: |
          cd target/${{{{ matrix.target }}}}/release
          if [ "${{{{ matrix.os }}}}" = "windows-latest" ]; then
            7z a ../../../dx-${{{{ matrix.target }}}}.zip dx.exe
          else
            tar czvf ../../../dx-${{{{ matrix.target }}}}.tar.gz dx
          fi

      - name: Upload
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{{{ secrets.GITHUB_TOKEN }}}}
        with:
          upload_url: ${{{{ needs.create-release.outputs.upload_url }}}}
          asset_path: dx-${{{{ matrix.target }}}}.${{{{ matrix.archive }}}}
          asset_name: dx-${{{{ matrix.target }}}}.${{{{ matrix.archive }}}}
          asset_content_type: application/octet-stream
"#
    );

    println!();

    // =========================================================================
    // MODERN RELEASE ACTION
    // =========================================================================

    println!("--- Modern Release with cargo-dist ---");
    println!(
        r#"
Alternative: Use cargo-dist for automated releases

Install:
  cargo install cargo-dist

Initialize:
  cargo dist init

This generates:
  - .github/workflows/release.yml
  - Cargo.toml [dist] section

# Cargo.toml
[workspace.metadata.dist]
# CI to use
ci = ["github"]

# Targets to build
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
]

# Installers to generate
installers = ["shell", "powershell", "homebrew"]

# Include in archives
include = ["README.md", "LICENSE", "CHANGELOG.md"]

Create release:
  git tag v1.0.0
  git push --tags

cargo-dist automatically:
  - Cross-compiles for all targets
  - Creates archives with binaries
  - Generates install scripts
  - Creates GitHub release
  - Updates Homebrew formula
"#
    );

    println!();

    // =========================================================================
    // CHECKSUMS
    // =========================================================================

    println!("--- Checksums ---");
    println!(
        r#"
Generate checksums for releases:

jobs:
  checksums:
    needs: build-release
    runs-on: ubuntu-latest
    steps:
      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Generate checksums
        run: |
          cd artifacts
          sha256sum */dx-* > checksums-sha256.txt
          cat checksums-sha256.txt

      - name: Upload checksums
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{{{ secrets.GITHUB_TOKEN }}}}
        with:
          upload_url: ${{{{ needs.create-release.outputs.upload_url }}}}
          asset_path: artifacts/checksums-sha256.txt
          asset_name: checksums-sha256.txt
          asset_content_type: text/plain

Verify downloads:
  sha256sum -c checksums-sha256.txt
"#
    );

    println!();

    // =========================================================================
    // CHANGELOG
    // =========================================================================

    println!("--- Automated Changelog ---");
    println!(
        r#"
Generate changelog from commits:

jobs:
  changelog:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Generate changelog
        uses: orhun/git-cliff-action@v3
        with:
          config: cliff.toml
          args: --latest --strip header
        env:
          OUTPUT: CHANGELOG.md

      - name: Create release with changelog
        uses: softprops/action-gh-release@v2
        with:
          body_path: CHANGELOG.md
          files: |
            dx-*.tar.gz
            dx-*.zip
            checksums-sha256.txt

# cliff.toml (git-cliff config)
[changelog]
header = \"\"\"
# Changelog\n
\"\"\"
body = \"\"\"
{{%- for group, commits in commits | group_by(attribute=\"group\") %}}
## {{{{{{ group | upper_first }}}}}}
{{%- for commit in commits %}}
- {{{{{{ commit.message | upper_first }}}}}}
{{%- endfor %}}
{{%- endfor %}}
\"\"\"

[git]
conventional_commits = true
commit_parsers = [
    {{{{ message = \"^feat\", group = \"Features\" }}}},
    {{{{ message = \"^fix\", group = \"Bug Fixes\" }}}},
    {{{{ message = \"^doc\", group = \"Documentation\" }}}},
]
"#
    );

    println!();

    // =========================================================================
    // RELEASE PROCESS
    // =========================================================================

    println!("--- Release Process ---");
    println!(
        r#"
Manual release process:

1. Update version in Cargo.toml
   version = "1.2.0"

2. Update CHANGELOG.md

3. Commit changes
   git add -A
   git commit -m "chore: release v1.2.0"

4. Create and push tag
   git tag v1.2.0
   git push origin main --tags

5. CI automatically:
   - Runs tests
   - Builds for all targets
   - Creates draft release
   - Uploads binaries

6. Review and publish release on GitHub

Semantic versioning:
  v1.0.0 - First stable release
  v1.0.1 - Patch (bug fixes)
  v1.1.0 - Minor (new features, backwards compatible)
  v2.0.0 - Major (breaking changes)
  v1.0.0-beta.1 - Pre-release
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Release workflow:");
    println!("  1. Trigger on version tags (v*)");
    println!("  2. Cross-compile for all targets");
    println!("  3. Create archives and checksums");
    println!("  4. Generate changelog");
    println!("  5. Create GitHub release");
}
