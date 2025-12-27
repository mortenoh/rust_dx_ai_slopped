//! # Benchmark Testing with Criterion
//!
//! This example shows how to benchmark CLI operations.
//!
//! Run with: `cargo run --example 0604_benchmark_tests`

#![allow(dead_code)]

fn main() {
    println!("=== Benchmark Testing with Criterion ===\n");

    // =========================================================================
    // SETUP
    // =========================================================================

    println!("--- Setup ---");
    println!(
        r#"
Add criterion to Cargo.toml:

[dev-dependencies]
criterion = {{ version = "0.5", features = ["html_reports"] }}

[[bench]]
name = "my_benchmark"
harness = false

Create benches/my_benchmark.rs:

use criterion::{{criterion_group, criterion_main, Criterion}};

fn benchmark_function(c: &mut Criterion) {{
    c.bench_function("my_operation", |b| {{
        b.iter(|| {{
            // Code to benchmark
            do_something()
        }})
    }});
}}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
"#
    );

    println!();

    // =========================================================================
    // BASIC BENCHMARKS
    // =========================================================================

    println!("--- Basic Benchmarks ---");
    println!(
        r#"
Simple benchmarks:

use criterion::{{black_box, criterion_group, criterion_main, Criterion}};
use sha2::{{Sha256, Digest}};

fn bench_sha256(c: &mut Criterion) {{
    let data = vec![0u8; 1024];  // 1KB of data

    c.bench_function("sha256_1kb", |b| {{
        b.iter(|| {{
            let mut hasher = Sha256::new();
            hasher.update(black_box(&data));
            hasher.finalize()
        }})
    }});
}}

fn bench_base64(c: &mut Criterion) {{
    let data = vec![0u8; 1024];

    c.bench_function("base64_encode_1kb", |b| {{
        b.iter(|| {{
            base64::encode(black_box(&data))
        }})
    }});

    let encoded = base64::encode(&data);
    c.bench_function("base64_decode_1kb", |b| {{
        b.iter(|| {{
            base64::decode(black_box(&encoded))
        }})
    }});
}}

criterion_group!(benches, bench_sha256, bench_base64);
criterion_main!(benches);

// Run with: cargo bench
"#
    );

    println!();

    // =========================================================================
    // COMPARING IMPLEMENTATIONS
    // =========================================================================

    println!("--- Comparing Implementations ---");
    println!(
        r#"
Compare different approaches:

fn bench_hash_algorithms(c: &mut Criterion) {{
    let data = vec![0u8; 1024 * 1024];  // 1MB

    let mut group = c.benchmark_group("hash_algorithms");

    group.bench_function("md5", |b| {{
        b.iter(|| {{
            use md5::{{Md5, Digest}};
            let mut hasher = Md5::new();
            hasher.update(black_box(&data));
            hasher.finalize()
        }})
    }});

    group.bench_function("sha256", |b| {{
        b.iter(|| {{
            use sha2::{{Sha256, Digest}};
            let mut hasher = Sha256::new();
            hasher.update(black_box(&data));
            hasher.finalize()
        }})
    }});

    group.bench_function("sha512", |b| {{
        b.iter(|| {{
            use sha2::{{Sha512, Digest}};
            let mut hasher = Sha512::new();
            hasher.update(black_box(&data));
            hasher.finalize()
        }})
    }});

    group.finish();
}}

// Output shows comparison:
// hash_algorithms/md5     time: [1.2345 ms ...]
// hash_algorithms/sha256  time: [2.3456 ms ...]
// hash_algorithms/sha512  time: [1.8765 ms ...]
"#
    );

    println!();

    // =========================================================================
    // PARAMETERIZED BENCHMARKS
    // =========================================================================

    println!("--- Parameterized Benchmarks ---");
    println!(
        r#"
Test with different input sizes:

use criterion::{{BenchmarkId, Throughput}};

fn bench_with_sizes(c: &mut Criterion) {{
    let sizes = [64, 256, 1024, 4096, 16384];

    let mut group = c.benchmark_group("hash_by_size");

    for size in sizes {{
        let data = vec![0u8; size];

        // Set throughput for bytes/sec reporting
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("sha256", size),
            &data,
            |b, data| {{
                b.iter(|| sha256(black_box(data)))
            }}
        );
    }}

    group.finish();
}}

// Output includes throughput:
// hash_by_size/sha256/64    time: [100 ns]  thrpt: [640 MB/s]
// hash_by_size/sha256/256   time: [200 ns]  thrpt: [1.28 GB/s]
// hash_by_size/sha256/1024  time: [400 ns]  thrpt: [2.56 GB/s]
"#
    );

    println!();

    // =========================================================================
    // SETUP AND TEARDOWN
    // =========================================================================

    println!("--- Setup and Teardown ---");
    println!(
        r#"
Expensive setup outside the benchmark loop:

fn bench_with_setup(c: &mut Criterion) {{
    // Expensive setup (not measured)
    let large_data = load_test_file();

    c.bench_function("process_large_file", |b| {{
        b.iter(|| {{
            // Only this is measured
            process(black_box(&large_data))
        }})
    }});
}}

// Using iter_batched for per-iteration setup
fn bench_with_per_iter_setup(c: &mut Criterion) {{
    c.bench_function("with_setup", |b| {{
        b.iter_batched(
            || create_temp_file(),  // Setup (not measured)
            |file| process_file(file),  // Benchmark (measured)
            criterion::BatchSize::SmallInput
        )
    }});
}}

// BatchSize options:
// - SmallInput: Reuse setup for many iterations
// - LargeInput: One setup per iteration
// - PerIteration: Always fresh setup
"#
    );

    println!();

    // =========================================================================
    // CLI BENCHMARKS
    // =========================================================================

    println!("--- CLI Benchmarks ---");
    println!(
        r#"
Benchmark CLI commands (integration style):

use std::process::Command;

fn bench_cli_hash(c: &mut Criterion) {{
    // Create test file once
    let temp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp.path(), vec![0u8; 1024 * 1024]).unwrap();
    let path = temp.path().to_str().unwrap();

    c.bench_function("cli_hash_1mb", |b| {{
        b.iter(|| {{
            Command::new("./target/release/dx")
                .args(["hash", path])
                .output()
                .unwrap()
        }})
    }});
}}

