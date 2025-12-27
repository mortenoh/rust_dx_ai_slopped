# Optimization Techniques

Common optimizations for Rust CLI applications.

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
fn to_upper_alloc(s: &str) -> String {
    s.to_uppercase()
}

// Reuses buffer
fn to_upper_reuse(s: &str, buf: &mut String) {
    buf.clear();
    buf.extend(s.chars().map(|c| c.to_ascii_uppercase()));
}
```

## Use Iterators

```rust
// Collects intermediate results
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

## Parallelize

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

## Use Appropriate Types

```rust
// String for owned, &str for borrowed
fn process(s: &str) -> usize { s.len() }

// Box for single items, Vec for multiple
// Rc for shared, Arc for threaded
```

## Profile First

1. Measure baseline performance
2. Profile to find bottlenecks
3. Optimize the hot path
4. Measure improvement
5. Repeat
