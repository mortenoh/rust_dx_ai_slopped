# mdbook Setup

Set up mdbook for your CLI documentation.

## Installation

```bash
cargo install mdbook
```

## Initialize Project

```bash
mdbook init docs
```

Creates:
```
docs/
├── book.toml      # Configuration
├── src/
│   ├── SUMMARY.md # Table of contents
│   └── chapter_1.md
```

## Configuration (book.toml)

```toml
[book]
title = "dx - Developer CLI Tools"
authors = ["Your Name"]
description = "A collection of developer utilities"
language = "en"

[build]
build-dir = "book"

[output.html]
default-theme = "rust"
preferred-dark-theme = "ayu"
git-repository-url = "https://github.com/user/dx"
edit-url-template = "https://github.com/user/dx/edit/main/docs/{path}"

[output.html.search]
enable = true
limit-results = 30
use-hierarchical-headings = true

[output.html.fold]
enable = true
level = 1
```

## SUMMARY.md Structure

```markdown
# Summary

[Introduction](./intro.md)

# User Guide

- [Installation](./guide/install.md)
- [Quick Start](./guide/quickstart.md)
- [Commands](./commands/README.md)
    - [hash](./commands/hash.md)
    - [encode](./commands/encode.md)

# Reference

- [Configuration](./reference/config.md)
- [Environment Variables](./reference/env.md)

-----------

[Contributing](./contributing.md)
[Changelog](./changelog.md)
```

## Build and Serve

```bash
# Build static site
mdbook build docs/

# Serve with live reload
mdbook serve docs/ --open

# Watch for changes
mdbook watch docs/
```

## Directory Layout

```
project/
├── Cargo.toml
├── src/
│   └── main.rs
├── docs/
│   ├── book.toml
│   └── src/
│       ├── SUMMARY.md
│       ├── intro.md
│       └── commands/
│           ├── README.md
│           └── hash.md
└── README.md
```

## CI Integration

```yaml
# .github/workflows/docs.yml
name: Deploy docs

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install mdbook
        run: cargo install mdbook

      - name: Build docs
        run: mdbook build docs/

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/book
```
