# Speed Optimization

Make your CLI run faster.

## Measure Performance

```bash
# Benchmark startup time
hyperfine 'dx --version'

# Compare versions
hyperfine 'dx-old hash file.txt' 'dx-new hash file.txt'

# With warmup
hyperfine --warmup 3 'dx hash file.txt'
```

## Algorithm Choice

```rust
// O(n²) - slow
fn find_duplicates_slow(items: &[i32]) -> Vec<i32> {
    items.iter()
        .filter(|&x| items.iter().filter(|&y| x == y).count() > 1)
        .cloned()
        .collect()
}

// O(n) - fast
fn find_duplicates_fast(items: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut dups = HashSet::new();
    for &item in items {
        if !seen.insert(item) {
            dups.insert(item);
        }
    }
    dups.into_iter().collect()
}
```

## Avoid Allocations

```rust
// Allocates on each call
fn process_slow(s: &str) -> String {
    s.to_uppercase()
}

// Reuses buffer
fn process_fast(s: &str, buf: &mut String) {
    buf.clear();
    buf.extend(s.chars().map(|c| c.to_ascii_uppercase()));
}
```

## Use Iterators

```rust
// Creates intermediate Vec
let sum: i32 = data.iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>()
    .iter()
    .sum();

// Zero-cost iteration
let sum: i32 = data.iter()
    .map(|x| x * 2)
    .sum();
```

## Lazy Evaluation

```rust
// Evaluates all items
fn find_first_match(items: &[Item]) -> Option<&Item> {
    let results: Vec<_> = items.iter()
        .filter(|i| i.matches())
        .collect();
    results.first().copied()
}

// Stops at first match
fn find_first_match_fast(items: &[Item]) -> Option<&Item> {
    items.iter().find(|i| i.matches())
}
```

## Parallelization

```toml
[dependencies]
rayon = "1"
```

```rust
use rayon::prelude::*;

// Sequential
let results: Vec<_> = items.iter()
    .map(|x| expensive_operation(x))
    .collect();

// Parallel
let results: Vec<_> = items.par_iter()
    .map(|x| expensive_operation(x))
    .collect();
```

## String Operations

```rust
// Slow: many allocations
let mut s = String::new();
for item in items {
    s = s + &item.to_string() + ", ";
}

// Fast: single allocation
let s = items.iter()
    .map(|i| i.to_string())
    .collect::<Vec<_>>()
    .join(", ");

// Even faster: with capacity
let mut s = String::with_capacity(items.len() * 10);
for (i, item) in items.iter().enumerate() {
    if i > 0 { s.push_str(", "); }
    write!(&mut s, "{}", item).unwrap();
}
```

## File I/O

```rust
use std::io::{BufReader, BufWriter};

// Slow: many small reads
let content = std::fs::read_to_string(path)?;

// Fast: buffered reading
let file = File::open(path)?;
let reader = BufReader::new(file);
for line in reader.lines() {
    process(line?);
}

// Fast: buffered writing
let file = File::create(path)?;
let mut writer = BufWriter::new(file);
writeln!(writer, "{}", content)?;
```

## Caching

```rust
use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

fn expensive_lookup(key: &str) -> String {
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.entry(key.to_string())
            .or_insert_with(|| compute_value(key))
            .clone()
    })
}
```

## Profile Before Optimizing

Always profile to find the actual bottleneck:

```bash
cargo flamegraph --bin dx -- hash large_file.txt
```
