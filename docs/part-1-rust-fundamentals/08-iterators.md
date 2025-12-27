# Iterators

Iterators are a core abstraction in Rust for processing sequences of elements. They're lazy, composable, and zero-cost.

## Iterator Basics

An iterator implements the `Iterator` trait:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Creating Iterators

```rust
fn main() {
    let v = vec![1, 2, 3];

    // iter() - borrows elements
    let iter = v.iter();  // Iterator<Item = &i32>

    // iter_mut() - mutable borrows
    let mut v = vec![1, 2, 3];
    let iter_mut = v.iter_mut();  // Iterator<Item = &mut i32>

    // into_iter() - takes ownership
    let v = vec![1, 2, 3];
    let into_iter = v.into_iter();  // Iterator<Item = i32>
}
```

### Manual Iteration

```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}
```

### for Loop Uses Iterators

```rust
fn main() {
    let v = vec![1, 2, 3];

    // These are equivalent:
    for val in v.iter() {
        println!("{}", val);
    }

    for val in &v {  // &v calls .iter()
        println!("{}", val);
    }
}
```

## Iterator Adaptors

Adaptors transform iterators into other iterators. They're lazy - nothing happens until consumed.

### map

Transform each element:

```rust
fn main() {
    let v = vec![1, 2, 3];

    let doubled: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .collect();

    assert_eq!(doubled, vec![2, 4, 6]);
}
```

### filter

Keep elements matching a predicate:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let evens: Vec<&i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .collect();

    assert_eq!(evens, vec![&2, &4, &6]);
}
```

### filter_map

Combine filter and map:

```rust
fn main() {
    let strings = vec!["1", "two", "3", "four", "5"];

    let numbers: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    assert_eq!(numbers, vec![1, 3, 5]);
}
```

### flat_map

Map and flatten:

```rust
fn main() {
    let words = vec!["hello", "world"];

    let chars: Vec<char> = words.iter()
        .flat_map(|s| s.chars())
        .collect();

    assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']);
}
```

### take and skip

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let first_three: Vec<_> = v.iter().take(3).collect();
    assert_eq!(first_three, vec![&1, &2, &3]);

    let skip_two: Vec<_> = v.iter().skip(2).collect();
    assert_eq!(skip_two, vec![&3, &4, &5]);
}
```

### take_while and skip_while

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 1, 2];

    let small: Vec<_> = v.iter()
        .take_while(|&&x| x < 4)
        .collect();
    assert_eq!(small, vec![&1, &2, &3]);

    let large: Vec<_> = v.iter()
        .skip_while(|&&x| x < 4)
        .collect();
    assert_eq!(large, vec![&4, &5, &1, &2]);
}
```

### enumerate

Add index to each element:

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}
```

### zip

Combine two iterators:

```rust
fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];

    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{}: {}", name, score);
    }

    // Or collect into pairs
    let paired: Vec<_> = names.iter().zip(scores.iter()).collect();
}
```

### chain

Concatenate iterators:

```rust
fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    let combined: Vec<_> = a.iter().chain(b.iter()).collect();
    assert_eq!(combined, vec![&1, &2, &3, &4, &5, &6]);
}
```

### rev

Reverse an iterator:

```rust
fn main() {
    let v = vec![1, 2, 3];

    let reversed: Vec<_> = v.iter().rev().collect();
    assert_eq!(reversed, vec![&3, &2, &1]);
}
```

### peekable

Look ahead without consuming:

```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter().peekable();

    assert_eq!(iter.peek(), Some(&&1));  // Peek doesn't consume
    assert_eq!(iter.next(), Some(&1));   // Now consume
    assert_eq!(iter.next(), Some(&2));
}
```

### inspect

Debug intermediate values:

```rust
fn main() {
    let sum: i32 = (1..10)
        .inspect(|x| println!("before filter: {}", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("after filter: {}", x))
        .sum();

    println!("sum: {}", sum);
}
```

## Consuming Adaptors

These methods consume the iterator, producing a final value.

### collect

Convert to a collection:

```rust
fn main() {
    let v: Vec<i32> = (1..=5).collect();

    let s: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();

    use std::collections::HashSet;
    let set: HashSet<i32> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
}
```

### sum and product

```rust
fn main() {
    let sum: i32 = (1..=10).sum();  // 55
    let product: i32 = (1..=5).product();  // 120
}
```

