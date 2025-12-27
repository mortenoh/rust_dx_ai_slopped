# Collections

Rust's standard library includes several powerful collection types. Unlike arrays and tuples, collections store data on the heap and can grow or shrink at runtime.

## `Vec<T>` - Dynamic Arrays

### Creating Vectors

```rust
fn main() {
    // Empty vector with type annotation
    let v: Vec<i32> = Vec::new();

    // Using vec! macro
    let v = vec![1, 2, 3];

    // With capacity pre-allocated
    let mut v = Vec::with_capacity(100);
    v.push(1);
}
```

### Adding Elements

```rust
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    // Extend from iterator
    v.extend([8, 9, 10]);

    // Append another vector
    let mut v2 = vec![11, 12];
    v.append(&mut v2);  // v2 is now empty
}
```

### Accessing Elements

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Indexing (panics if out of bounds)
    let third = v[2];

    // Safe access with get()
    match v.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("No third element"),
    }

    // First and last
    if let Some(first) = v.first() {
        println!("First: {}", first);
    }

    if let Some(last) = v.last() {
        println!("Last: {}", last);
    }
}
```

### Iterating

```rust
fn main() {
    let v = vec![100, 32, 57];

    // Immutable iteration
    for i in &v {
        println!("{}", i);
    }

    // Mutable iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Consuming iteration (takes ownership)
    for i in v {
        println!("{}", i);
    }
    // v is no longer valid here
}
```

### Removing Elements

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Remove by index
    let removed = v.remove(2);  // Returns 3

    // Pop last element
    let last = v.pop();  // Returns Some(5)

    // Retain elements matching predicate
    v.retain(|&x| x % 2 == 0);

    // Clear all
    v.clear();
}
```

### Useful Methods

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    println!("Length: {}", v.len());
    println!("Is empty: {}", v.is_empty());
    println!("Contains 3: {}", v.contains(&3));

    // Slicing
    let slice = &v[1..4];  // [2, 3, 4]

    // Sorting
    let mut v = vec![3, 1, 4, 1, 5];
    v.sort();            // [1, 1, 3, 4, 5]
    v.sort_by(|a, b| b.cmp(a));  // Descending

    // Deduplication (requires sorted)
    v.sort();
    v.dedup();           // [1, 3, 4, 5]
}
```

### Storing Multiple Types with Enums

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
```

## String - UTF-8 Text

Strings in Rust are UTF-8 encoded. There are two main string types:
- `String` - Owned, growable
- `&str` - Borrowed string slice

### Creating Strings

```rust
fn main() {
    // Empty string
    let mut s = String::new();

    // From string literal
    let s = String::from("hello");
    let s = "hello".to_string();

    // From characters
    let s: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();
}
```

### Updating Strings

```rust
fn main() {
    let mut s = String::from("hello");

    // Append string slice
    s.push_str(" world");

    // Append single character
    s.push('!');

    // Concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is moved, s2 is borrowed

    // format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
}
```

### Indexing Strings

Rust strings don't support direct indexing because UTF-8 characters vary in size:

```rust
fn main() {
    let hello = String::from("Hello");
    // let h = hello[0];  // Error! Strings can't be indexed

    // Use slicing carefully (must be valid UTF-8 boundaries)
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // "Зд" (each Cyrillic char is 2 bytes)
    // let s = &hello[0..1];  // Panic! Not a valid boundary
}
```

### Iterating Over Strings

```rust
fn main() {
    let hello = "नमस्ते";

    // Characters
    for c in hello.chars() {
        println!("{}", c);
    }

    // Bytes
    for b in hello.bytes() {
        println!("{}", b);
    }
}
```

### Common String Operations

```rust
fn main() {
    let s = String::from("  Hello, World!  ");

    // Trimming
    let trimmed = s.trim();

    // Case conversion
    let upper = s.to_uppercase();
    let lower = s.to_lowercase();

    // Contains
    let has_hello = s.contains("Hello");

    // Replace
    let replaced = s.replace("World", "Rust");

    // Split
    for word in s.split_whitespace() {
        println!("{}", word);
    }

    for part in s.split(',') {
        println!("{}", part);
    }

    // Lines
    let text = "line 1\nline 2\nline 3";
    for line in text.lines() {
        println!("{}", line);
    }
}
```

## `HashMap<K, V>` - Key-Value Store

### Creating HashMaps

```rust
use std::collections::HashMap;

fn main() {
    // Empty HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // With capacity
    let mut scores: HashMap<String, i32> = HashMap::with_capacity(10);

    // From iterator of tuples
    let teams = vec![
        (String::from("Blue"), 10),
        (String::from("Yellow"), 50),
    ];
    let scores: HashMap<_, _> = teams.into_iter().collect();
}
```

