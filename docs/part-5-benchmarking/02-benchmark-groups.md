# Benchmark Groups

Group related benchmarks for comparison.

## Creating Groups

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_algorithms(c: &mut Criterion) {
    let data = "test data".to_string();

    let mut group = c.benchmark_group("hashing");

    group.bench_function("md5", |b| {
        b.iter(|| hash_md5(&data))
    });

    group.bench_function("sha256", |b| {
        b.iter(|| hash_sha256(&data))
    });

    group.bench_function("sha512", |b| {
        b.iter(|| hash_sha512(&data))
    });

    group.finish();
}
```

## With Different Inputs

```rust
fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_by_size");

    for size in [100, 1000, 10000].iter() {
        let data = "x".repeat(*size);

        group.bench_with_input(
            BenchmarkId::new("sha256", size),
            &data,
            |b, data| b.iter(|| hash_sha256(data)),
        );
    }

    group.finish();
}
```

## Throughput

```rust
use criterion::Throughput;

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");

    for size in [1024, 4096, 16384] {
        let data = vec![0u8; size];

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &data,
            |b, data| b.iter(|| process(data)),
        );
    }

    group.finish();
}
```

## Configuration

```rust
group.sample_size(100);           // Number of samples
group.measurement_time(Duration::from_secs(10));
group.warm_up_time(Duration::from_secs(3));
```
