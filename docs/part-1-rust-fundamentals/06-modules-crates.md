# Modules and Crates

Rust's module system helps you organize code into logical units with controlled visibility. This chapter covers packages, crates, modules, and paths.

## Terminology

- **Package**: A Cargo project containing one or more crates
- **Crate**: A compilation unit (binary or library)
- **Module**: A namespace for organizing code within a crate
- **Path**: How you refer to items (functions, structs, etc.)

## Packages and Crates

### Creating a Package

```bash
cargo new my_project      # Binary crate
cargo new my_lib --lib    # Library crate
```

A package contains:
- `Cargo.toml` - Package manifest
- `src/main.rs` - Binary crate root (optional)
- `src/lib.rs` - Library crate root (optional)

### Package Rules

- A package must contain at least one crate
- A package can contain at most one library crate
- A package can contain multiple binary crates

### Multiple Binaries

```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs           # Default binary
│   └── lib.rs            # Library
└── src/bin/
    ├── tool1.rs          # Additional binary
    └── tool2.rs          # Additional binary
```

Run with:

```bash
cargo run                  # Runs default (main.rs)
cargo run --bin tool1      # Runs tool1
cargo run --bin tool2      # Runs tool2
```

## Defining Modules

### Inline Modules

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

### Module Tree

The module structure forms a tree:

```
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
```

## File-Based Modules

### Single File Module

```
src/
├── main.rs
└── utils.rs
```

`src/main.rs`:
```rust
mod utils;  // Loads src/utils.rs

fn main() {
    utils::helper();
}
```

`src/utils.rs`:
```rust
pub fn helper() {
    println!("I'm a helper!");
}
```

### Directory-Based Module

```
src/
├── main.rs
└── utils/
    ├── mod.rs
    └── strings.rs
```

`src/main.rs`:
```rust
mod utils;

fn main() {
    utils::strings::capitalize("hello");
}
```

`src/utils/mod.rs`:
```rust
pub mod strings;
```

`src/utils/strings.rs`:
```rust
pub fn capitalize(s: &str) -> String {
    // ...
}
```

### Modern Style (Rust 2018+)

Instead of `mod.rs`, use `<module>.rs`:

```
src/
├── main.rs
├── utils.rs           # Declares submodules
└── utils/
    └── strings.rs     # Submodule
```

`src/utils.rs`:
```rust
pub mod strings;
```

## Paths

### Absolute Paths

Start from crate root:

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

### Relative Paths

Start from current module:

```rust
front_of_house::hosting::add_to_waitlist();
```

### super

Access parent module:

```rust
mod parent {
    pub fn function() {}

    mod child {
        pub fn call_parent() {
            super::function();
        }
    }
}
```

### self

Refer to current module (useful in use statements):

```rust
use self::submodule::function;
```

## Visibility (pub)

By default, everything is private. Use `pub` for public access:

```rust
mod outer {
    pub mod inner {
        pub fn public_function() {}
        fn private_function() {}  // Only visible within inner
    }

    fn outer_function() {
        inner::public_function();     // OK
        // inner::private_function();  // Error: private
    }
}

fn main() {
    outer::inner::public_function();  // OK
}
```

### pub(crate)

Visible within the crate only:

```rust
pub(crate) fn internal_function() {}
```

### pub(super)

Visible to parent module:

```rust
mod parent {
    mod child {
        pub(super) fn parent_can_see() {}
    }

    fn use_it() {
        child::parent_can_see();  // OK
    }
}
```

### Struct Field Visibility

```rust
mod example {
    pub struct PublicStruct {
        pub public_field: i32,
        private_field: i32,  // Private by default
    }

    impl PublicStruct {
        pub fn new(public: i32, private: i32) -> Self {
            PublicStruct {
                public_field: public,
                private_field: private,
            }
        }
    }
}

fn main() {
    let s = example::PublicStruct::new(1, 2);
    println!("{}", s.public_field);
    // println!("{}", s.private_field);  // Error: private
}
```

## The use Keyword

### Bringing Items into Scope

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

### Idiomatic use Patterns

For functions, bring the parent module:

