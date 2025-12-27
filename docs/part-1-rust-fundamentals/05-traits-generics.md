# Traits and Generics

Traits define shared behavior, and generics let you write flexible, reusable code. Together, they enable Rust's zero-cost abstractions.

## Generics

### Generic Functions

Write functions that work with multiple types:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));
}
```

### Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
}
```

### Multiple Generic Types

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mixed = Point { x: 5, y: 4.0 };
}
```

### Generic Enums

Standard library examples:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Generic Methods

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement only for specific types
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("x = {}", p.x());
    println!("distance = {}", p.distance_from_origin());
}
```

## Defining Traits

Traits define shared behavior:

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

### Implementing Traits

```rust
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("NYC"),
        author: String::from("Jane"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("alice"),
        content: String::from("Hello world!"),
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
```

### Default Implementations

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses default summarize()
}
```

## Trait Bounds

### Basic Trait Bounds

```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### Syntactic Sugar: impl Trait

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Multiple Trait Bounds

```rust
fn notify<T: Summary + Display>(item: &T) {
    println!("{}", item);  // Display
    println!("{}", item.summarize());  // Summary
}

// Or with impl Trait
fn notify(item: &(impl Summary + Display)) {
    println!("{} - {}", item, item.summarize());
}
```

### where Clauses

For complex bounds:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## Returning Traits

### impl Trait in Return Position

```rust
fn make_summarizable() -> impl Summary {
    Tweet {
        username: String::from("bot"),
        content: String::from("Hello from a function"),
    }
}
```

**Limitation**: Can only return a single concrete type:

```rust
// This won't compile:
fn choose_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { /* ... */ }
    } else {
        Tweet { /* ... */ }  // Error: different type
    }
}
```

### Trait Objects for Multiple Types

Use `dyn Trait` for runtime polymorphism:

```rust
fn choose_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle { /* ... */ })
    } else {
        Box::new(Tweet { /* ... */ })
    }
}
```

## Common Standard Library Traits

### Display and Debug

```rust
use std::fmt::{self, Display};

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{}", p);    // Display: (3, 4)
    println!("{:?}", p);  // Debug: Point { x: 3, y: 4 }
}
```

### Clone and Copy

```rust
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Clone: explicit deep copy
let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();

// Copy: implicit copy (stack-only types)
let p3 = p1;  // p1 is still valid
```

### Default

```rust
#[derive(Default)]
struct Config {
    verbose: bool,
    retries: u32,
    timeout: u64,
}

fn main() {
    let config = Config::default();
    // Config { verbose: false, retries: 0, timeout: 0 }

    let custom = Config {
        verbose: true,
        ..Default::default()
    };
}
```

### PartialEq and Eq

```rust
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };

    if p1 == p2 {
        println!("Points are equal");
    }
}
```

### PartialOrd and Ord

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

fn main() {
    let v1 = Version { major: 1, minor: 2, patch: 0 };
    let v2 = Version { major: 1, minor: 3, patch: 0 };

    if v1 < v2 {
        println!("v1 is older");
    }
}
```

### Hash

```rust
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct Key {
    id: u32,
    name: String,
}

fn main() {
    let mut map = HashMap::new();
    map.insert(Key { id: 1, name: String::from("one") }, "value");
}
```

### From and Into

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let c = Celsius(100.0);
    let f: Fahrenheit = c.into();  // Into is auto-implemented
    let f2 = Fahrenheit::from(Celsius(0.0));
}
```

### FromStr

```rust
use std::str::FromStr;

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err("Expected r,g,b format".to_string());
        }

        Ok(Color {
            r: parts[0].parse().map_err(|_| "Invalid red")?,
            g: parts[1].parse().map_err(|_| "Invalid green")?,
            b: parts[2].parse().map_err(|_| "Invalid blue")?,
        })
    }
}

fn main() {
    let color: Color = "255,128,0".parse().unwrap();
    println!("{:?}", color);
}
```

## Associated Types

Define types within traits:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

## Supertraits

Require other traits:

```rust
use std::fmt::Display;

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", " ".repeat(len));
        println!("* {} *", output);
        println!("* {} *", " ".repeat(len));
        println!("{}", "*".repeat(len + 4));
    }
}

// Must implement Display to implement OutlinePrint
struct Point { x: i32, y: i32 }

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}
```

## Newtype Pattern

Implement external traits on external types:

```rust
use std::fmt;

// Wrapper around Vec<String>
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    println!("{}", w);  // [hello, world]
}
```

## Blanket Implementations

Implement traits for all types matching a bound:

```rust
// From the standard library:
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // ...
    }
}
```

Create your own:

```rust
trait Describable {
    fn describe(&self) -> String;
}

// Implement for all Debug types
impl<T: std::fmt::Debug> Describable for T {
    fn describe(&self) -> String {
        format!("{:?}", self)
    }
}
```

## Practical Example: CLI Output

```rust
use std::fmt::Display;

trait Outputable {
    fn to_text(&self) -> String;
    fn to_json(&self) -> String;
}

#[derive(Debug)]
struct FileInfo {
    name: String,
    size: u64,
    modified: String,
}

impl Outputable for FileInfo {
    fn to_text(&self) -> String {
        format!("{} ({} bytes, modified {})",
            self.name, self.size, self.modified)
    }

    fn to_json(&self) -> String {
        format!(r#"{{"name":"{}","size":{},"modified":"{}"}}"#,
            self.name, self.size, self.modified)
    }
}

fn print_output<T: Outputable>(item: &T, format: &str) {
    match format {
        "json" => println!("{}", item.to_json()),
        _ => println!("{}", item.to_text()),
    }
}

fn main() {
    let file = FileInfo {
        name: String::from("document.txt"),
        size: 1024,
        modified: String::from("2024-01-15"),
    };

    print_output(&file, "text");
    print_output(&file, "json");
}
```

## Summary

| Concept | Purpose |
|---------|---------|
| Generics `<T>` | Write code that works with multiple types |
| Trait bounds `T: Trait` | Require specific behavior |
| `impl Trait` | Simpler syntax for trait bounds |
| `dyn Trait` | Runtime polymorphism (trait objects) |
| Associated types | Type aliases in traits |
| Supertraits | Trait dependencies |
| Blanket implementations | Implement for all matching types |

Key traits to know:
- `Debug`, `Display` - Formatting
- `Clone`, `Copy` - Duplication
- `Default` - Default values
- `PartialEq`, `Eq` - Equality
- `PartialOrd`, `Ord` - Ordering
- `Hash` - Hashing
- `From`, `Into` - Conversions
- `FromStr` - Parse from strings
