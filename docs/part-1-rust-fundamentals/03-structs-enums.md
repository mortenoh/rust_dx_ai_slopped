# Structs and Enums

Structs and enums are the building blocks for creating custom types in Rust. They let you model your domain with type safety.

## Defining Structs

A struct groups related data together:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("User: {}", user.username);
}
```

## Mutable Structs

The entire struct must be mutable to modify any field:

```rust
fn main() {
    let mut user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user.email = String::from("newemail@example.com");
    user.sign_in_count += 1;
}
```

## Field Init Shorthand

When variable names match field names:

```rust
fn create_user(username: String, email: String) -> User {
    User {
        username,          // Same as username: username
        email,             // Same as email: email
        sign_in_count: 1,
        active: true,
    }
}
```

## Struct Update Syntax

Create a new struct from an existing one:

```rust
fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1  // Use remaining fields from user1
    };

    // Note: user1.username has moved to user2
    // user1.email and user1.active are still valid (Copy types or not moved)
}
```

## Tuple Structs

Structs without named fields:

```rust
struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);

    println!("Red: {}", black.0);
    println!("X: {}", origin.0);

    // Destructuring
    let Color(r, g, b) = black;
    println!("RGB: {}, {}, {}", r, g, b);
}
```

## Unit-Like Structs

Structs with no fields (useful for traits):

```rust
struct AlwaysEqual;

fn main() {
    let _subject = AlwaysEqual;
}
```

## Deriving Traits

Use `#[derive]` to automatically implement common traits:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();

    println!("{:?}", p1);           // Debug printing
    println!("{:#?}", p1);          // Pretty debug printing

    if p1 == p2 {                   // PartialEq comparison
        println!("Points are equal");
    }
}
```

Common derivable traits:
- `Debug` - Format with `{:?}`
- `Clone` - Explicit copying with `.clone()`
- `Copy` - Implicit copying (requires `Clone`)
- `PartialEq` - Equality comparison with `==`
- `Eq` - Total equality (requires `PartialEq`)
- `PartialOrd` - Ordering comparison with `<`, `>`, etc.
- `Ord` - Total ordering (requires `PartialOrd` + `Eq`)
- `Hash` - Hashing for use in `HashMap`/`HashSet`
- `Default` - Default value with `Default::default()`

## Methods with impl

Add methods to structs with `impl` blocks:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method: takes &self, &mut self, or self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Method that mutates
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // Associated function (no self) - like static methods
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);  // Associated function
    let rect2 = Rectangle::square(20);   // Associated function

    println!("Area: {}", rect1.area());  // Method call

    if rect1.can_hold(&rect2) {
        println!("rect1 can hold rect2");
    }

    let mut rect3 = Rectangle::new(10, 10);
    rect3.double();
    println!("{:?}", rect3);
}
```

## Multiple impl Blocks

You can split methods across multiple `impl` blocks:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

## Defining Enums

Enums define a type with a fixed set of variants:

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    // Handle the IP address kind
}
```

## Enums with Data

Each variant can hold different types and amounts of data:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

## Complex Enum Variants

Variants can hold any data type:

```rust
enum Message {
    Quit,                          // No data
    Move { x: i32, y: i32 },       // Named fields (like a struct)
    Write(String),                 // Single value
    ChangeColor(i32, i32, i32),    // Multiple values (like tuple struct)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        }
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        msg.call();
    }
}
```

## The Option Enum

Rust doesn't have null. Instead, it uses `Option<T>`:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Usage:

```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let some_string: Option<String> = Some(String::from("hello"));
    let absent_number: Option<i32> = None;

    // Must handle both cases to use the value
    match some_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }
}
```

## Pattern Matching with match

`match` is exhaustive pattern matching:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Binding Values in Patterns

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),  // State name
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {}", state);
            25
        }
    }
}
```

### Matching `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

### Catch-all Patterns

