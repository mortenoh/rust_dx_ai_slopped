//! Hash benchmarks using Criterion.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};

fn bench_hashing(c: &mut Criterion) {
    let data_sizes = [64, 1024, 65536]; // 64B, 1KB, 64KB

    let mut group = c.benchmark_group("hash_algorithms");

    for size in data_sizes {
        let data: Vec<u8> = (0..size).map(|i| i as u8).collect();

        group.bench_with_input(BenchmarkId::new("MD5", size), &data, |b, data| {
            b.iter(|| {
                let mut hasher = Md5::new();
                hasher.update(black_box(data));
                hasher.finalize()
            })
        });

        group.bench_with_input(BenchmarkId::new("SHA256", size), &data, |b, data| {
            b.iter(|| {
                let mut hasher = Sha256::new();
                hasher.update(black_box(data));
                hasher.finalize()
            })
        });

        group.bench_with_input(BenchmarkId::new("SHA512", size), &data, |b, data| {
            b.iter(|| {
                let mut hasher = Sha512::new();
                hasher.update(black_box(data));
                hasher.finalize()
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_hashing);
criterion_main!(benches);
