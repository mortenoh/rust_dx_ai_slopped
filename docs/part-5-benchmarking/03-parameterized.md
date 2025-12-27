# Parameterized Benchmarks

Benchmark with varying inputs.

## Basic Parameterization

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_sort");

    for size in [10, 100, 1000, 10000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let mut data: Vec<i32> = (0..size).rev().collect();
                b.iter(|| {
                    data.sort();
                    data.reverse();
                })
            },
        );
    }

    group.finish();
}
```

## Multiple Parameters

```rust
fn bench_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    for algorithm in ["base64", "hex"] {
        for size in [100, 1000] {
            let data = vec![0u8; size];
            let id = format!("{}/{}", algorithm, size);

            group.bench_with_input(
                BenchmarkId::new(algorithm, size),
                &(algorithm, &data),
                |b, (alg, data)| {
                    b.iter(|| encode(alg, data))
                },
            );
        }
    }

    group.finish();
}
```

## With Setup

```rust
fn bench_with_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_processing");

    for size in [1024, 4096] {
        group.bench_function(
            BenchmarkId::from_parameter(size),
            |b| {
                // Setup (not measured)
                let data = create_test_data(size);

                b.iter(|| {
                    // Only this is measured
                    process(&data)
                })
            },
        );
    }

    group.finish();
}
```

## Iterator-Based

```rust
fn bench_iter(c: &mut Criterion) {
    c.bench_function("collect_iter", |b| {
        b.iter_batched(
            || (0..1000).collect::<Vec<_>>(),  // Setup
            |data| data.iter().sum::<i32>(),   // Measured
            criterion::BatchSize::SmallInput,
        )
    });
}
```
