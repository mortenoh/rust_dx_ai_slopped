# Introduction to Rust

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It's an ideal choice for building CLI applications because it produces standalone binaries with no runtime dependencies, offers excellent performance, and provides memory safety without garbage collection.

## Why Rust for CLI Applications?

### Performance
Rust compiles to native machine code, giving you C/C++ level performance. Your CLI tools start instantly and use minimal resources.

### Safety
Rust's ownership system catches bugs at compile time that would be runtime crashes in other languages. Memory leaks, null pointer dereferences, and data races are prevented by the compiler.

### Single Binary Distribution
Rust produces statically-linked binaries by default. Your users don't need to install a runtime, manage dependencies, or deal with version conflicts. Just download and run.

### Cross-Platform
Write once, compile for Windows, macOS, and Linux. Rust's standard library abstracts platform differences, and tools like `cross` make cross-compilation straightforward.

### Excellent Tooling
- **Cargo**: Package manager and build system
- **rustfmt**: Automatic code formatting
- **clippy**: Linting and suggestions
- **rust-analyzer**: IDE support

## What You'll Learn in This Part

This part covers the Rust fundamentals you need to build robust CLI applications:

1. **Getting Started** - Installation, first program, project structure
2. **Ownership and Borrowing** - Rust's unique memory management model
3. **Structs and Enums** - Custom data types
4. **Error Handling** - Result, Option, and the ? operator
5. **Traits and Generics** - Polymorphism in Rust
6. **Modules and Crates** - Code organization
7. **Collections** - Vec, HashMap, and friends
8. **Iterators** - Functional-style data processing
9. **Closures** - Anonymous functions
10. **Macros Basics** - Metaprogramming fundamentals

## Prerequisites

- Basic programming experience in any language
- A terminal/command line interface
- Willingness to think differently about memory management

## The Rust Philosophy

Rust has a few key principles that inform its design:

### Zero-Cost Abstractions
High-level features compile to efficient machine code. You don't pay performance penalties for using abstractions.

### Fearless Concurrency
The same ownership rules that prevent memory bugs also prevent data races. You can write concurrent code with confidence.

### Explicit Over Implicit
Rust prefers explicit code. Type conversions, error handling, and memory operations are visible in the code rather than hidden.

### Correctness First
The compiler is strict, sometimes frustratingly so. But this strictness catches bugs before they reach production.

## A Taste of Rust

Here's a simple Rust program that demonstrates several core concepts:

```rust
use std::collections::HashMap;

fn main() {
    // Immutable by default
    let greeting = "Hello";

    // Mutable binding
    let mut counts: HashMap<&str, i32> = HashMap::new();

    // Ownership and borrowing
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];

    for word in &words {  // Borrow the vector
        *counts.entry(word).or_insert(0) += 1;
    }

    // Pattern matching
    for (word, count) in &counts {
        match count {
            1 => println!("{}: {} time", word, count),
            _ => println!("{}: {} times", word, count),
        }
    }

    // Error handling with Result
    let result: Result<i32, &str> = Ok(42);
    if let Ok(value) = result {
        println!("{} {}", greeting, value);
    }
}
```

This small example shows:
- Variable declarations with `let` and `let mut`
- Type annotations with `: Type`
- Collections (`HashMap`, `Vec`)
- Borrowing with `&`
- Iterators and loops
- Pattern matching with `match`
- The `Result` type for error handling

Don't worry if this seems like a lot - we'll cover each concept in detail throughout this part.

## Getting Help

When learning Rust, these resources are invaluable:

- **The Rust Book**: [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) - The official, comprehensive guide
- **Rust by Example**: [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) - Learn by working examples
- **Rustlings**: [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- **The Rust Reference**: [doc.rust-lang.org/reference](https://doc.rust-lang.org/reference/) - Precise language specification

Let's begin with setting up your Rust development environment.
