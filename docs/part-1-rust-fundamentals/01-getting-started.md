# Getting Started with Rust

This chapter covers installing Rust, creating your first project, understanding the project structure, and running your first program.

## Installing Rust

The recommended way to install Rust is through `rustup`, the official Rust toolchain manager.

### On macOS and Linux

Open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts. The installer will:
1. Download and install the latest stable Rust toolchain
2. Configure your PATH environment variable
3. Install `cargo`, `rustc`, `rustup`, and other tools

After installation, restart your terminal or run:

```bash
source $HOME/.cargo/env
```

### On Windows

Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs/).

You'll also need the Visual Studio C++ Build Tools:
1. Download the Visual Studio Installer
2. Select "Desktop development with C++"
3. Install

### Verify Installation

```bash
rustc --version
# rustc 1.75.0 (82e1608df 2023-12-21)

cargo --version
# cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

## Updating Rust

Rust releases a new stable version every six weeks. Update with:

```bash
rustup update
```

## Your First Rust Program

Let's start with the traditional "Hello, World!" program.

### Creating a Project

Use Cargo, Rust's package manager and build tool:

```bash
cargo new hello_world
cd hello_world
```

This creates the following structure:

```
hello_world/
├── Cargo.toml
└── src/
    └── main.rs
```

### The Cargo.toml File

`Cargo.toml` is the project manifest:

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Key sections:
- `[package]` - Project metadata
- `name` - Package name (used for the binary/library)
- `version` - Semantic version
- `edition` - Rust edition (2015, 2018, 2021)
- `[dependencies]` - External crates your project uses

### The main.rs File

`src/main.rs` is the entry point for binary crates:

```rust
fn main() {
    println!("Hello, world!");
}
```

Breaking this down:
- `fn main()` - The main function, entry point of the program
- `println!` - A macro (note the `!`) that prints to stdout
- Statements end with semicolons

### Running the Program

```bash
cargo run
```

Output:

```
   Compiling hello_world v0.1.0 (/path/to/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/hello_world`
Hello, world!
```

## Understanding Cargo Commands

### Common Commands

```bash
cargo new <name>      # Create a new project
cargo build           # Compile the project
cargo run             # Compile and run
cargo check           # Check for errors without building
cargo test            # Run tests
cargo doc             # Generate documentation
cargo clean           # Remove build artifacts
cargo update          # Update dependencies
```

### Build Modes

```bash
cargo build           # Debug build (fast compilation, slow runtime)
cargo build --release # Release build (slow compilation, fast runtime)
```

Debug builds include debug symbols and skip optimizations. Release builds enable full optimizations.

### Binary Locations

```
target/
├── debug/
│   └── hello_world       # Debug binary
└── release/
    └── hello_world       # Release binary
```

## Variables and Types

### Variable Declaration

Variables are declared with `let`:

```rust
fn main() {
    let x = 5;              // Type inferred as i32
    let y: i32 = 10;        // Explicit type annotation
    let z: f64 = 3.14;      // 64-bit floating point

    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

### Immutability by Default

Variables are immutable by default:

```rust
fn main() {
    let x = 5;
    x = 6;  // Error: cannot assign twice to immutable variable
}
```

Use `mut` for mutable variables:

```rust
fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x = 6;  // This works
    println!("x = {}", x);
}
```

### Shadowing

You can declare a new variable with the same name:

```rust
fn main() {
    let x = 5;
    let x = x + 1;       // New variable, shadows the first
    let x = x * 2;       // Shadows again

    println!("x = {}", x);  // 12
}
```

Shadowing differs from mutation:
- Creates a new variable
- Can change types
- The new variable is still immutable

### Constants

Constants are always immutable and must have a type annotation:

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159265359;
```

## Primitive Types

### Integers

| Signed | Unsigned | Size |
|--------|----------|------|
| i8 | u8 | 8-bit |
| i16 | u16 | 16-bit |
| i32 | u32 | 32-bit |
| i64 | u64 | 64-bit |
| i128 | u128 | 128-bit |
| isize | usize | pointer-sized |

```rust
let a: i32 = -42;
let b: u64 = 1_000_000;  // Underscores for readability
let c: usize = 10;        // Common for indexing
```

### Floating Point

```rust
let x: f64 = 3.14;    // 64-bit, default
let y: f32 = 2.71;    // 32-bit
```

### Boolean

```rust
let t: bool = true;
let f = false;
```

### Character

Rust's `char` is 4 bytes (Unicode scalar value):

```rust
let c: char = 'z';
let heart = '❤';
let emoji = '😀';
```

### Tuples

Fixed-size, heterogeneous:

```rust
let tup: (i32, f64, char) = (500, 6.4, 'x');
let (x, y, z) = tup;              // Destructuring
let first = tup.0;                 // Index access
```

### Arrays

Fixed-size, homogeneous:

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];
let second = arr[1];

let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

## Functions

```rust
fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    greet("World");
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value (expression)
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

Key points:
- Parameters must have type annotations
- Return type follows `->` (omit for `()`)
- Last expression without semicolon is the return value
- Use `return` for early returns

## Control Flow

### if/else

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("less than 5");
    } else if number > 10 {
        println!("greater than 10");
    } else {
        println!("between 5 and 10");
    }

    // if is an expression
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, description);
}
```

### loop

Infinite loop until `break`:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // Return value from loop
        }
    };

    println!("result = {}", result);  // 20
}
```

### while

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}
```

### for

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    // Iterate over elements
    for element in arr {
        println!("value: {}", element);
    }

    // Range (exclusive end)
    for i in 0..5 {
        println!("index: {}", i);
    }

    // Range (inclusive end)
    for i in 1..=3 {
        println!("count: {}", i);  // 1, 2, 3
    }

    // Reverse range
    for i in (1..4).rev() {
        println!("countdown: {}", i);  // 3, 2, 1
    }
}
```

## Comments

```rust
// Single-line comment

/*
   Multi-line
   comment
*/

/// Documentation comment (generates docs)
/// Supports **Markdown**
fn documented_function() {}

//! Module-level documentation comment
//! Placed at the top of a file
```

## Exercise: A Simple CLI

Let's build a tiny CLI that greets the user:

```rust
use std::io::{self, Write};

fn main() {
    // Prompt for name
    print!("Enter your name: ");
    io::stdout().flush().unwrap();  // Ensure prompt appears

    // Read input
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Trim whitespace
    let name = name.trim();

    // Greet
    if name.is_empty() {
        println!("Hello, stranger!");
    } else {
        println!("Hello, {}!", name);
    }
}
```

Run it:

```bash
cargo run
# Enter your name: Alice
# Hello, Alice!
```

This introduces:
- The `std::io` module for input/output
- `String::new()` for creating strings
- `read_line()` for reading user input
- `.expect()` for basic error handling
- `.trim()` for removing whitespace

## Summary

In this chapter, you learned:

- **Installation**: Use `rustup` to install and manage Rust
- **Cargo**: Rust's build tool and package manager
- **Variables**: Immutable by default, `mut` for mutability
- **Types**: Integers, floats, bools, chars, tuples, arrays
- **Functions**: With parameters, return types, and expressions
- **Control flow**: `if/else`, `loop`, `while`, `for`

Next, we'll dive into Rust's most distinctive feature: ownership and borrowing.
