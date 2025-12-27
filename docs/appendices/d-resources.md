# Resources

Books, websites, and tools for Rust CLI development.

## Official Documentation

- [The Rust Book](https://doc.rust-lang.org/book/) - Start here
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rust Reference](https://doc.rust-lang.org/reference/) - Language specification
- [Standard Library](https://doc.rust-lang.org/std/) - API reference
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Package manager

## CLI-Specific

- [Command Line Applications in Rust](https://rust-cli.github.io/book/) - The CLI book
- [clap Documentation](https://docs.rs/clap/) - Argument parser
- [clap Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)

## Books

- **The Rust Programming Language** - Official book
- **Programming Rust** - O'Reilly, comprehensive
- **Rust in Action** - Systems programming focus
- **Zero To Production in Rust** - Web/backend focus
- **Command-Line Rust** - CLI exercises

## Websites

- [crates.io](https://crates.io/) - Package registry
- [docs.rs](https://docs.rs/) - Documentation hosting
- [lib.rs](https://lib.rs/) - Curated crate search
- [This Week in Rust](https://this-week-in-rust.org/) - Weekly newsletter
- [Rust Blog](https://blog.rust-lang.org/) - Official blog

## Community

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://reddit.com/r/rust)
- [Rust Zulip](https://rust-lang.zulipchat.com/)

## Tools

### Development

| Tool | Purpose |
|------|---------|
| `rustup` | Toolchain manager |
| `rust-analyzer` | IDE support |
| `cargo-watch` | Auto-rebuild |
| `bacon` | Background checker |

### Quality

| Tool | Purpose |
|------|---------|
| `clippy` | Linter |
| `rustfmt` | Formatter |
| `cargo-audit` | Security audit |
| `cargo-deny` | License/dependency checks |

### Performance

| Tool | Purpose |
|------|---------|
| `flamegraph` | CPU profiling |
| `hyperfine` | Benchmarking |
| `cargo-bloat` | Binary size |
| `heaptrack` | Memory profiling |

## IDE Setup

### VS Code

```json
// Recommended extensions
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "serayuzgur.crates",
    "vadimcn.vscode-lldb"
  ]
}
```

### Neovim

```lua
-- Using lazy.nvim
{
  "neovim/nvim-lspconfig",
  config = function()
    require("lspconfig").rust_analyzer.setup{}
  end
}
```

## Example Projects

Learn from well-written CLIs:

- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast grep
- [bat](https://github.com/sharkdp/bat) - cat with wings
- [fd](https://github.com/sharkdp/fd) - Fast find
- [exa/eza](https://github.com/eza-community/eza) - Modern ls
- [starship](https://github.com/starship/starship) - Shell prompt
- [zoxide](https://github.com/ajeetdsouza/zoxide) - Smart cd
- [delta](https://github.com/dandavison/delta) - Diff viewer
- [bottom](https://github.com/ClementTsang/bottom) - System monitor

## Cheat Sheets

- [Rust Cheat Sheet](https://cheats.rs/)
- [Rust Container Cheat Sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/)

## Staying Updated

```bash
# Update Rust
rustup update

# Check for new crate versions
cargo outdated

# Security advisories
cargo audit
```
