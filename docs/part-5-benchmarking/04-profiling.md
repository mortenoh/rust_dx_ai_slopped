# Profiling with Flamegraph

Visualize where time is spent in your code.

## Setup

```bash
# Install flamegraph
cargo install flamegraph

# On Linux, allow perf
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

## Generate Flamegraph

```bash
cargo flamegraph --bin dx -- hash large_file.txt
```

Opens `flamegraph.svg` showing CPU time distribution.

## Profiling Benchmarks

```bash
cargo flamegraph --bench benchmarks -- --bench hash
```

## Reading Flamegraphs

- Width = time spent
- Stack grows upward
- Click to zoom
- Search for functions

## perf (Linux)

```bash
perf record cargo run --release -- hash file.txt
perf report
```

## Instruments (macOS)

```bash
cargo instruments -t time --bin dx -- hash file.txt
```

## DHAT (Memory Profiling)

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
```
