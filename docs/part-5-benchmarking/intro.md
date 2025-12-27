# Introduction to Benchmarking

Benchmarking measures code performance to guide optimization.

## Why Benchmark?

- Identify bottlenecks
- Compare implementations
- Track performance regressions
- Validate optimizations

## Tools

| Tool | Use Case |
|------|----------|
| Criterion | Statistical benchmarking |
| cargo bench | Built-in (nightly) |
| hyperfine | CLI timing |
| flamegraph | CPU profiling |

## Setup

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

## Running Benchmarks

```bash
cargo bench                    # All benchmarks
cargo bench -- hash            # Filter by name
cargo bench -- --save-baseline main  # Save baseline
cargo bench -- --baseline main       # Compare to baseline
```