### Inserting and Accessing

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Access with get (returns Option)
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);  // Some(&10)

    // Access with indexing (panics if missing)
    let score = scores["Blue"];

    // Default value
    let score = scores.get("Red").unwrap_or(&0);
}
```

### Iterating

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Keys only
    for key in scores.keys() {
        println!("Team: {}", key);
    }

    // Values only
    for value in scores.values() {
        println!("Score: {}", value);
    }
}
```

### Updating Values

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Overwriting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);  // Overwrites

    // Only insert if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(50);  // Not inserted
    scores.entry(String::from("Red")).or_insert(50);   // Inserted

    // Update based on old value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    // {"hello": 1, "world": 2, "wonderful": 1}
}
```

### Entry API

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    // or_insert - insert if missing
    map.entry("key").or_insert(vec![1]);

    // or_insert_with - lazy initialization
    map.entry("key2").or_insert_with(|| expensive_calculation());

    // or_default - use Default trait
    let mut counts: HashMap<&str, i32> = HashMap::new();
    *counts.entry("word").or_default() += 1;

    // and_modify - modify existing value
    map.entry("key")
        .and_modify(|v| v.push(2))
        .or_insert(vec![2]);
}
```

## `HashSet<T>` - Unique Values

```rust
use std::collections::HashSet;

fn main() {
    let mut books = HashSet::new();

    // Insert (returns bool: true if new)
    books.insert("The Rust Book");
    books.insert("Programming Rust");
    books.insert("The Rust Book");  // Returns false, not added

    println!("Count: {}", books.len());  // 2

    // Contains
    if books.contains("The Rust Book") {
        println!("We have it!");
    }

    // Remove
    books.remove("The Rust Book");

    // Set operations
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [2, 3, 4].iter().cloned().collect();

    // Union: {1, 2, 3, 4}
    let union: HashSet<_> = a.union(&b).cloned().collect();

    // Intersection: {2, 3}
    let intersection: HashSet<_> = a.intersection(&b).cloned().collect();

    // Difference (a - b): {1}
    let difference: HashSet<_> = a.difference(&b).cloned().collect();

    // Symmetric difference: {1, 4}
    let sym_diff: HashSet<_> = a.symmetric_difference(&b).cloned().collect();
}
```

## `VecDeque<T>` - Double-Ended Queue

```rust
use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();

    // Add to ends
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);  // [0, 1, 2]

    // Remove from ends
    let front = deque.pop_front();  // Some(0)
    let back = deque.pop_back();    // Some(2)

    // Use as queue (FIFO)
    let mut queue = VecDeque::new();
    queue.push_back("first");
    queue.push_back("second");
    let next = queue.pop_front();  // Some("first")
}
```

## `BinaryHeap<T>` - Priority Queue

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();

    // Push values
    heap.push(1);
    heap.push(5);
    heap.push(2);

    // Peek at largest (doesn't remove)
    assert_eq!(heap.peek(), Some(&5));

    // Pop largest
    while let Some(value) = heap.pop() {
        println!("{}", value);  // 5, 2, 1
    }

    // For min-heap, use Reverse wrapper
    use std::cmp::Reverse;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(3));

    if let Some(Reverse(min)) = min_heap.pop() {
        println!("Min: {}", min);  // 1
    }
}
```

## `BTreeMap<K, V>` - Sorted Map

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();

    map.insert(3, "c");
    map.insert(1, "a");
    map.insert(2, "b");

    // Iteration is in sorted order
    for (key, value) in &map {
        println!("{}: {}", key, value);  // 1: a, 2: b, 3: c
    }

    // Range queries
    for (key, value) in map.range(1..3) {
        println!("{}: {}", key, value);  // 1: a, 2: b
    }
}
```

## Choosing the Right Collection

| Collection | Use When |
|------------|----------|
| `Vec<T>` | Need ordered, indexable sequence |
| `VecDeque<T>` | Need efficient add/remove at both ends |
| `HashMap<K, V>` | Need fast key-value lookup |
| `BTreeMap<K, V>` | Need sorted keys or range queries |
| `HashSet<T>` | Need unique values with fast lookup |
| `BTreeSet<T>` | Need unique, sorted values |
| `BinaryHeap<T>` | Need priority queue (always get largest) |

## Performance Characteristics

| Operation | Vec | HashMap | BTreeMap |
|-----------|-----|---------|----------|
| Access by index | O(1) | - | - |
| Access by key | - | O(1) avg | O(log n) |
| Insert | O(1) amortized | O(1) avg | O(log n) |
| Remove | O(n) | O(1) avg | O(log n) |
| Iteration | O(n) | O(n) | O(n) |

## Summary

- **Vec** - The go-to collection for ordered data
- **String** - UTF-8 encoded text, heap-allocated
- **HashMap** - Fast key-value storage
- **HashSet** - Unique values with O(1) lookup
- **VecDeque** - Efficient queue operations
- **BinaryHeap** - Priority queue
- **BTreeMap/BTreeSet** - Sorted alternatives

Choose collections based on:
1. What operations you need
2. Performance requirements
3. Whether you need ordering
4. Memory characteristics