```rust
fn describe_number(n: i32) {
    match n {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        other => println!("Number: {}", other),  // Catch-all, binds value
    }
}

fn check_even(n: i32) {
    match n {
        0 => println!("Zero"),
        n if n % 2 == 0 => println!("Even"),
        _ => println!("Odd"),  // Catch-all, ignores value
    }
}
```

## if let

Concise pattern matching for single patterns:

```rust
fn main() {
    let some_value = Some(3);

    // Verbose match
    match some_value {
        Some(3) => println!("three!"),
        _ => (),
    }

    // Concise if let
    if let Some(3) = some_value {
        println!("three!");
    }

    // With else
    if let Some(n) = some_value {
        println!("Got: {}", n);
    } else {
        println!("Got nothing");
    }
}
```

## while let

Loop while a pattern matches:

```rust
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

## let else

New in Rust 2021 - handle the non-matching case:

```rust
fn process_value(value: Option<i32>) {
    let Some(n) = value else {
        println!("No value!");
        return;
    };

    println!("Processing: {}", n);
}
```

## Destructuring in Patterns

### Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);

    // Rename variables
    let Point { x: a, y: b } = p;
    println!("a: {}, b: {}", a, b);

    // Partial destructuring
    let Point { x, .. } = p;
    println!("x: {}", x);

    // In match
    match p {
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}
```

### Enums

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    let color = Color::Rgb(255, 128, 0);

    match color {
        Color::Rgb(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
        Color::Hsv(h, s, v) => println!("HSV: {}, {}, {}", h, s, v),
    }
}
```

### Nested Destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Circle { center: Point, radius: i32 },
    Rectangle { top_left: Point, bottom_right: Point },
}

fn main() {
    let shape = Shape::Circle {
        center: Point { x: 0, y: 0 },
        radius: 10,
    };

    match shape {
        Shape::Circle { center: Point { x, y }, radius } => {
            println!("Circle at ({}, {}) with radius {}", x, y, radius);
        }
        Shape::Rectangle { top_left, bottom_right } => {
            println!("Rectangle from {:?} to {:?}",
                (top_left.x, top_left.y),
                (bottom_right.x, bottom_right.y));
        }
    }
}
```

## Pattern Matching Reference

| Pattern | Description |
|---------|-------------|
| `_` | Wildcard, matches anything |
| `x` | Binding, matches anything and binds |
| `1` \| `2` | Or pattern |
| `1..=5` | Inclusive range |
| `(x, y)` | Tuple pattern |
| `[a, b, c]` | Array pattern |
| `[first, ..]` | Slice with rest |
| `Point { x, y }` | Struct pattern |
| `Some(x)` | Enum variant |
| `ref x` | Borrow instead of move |
| `x @ 1..=5` | Bind while matching |
| `if condition` | Match guard |

## Practical Example: CLI Configuration

```rust
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: bool,
    pub output: OutputFormat,
    pub color: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            verbose: false,
            output: OutputFormat::Text,
            color: true,
        }
    }
}

impl Config {
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn with_output(mut self, format: OutputFormat) -> Self {
        self.output = format;
        self
    }
}

fn main() {
    let config = Config::default()
        .with_verbose()
        .with_output(OutputFormat::Json);

    match config.output {
        OutputFormat::Text => println!("Plain text output"),
        OutputFormat::Json => println!("JSON output"),
        OutputFormat::Yaml => println!("YAML output"),
    }
}
```

## Summary

- **Structs** group related data with named fields
- **Tuple structs** have unnamed fields
- **impl blocks** add methods to types
- **Enums** define types with variants
- **Variants** can hold different data types
- `Option<T>` replaces null
- **match** provides exhaustive pattern matching
- **if let** / **while let** for single-pattern matching
- **Destructuring** extracts data from complex types

Structs and enums are fundamental to modeling your domain in Rust. Combined with pattern matching, they provide a powerful, type-safe way to handle complex data.
