# Introduction to Optimization

Make your CLI faster, smaller, and more efficient.

## Optimization Goals

| Goal | Metric | Tool |
|------|--------|------|
| Faster startup | Time to first output | `hyperfine` |
| Smaller binary | File size in bytes | `cargo bloat` |
| Less memory | Peak RSS | `heaptrack` |
| Faster builds | Compile time | `cargo build --timings` |

## The Optimization Process

1. **Measure** - Establish baseline
2. **Profile** - Find bottlenecks
3. **Optimize** - Fix the hot path
4. **Verify** - Confirm improvement
5. **Repeat** - Until satisfied

## Quick Wins

### Release Mode

```bash
# Debug: slow, large
cargo build

# Release: fast, small
cargo build --release
```

### LTO (Link-Time Optimization)

```toml
[profile.release]
lto = true
```

### Strip Symbols

```toml
[profile.release]
strip = true
```

## What You'll Learn

| Chapter | Topic |
|---------|-------|
| 1 | Release build configuration |
| 2 | Binary size optimization |
| 3 | Runtime speed optimization |
| 4 | Memory optimization |
| 5 | Compile time improvement |
| 6 | Profiling and analysis |

## Typical Results

```
Before optimization:
  Binary: 15 MB
  Startup: 50ms
  Memory: 20 MB

After optimization:
  Binary: 2 MB   (-87%)
  Startup: 5ms   (-90%)
  Memory: 8 MB   (-60%)
```

## Essential Tools

```bash
# Benchmarking
cargo install hyperfine
cargo install criterion

# Binary analysis
cargo install cargo-bloat
cargo install cargo-binutils

# Profiling
cargo install flamegraph
cargo install cargo-instruments  # macOS
```

## Trade-offs

| Optimization | Benefit | Cost |
|--------------|---------|------|
| LTO | Smaller, faster | Slower compile |
| Strip | Smaller | No symbols |
| opt-level=z | Smaller | Slower |
| opt-level=3 | Faster | Larger |
