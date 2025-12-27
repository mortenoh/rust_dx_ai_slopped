# Macros Basics

Macros are a form of metaprogramming - code that writes code. Rust has two types: declarative macros (`macro_rules!`) and procedural macros.

## Why Macros?

- **Code generation**: Reduce boilerplate
- **DSLs**: Create domain-specific languages
- **Variadic functions**: Accept any number of arguments
- **Compile-time computation**: Generate code at compile time

## Using Built-in Macros

### println! and format!

```rust
fn main() {
    // Basic printing
    println!("Hello, world!");

    // With placeholders
    let name = "Alice";
    println!("Hello, {}!", name);

    // Debug formatting
    let nums = vec![1, 2, 3];
    println!("{:?}", nums);   // [1, 2, 3]
    println!("{:#?}", nums);  // Pretty-printed

    // Named arguments
    println!("{name} has {count} items", name = "Bob", count = 5);

    // Formatting
    println!("{:>10}", "right");  // Right-align
    println!("{:<10}", "left");   // Left-align
    println!("{:^10}", "center"); // Center

    // Numbers
    println!("{:05}", 42);        // Zero-pad: 00042
    println!("{:.2}", 3.14159);   // Precision: 3.14
    println!("{:b}", 255);        // Binary: 11111111
    println!("{:x}", 255);        // Hex: ff
    println!("{:o}", 64);         // Octal: 100
}
```

### vec!

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let zeros = vec![0; 10];  // 10 zeros
}
```

### assert! and debug_assert!

```rust
fn main() {
    assert!(true);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);

    // With custom message
    assert!(1 > 0, "Math is broken!");
    assert_eq!(1, 1, "One should equal one");

    // debug_assert! only runs in debug builds
    debug_assert!(expensive_check());
}
```

### dbg!

```rust
fn main() {
    let a = 2;
    let b = dbg!(a * 2) + 1;  // Prints: [src/main.rs:3] a * 2 = 4

    // Returns the value, can be chained
    dbg!(b);  // Prints: [src/main.rs:6] b = 5
}
```

### cfg! and cfg

```rust
fn main() {
    // Runtime check
    if cfg!(target_os = "linux") {
        println!("Running on Linux!");
    }

    if cfg!(debug_assertions) {
        println!("Debug mode");
    }
}

// Compile-time conditional
#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Windows function");
}

#[cfg(not(target_os = "windows"))]
fn platform_specific() {
    println!("Non-Windows function");
}
```

### include_str! and include_bytes!

```rust
// Include file contents at compile time
const README: &str = include_str!("../README.md");
const BINARY: &[u8] = include_bytes!("../data.bin");
```

### todo!, unimplemented!, unreachable!

```rust
fn work_in_progress() -> i32 {
    todo!("Implement this later")
}

fn not_supported() {
    unimplemented!("This feature is not supported")
}

fn process(value: i32) {
    match value {
        0 => println!("zero"),
        1 => println!("one"),
        _ => unreachable!("Value must be 0 or 1"),
    }
}
```

## Declarative Macros

### Basic Syntax

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();  // Hello!
}
```

### With Arguments

```rust
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!("World");  // Hello, World!
    greet!("Alice");  // Hello, Alice!
}
```

### Fragment Specifiers

| Specifier | Matches |
|-----------|---------|
| `expr` | Expressions |
| `ident` | Identifiers |
| `ty` | Types |
| `pat` | Patterns |
| `path` | Paths (std::io::Write) |
| `stmt` | Statements |
| `block` | Block expressions |
| `item` | Items (functions, structs, etc.) |
| `literal` | Literals |
| `tt` | Token tree (anything) |

### Multiple Patterns

```rust
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (sub $a:expr, $b:expr) => {
        $a - $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
}

fn main() {
    println!("{}", calculate!(add 2, 3));  // 5
    println!("{}", calculate!(sub 5, 2));  // 3
    println!("{}", calculate!(mul 4, 3));  // 12
}
```

### Repetition

```rust
// Variadic macro
macro_rules! vec_sum {
    ( $( $x:expr ),* ) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )*
            sum
        }
    };
}

fn main() {
    println!("{}", vec_sum!(1, 2, 3, 4, 5));  // 15
}
```

Repetition operators:
- `*` - Zero or more
- `+` - One or more
- `?` - Zero or one

### Creating vec! Style Macros