### count and last

```rust
fn main() {
    let count = (1..=100).count();  // 100

    let last = (1..=5).last();  // Some(5)
}
```

### fold

Accumulate with initial value:

```rust
fn main() {
    let sum = (1..=5).fold(0, |acc, x| acc + x);  // 15

    let factorial = (1..=5).fold(1, |acc, x| acc * x);  // 120

    let concat = ["a", "b", "c"].iter()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        });  // "abc"
}
```

### reduce

Like fold but uses first element as initial:

```rust
fn main() {
    let max = [3, 1, 4, 1, 5].iter()
        .copied()
        .reduce(|a, b| if a > b { a } else { b });

    assert_eq!(max, Some(5));
}
```

### find and position

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let found = v.iter().find(|&&x| x > 3);  // Some(&4)

    let position = v.iter().position(|&x| x > 3);  // Some(3)
}
```

### any and all

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let has_even = v.iter().any(|x| x % 2 == 0);  // true
    let all_positive = v.iter().all(|x| *x > 0);   // true
}
```

### min, max, min_by, max_by

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9];

    let min = v.iter().min();  // Some(&1)
    let max = v.iter().max();  // Some(&9)

    // With custom comparison
    let words = vec!["hello", "hi", "hey"];
    let shortest = words.iter().min_by_key(|s| s.len());  // Some(&"hi")
    let longest = words.iter().max_by_key(|s| s.len());   // Some(&"hello")
}
```

### partition

Split into two collections:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let (evens, odds): (Vec<_>, Vec<_>) = v.iter()
        .partition(|&&x| x % 2 == 0);

    assert_eq!(evens, vec![&2, &4, &6]);
    assert_eq!(odds, vec![&1, &3, &5]);
}
```

## Creating Custom Iterators

### Implementing Iterator

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let sum: u32 = Counter::new(5).sum();
    assert_eq!(sum, 15);  // 1 + 2 + 3 + 4 + 5
}
```

### Using Iterator Methods on Custom Types

```rust
fn main() {
    let pairs: Vec<(u32, u32)> = Counter::new(3)
        .zip(Counter::new(3).skip(1))
        .collect();

    assert_eq!(pairs, vec![(1, 2), (2, 3)]);
}
```

## Range Iterators

```rust
fn main() {
    // Exclusive range
    for i in 0..5 {
        println!("{}", i);  // 0, 1, 2, 3, 4
    }

    // Inclusive range
    for i in 0..=5 {
        println!("{}", i);  // 0, 1, 2, 3, 4, 5
    }

    // With step
    for i in (0..10).step_by(2) {
        println!("{}", i);  // 0, 2, 4, 6, 8
    }

    // Reversed
    for i in (0..5).rev() {
        println!("{}", i);  // 4, 3, 2, 1, 0
    }
}
```

## Iterator Performance

Iterators are zero-cost abstractions. They compile to the same machine code as hand-written loops.

```rust
// These are equally fast:
fn sum_loop(v: &[i32]) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

fn sum_iter(v: &[i32]) -> i32 {
    v.iter().sum()
}
```

## Practical Examples

### Processing Lines

```rust
fn process_config(content: &str) -> Vec<(String, String)> {
    content.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<_> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}
```

### Counting Words

```rust
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word).or_insert(0) += 1;
            counts
        })
}
```

### Finding Files

```rust
use std::path::Path;

fn find_rust_files(dir: &Path) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "rs"))
        .collect()
}
```

## Summary

| Category | Methods |
|----------|---------|
| **Creation** | `iter()`, `iter_mut()`, `into_iter()` |
| **Transformation** | `map()`, `filter()`, `filter_map()`, `flat_map()` |
| **Selection** | `take()`, `skip()`, `take_while()`, `skip_while()` |
| **Combining** | `zip()`, `chain()`, `enumerate()` |
| **Aggregation** | `fold()`, `reduce()`, `sum()`, `product()` |
| **Searching** | `find()`, `position()`, `any()`, `all()` |
| **Collecting** | `collect()`, `partition()` |

Key points:
- Iterators are lazy - they don't do work until consumed
- Use adaptors to build processing pipelines
- `collect()` converts to any collection implementing `FromIterator`
- Zero-cost: as fast as hand-written loops
- Implement `Iterator` trait for custom types