// Note: CLI benchmarks include process startup overhead
// For pure algorithm benchmarks, test the library directly
"#
    );

    println!();

    // =========================================================================
    // CONFIGURATION
    // =========================================================================

    println!("--- Configuration ---");
    println!(
        r#"
Configure benchmark behavior:

use criterion::{{Criterion, SamplingMode}};
use std::time::Duration;

fn custom_criterion() -> Criterion {{
    Criterion::default()
        .sample_size(100)          // Number of samples (default: 100)
        .measurement_time(Duration::from_secs(5))  // Time per benchmark
        .warm_up_time(Duration::from_secs(1))
        .with_plots()              // Generate HTML plots
        .sampling_mode(SamplingMode::Flat)  // or Linear, Auto
}}

criterion_group! {{
    name = benches;
    config = custom_criterion();
    targets = bench_sha256, bench_base64
}}

// For quick benchmarks during development:
fn quick_criterion() -> Criterion {{
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_millis(500))
}}
"#
    );

    println!();

    // =========================================================================
    // RUNNING BENCHMARKS
    // =========================================================================

    println!("--- Running Benchmarks ---");
    println!(
        r#"
Run benchmarks:

  cargo bench                    # Run all benchmarks
  cargo bench -- hash            # Run benchmarks matching "hash"
  cargo bench --bench my_bench   # Run specific benchmark file

Output location:
  target/criterion/              # Results and HTML reports
  target/criterion/report/       # Summary report

Baseline comparison:
  cargo bench -- --save-baseline main
  # Make changes...
  cargo bench -- --baseline main
  # Shows comparison to saved baseline

Quick check (fewer samples):
  cargo bench -- --quick

List benchmarks:
  cargo bench -- --list
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Benchmark testing with Criterion:");
    println!("  1. Use black_box() to prevent optimization");
    println!("  2. Group related benchmarks together");
    println!("  3. Use Throughput for bytes/sec metrics");
    println!("  4. Setup outside iter() to exclude from timing");
    println!("  5. Save baselines for regression detection");
}