```rust
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ( $( $x:expr ),+ $(,)? ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    };
    ( $x:expr; $n:expr ) => {
        vec![$x; $n]
    };
}

fn main() {
    let v1: Vec<i32> = my_vec![];
    let v2 = my_vec![1, 2, 3];
    let v3 = my_vec![0; 5];
}
```

### HashMap Literal Macro

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };

    println!("{:?}", scores);
}
```

### Generating Code

```rust
macro_rules! make_getter {
    ($name:ident, $field:ident, $ty:ty) => {
        impl $name {
            pub fn $field(&self) -> &$ty {
                &self.$field
            }
        }
    };
}

struct Person {
    name: String,
    age: u32,
}

make_getter!(Person, name, String);
make_getter!(Person, age, u32);

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{} is {} years old", p.name(), p.age());
}
```

## Procedural Macros

Procedural macros are more powerful but require a separate crate. There are three types:

### Custom Derive

```rust
// Used like: #[derive(MyTrait)]
use my_derive::MyTrait;

#[derive(MyTrait)]
struct Point {
    x: i32,
    y: i32,
}
```

### Attribute Macros

```rust
// Used like: #[my_attribute]
use my_macros::my_route;

#[my_route(GET, "/")]
fn index() -> &'static str {
    "Hello!"
}
```

### Function-like Macros

```rust
// Used like: sql!(...)
use my_macros::sql;

let query = sql!(SELECT * FROM users WHERE active = true);
```

### Example: Custom Derive (Concept)

Creating a derive macro requires a separate crate:

```toml
# my_derive/Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
```

```rust
// my_derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
```

## Common Macro Patterns

### Default Values

```rust
macro_rules! with_defaults {
    ($name:ident) => {
        with_defaults!($name, 0, "default")
    };
    ($name:ident, $value:expr) => {
        with_defaults!($name, $value, "default")
    };
    ($name:ident, $value:expr, $label:expr) => {
        let $name = ($value, $label);
    };
}

fn main() {
    with_defaults!(a);           // (0, "default")
    with_defaults!(b, 42);       // (42, "default")
    with_defaults!(c, 42, "X");  // (42, "X")
}
```

### DSL for Testing

```rust
macro_rules! test_case {
    ($name:ident: $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(process($input), $expected);
        }
    };
}

fn process(x: i32) -> i32 {
    x * 2
}

test_case!(test_zero: 0 => 0);
test_case!(test_positive: 5 => 10);
test_case!(test_negative: -3 => -6);
```

### Builder Pattern

```rust
macro_rules! builder {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(Default)]
        pub struct $name {
            $( $field: Option<$ty>, )*
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            $(
                pub fn $field(mut self, value: $ty) -> Self {
                    self.$field = Some(value);
                    self
                }
            )*
        }
    };
}

builder!(Config {
    host: String,
    port: u16,
    debug: bool,
});

fn main() {
    let config = Config::new()
        .host("localhost".to_string())
        .port(8080)
        .debug(true);
}
```

## Macro Debugging

### trace_macros!

```rust
#![feature(trace_macros)]

trace_macros!(true);
my_macro!(1, 2, 3);
trace_macros!(false);
```

### Cargo expand

```bash
cargo install cargo-expand
cargo expand
```

## Best Practices

1. **Start simple**: Use functions first, macros when necessary
2. **Document well**: Macros are harder to understand
3. **Test thoroughly**: Edge cases are easy to miss
4. **Limit scope**: Export only what's needed
5. **Hygiene**: Use unique names to avoid conflicts

## When to Use Macros

**Use macros for:**
- Variadic functions (`println!`, `vec!`)
- Compile-time code generation
- DSLs and custom syntax
- Reducing repetitive boilerplate
- Conditional compilation

**Don't use macros for:**
- Simple abstraction (use functions)
- Runtime behavior (use traits)
- When generics suffice

## Summary

| Macro Type | Use Case |
|------------|----------|
| `macro_rules!` | Simple pattern-based macros |
| `#[derive(...)]` | Auto-implement traits |
| `#[attribute]` | Transform items |
| `proc_macro` | Complex code generation |

Built-in macros to know:
- `println!`, `eprintln!`, `format!` - Output
- `vec!` - Vector creation
- `assert!`, `assert_eq!`, `assert_ne!` - Testing
- `dbg!` - Debugging
- `cfg!` - Conditional compilation
- `include_str!`, `include_bytes!` - File embedding
- `todo!`, `unimplemented!`, `unreachable!` - Placeholders
