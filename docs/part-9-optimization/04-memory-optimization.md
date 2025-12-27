# Memory Optimization

Reduce memory usage for better performance.

## Measure Memory Usage

```bash
# Peak memory (Linux)
/usr/bin/time -v dx hash large_file.txt 2>&1 | grep "Maximum resident"

# Memory over time
heaptrack dx hash large_file.txt
heaptrack_gui heaptrack.dx.*.gz
```

## Use References

```rust
// Clones data unnecessarily
fn process(data: Vec<u8>) -> Result<String> {
    // ...
}

// Borrows data
fn process(data: &[u8]) -> Result<String> {
    // ...
}
```

## Cow (Clone on Write)

```rust
use std::borrow::Cow;

fn process(input: &str) -> Cow<str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "_"))
    } else {
        Cow::Borrowed(input)
    }
}
```

## Streaming Processing

```rust
// Loads entire file into memory
fn hash_file_slow(path: &Path) -> Result<String> {
    let data = std::fs::read(path)?;
    Ok(compute_hash(&data))
}

// Streams file in chunks
fn hash_file_fast(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 { break; }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
```

## Pre-allocate Collections

```rust
// Grows incrementally
let mut results = Vec::new();
for item in items {
    results.push(process(item));
}

// Pre-allocates capacity
let mut results = Vec::with_capacity(items.len());
for item in items {
    results.push(process(item));
}

// Or use collect (often pre-allocates)
let results: Vec<_> = items.iter()
    .map(process)
    .collect();
```

## Box Large Types

```rust
// Stack allocation (large)
struct Config {
    data: [u8; 1024 * 1024],
}

// Heap allocation (small stack frame)
struct Config {
    data: Box<[u8; 1024 * 1024]>,
}
```

## Use SmallVec

```toml
[dependencies]
smallvec = "1"
```

```rust
use smallvec::SmallVec;

// Stays on stack for small sizes
let items: SmallVec<[i32; 8]> = SmallVec::new();
```

## Compact String

```toml
[dependencies]
compact_str = "0.7"
```

```rust
use compact_str::CompactString;

// Small strings stay inline (no heap allocation)
let s = CompactString::from("hello");
```

## Arena Allocation

```toml
[dependencies]
bumpalo = "3"
```

```rust
use bumpalo::Bump;

fn process_batch(items: &[String]) {
    let arena = Bump::new();

    // All allocations from this arena
    let results: Vec<&str> = items.iter()
        .map(|s| arena.alloc_str(s))
        .collect();

    // Arena freed at end of scope
}
```

## Drop Large Resources Early

```rust
fn process_file(path: &Path) -> Result<()> {
    let data = std::fs::read(path)?; // Large allocation

    let result = compute(&data);

    drop(data); // Free memory before next operation

    save_result(&result)?;
    Ok(())
}
```

## Memory-Mapped Files

```toml
[dependencies]
memmap2 = "0.9"
```

```rust
use memmap2::Mmap;

fn process_large_file(path: &Path) -> Result<()> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    // Access file as slice without loading into memory
    let data: &[u8] = &mmap[..];
    process(data)?;

    Ok(())
}
```
