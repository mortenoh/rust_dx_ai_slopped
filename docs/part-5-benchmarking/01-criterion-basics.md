# Criterion Basics

Criterion provides statistical benchmarking for Rust.

## Setup

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmarks"
harness = false
```

## Basic Benchmark

```rust
// benches/benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

## With Input

```rust
use criterion::{black_box, Criterion};

fn bench_hash(c: &mut Criterion) {
    let data = "hello world".repeat(1000);

    c.bench_function("sha256", |b| {
        b.iter(|| compute_hash(black_box(&data)))
    });
}
```

## Output

```
sha256                  time:   [1.2345 µs 1.2567 µs 1.2789 µs]
                        change: [-2.1234% +0.5678% +3.4567%] (p = 0.12)
```

## Reports

Open `target/criterion/report/index.html` for HTML reports with graphs.
