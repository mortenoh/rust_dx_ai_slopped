# Profiling and Analysis

Find and fix performance bottlenecks.

## CPU Profiling

### Flamegraph

```bash
cargo install flamegraph
```

```bash
# Generate flamegraph
cargo flamegraph --bin dx -- hash large_file.txt

# Open flamegraph.svg in browser
```

Reading flamegraphs:
- Width = time spent in function
- Stack grows upward (callers below callees)
- Click to zoom into subtree
- Search for function names

### perf (Linux)

```bash
# Record
perf record cargo run --release -- hash file.txt

# Analyze
perf report
```

### Instruments (macOS)

```bash
cargo instruments -t time --bin dx -- hash file.txt
```

## Memory Profiling

### DHAT

```toml
[dependencies]
dhat = { version = "0.3", optional = true }

[features]
dhat = ["dep:dhat"]
```

```rust
#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    // Your code
}
```

```bash
cargo run --features dhat --release
# Opens dh_view.html with analysis
```

### heaptrack (Linux)

```bash
heaptrack cargo run --release -- hash file.txt
heaptrack_gui heaptrack.*.gz
```

### Valgrind

```bash
valgrind --tool=massif cargo run --release -- hash file.txt
ms_print massif.out.*
```

## Binary Analysis

### cargo-bloat

```bash
cargo install cargo-bloat

# See what's in your binary
cargo bloat --release

# By crate
cargo bloat --release --crates

# Top functions
cargo bloat --release -n 20
```

### twiggy

```bash
cargo install twiggy

# Analyze binary
cargo build --release
twiggy top target/release/dx

# Dominators
twiggy dominators target/release/dx
```

## Benchmarking

### hyperfine

```bash
cargo install hyperfine

# Single command
hyperfine 'dx hash file.txt'

# Compare implementations
hyperfine 'dx-v1 hash file.txt' 'dx-v2 hash file.txt'

# With warmup
hyperfine --warmup 5 'dx hash file.txt'

# Export results
hyperfine --export-json results.json 'dx hash file.txt'
```

### Criterion

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "benchmarks"
harness = false
```

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.bench_function("hash_small", |b| {
        b.iter(|| hash(b"small data"))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
```

```bash
cargo bench
```

## Compile Time Analysis

```bash
# Build timings
cargo build --timings

# Opens cargo-timing.html
```

## Quick Profiling Workflow

1. **Measure baseline**
   ```bash
   hyperfine 'dx hash file.txt'
   ```

2. **Generate flamegraph**
   ```bash
   cargo flamegraph --bin dx -- hash file.txt
   ```

3. **Identify bottleneck** (wide bars in flamegraph)

4. **Optimize hot path**

5. **Verify improvement**
   ```bash
   hyperfine 'dx-old hash file.txt' 'dx-new hash file.txt'
   ```

## Continuous Benchmarking

```yaml
# .github/workflows/bench.yml
- name: Run benchmarks
  run: cargo bench -- --save-baseline new

- name: Compare to main
  run: cargo bench -- --baseline main --save-baseline new
```