```rust
use front_of_house::hosting;
hosting::add_to_waitlist();  // Clear where it comes from
```

For structs/enums, bring the full path:

```rust
use std::collections::HashMap;
let mut map = HashMap::new();
```

### Handling Name Conflicts

Option 1: Use parent modules

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result { /* ... */ }
fn function2() -> io::Result<()> { /* ... */ }
```

Option 2: Rename with `as`

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result { /* ... */ }
fn function2() -> IoResult<()> { /* ... */ }
```

### Re-exporting with pub use

```rust
mod internal {
    pub fn helper() {}
}

pub use internal::helper;  // Now accessible as crate::helper
```

### Nested Paths

```rust
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};  // self = std::io
```

### Glob Import

```rust
use std::collections::*;  // Import everything
```

Generally avoided except in:
- Tests
- Prelude modules

## Separating into Multiple Files

### Real Project Structure

```
src/
├── main.rs
├── lib.rs
├── cli/
│   ├── mod.rs
│   ├── args.rs
│   └── commands/
│       ├── mod.rs
│       ├── hash.rs
│       └── encode.rs
├── config/
│   ├── mod.rs
│   └── settings.rs
└── utils/
    ├── mod.rs
    └── output.rs
```

`src/lib.rs`:
```rust
pub mod cli;
pub mod config;
pub mod utils;
```

`src/cli/mod.rs`:
```rust
pub mod args;
pub mod commands;

pub use args::Args;
pub use commands::run;
```

`src/cli/commands/mod.rs`:
```rust
pub mod hash;
pub mod encode;

use crate::cli::Args;

pub fn run(args: Args) -> anyhow::Result<()> {
    match args.command {
        // ...
    }
}
```

`src/main.rs`:
```rust
use dx::cli::{Args, run};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args)
}
```

## External Crates

### Adding Dependencies

`Cargo.toml`:
```toml
[dependencies]
serde = "1.0"
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
```

Or via command line:
```bash
cargo add serde
cargo add serde_json
cargo add clap --features derive
```

### Using External Crates

```rust
use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config: String,
}
```

## Prelude Pattern

Create a convenience module for common imports:

`src/prelude.rs`:
```rust
pub use crate::config::Config;
pub use crate::error::{Error, Result};
pub use crate::utils::{format_output, print_error};
```

`src/lib.rs`:
```rust
pub mod prelude;
```

Usage:
```rust
use crate::prelude::*;
```

## Workspaces

For multiple related packages:

```
my_workspace/
├── Cargo.toml          # Workspace manifest
├── cli/
│   ├── Cargo.toml
│   └── src/
├── core/
│   ├── Cargo.toml
│   └── src/
└── utils/
    ├── Cargo.toml
    └── src/
```

Workspace `Cargo.toml`:
```toml
[workspace]
members = ["cli", "core", "utils"]
```

`cli/Cargo.toml`:
```toml
[package]
name = "cli"

[dependencies]
core = { path = "../core" }
utils = { path = "../utils" }
```

## Best Practices

### Module Organization

```
src/
├── lib.rs              # Public API
├── main.rs             # Binary entry point
├── commands/           # CLI commands
├── config/             # Configuration
├── error.rs            # Error types
├── utils/              # Utilities
└── prelude.rs          # Common imports
```

### Visibility Guidelines

1. Start with everything private
2. Make public only what's needed externally
3. Use `pub(crate)` for internal APIs
4. Document public items

### Naming Conventions

- Modules: `snake_case`
- Types: `PascalCase`
- Functions/variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`

## Summary

| Concept | Purpose |
|---------|---------|
| Package | Cargo project with Cargo.toml |
| Crate | Compilation unit (lib or bin) |
| Module | Namespace for code organization |
| `mod` | Declare a module |
| `pub` | Make items public |
| `use` | Bring items into scope |
| `pub use` | Re-export items |
| `super` | Parent module path |
| `crate` | Crate root path |

Key files:
- `Cargo.toml` - Package manifest
- `src/lib.rs` - Library crate root
- `src/main.rs` - Binary crate root
- `src/<module>.rs` - Module file
- `src/<module>/mod.rs` - Module directory
