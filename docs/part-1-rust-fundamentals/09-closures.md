# Closures

Closures are anonymous functions that can capture values from their environment. They're essential for functional programming patterns in Rust.

## Basic Syntax

```rust
fn main() {
    // Full annotation
    let add_one = |x: i32| -> i32 { x + 1 };

    // Types inferred
    let add_two = |x| x + 2;

    // Multiple parameters
    let add = |a, b| a + b;

    // No parameters
    let greet = || println!("Hello!");

    // Using closures
    println!("{}", add_one(5));  // 6
    println!("{}", add_two(5));  // 7
    println!("{}", add(2, 3));   // 5
    greet();                      // Hello!
}
```

## Type Inference

Closure types are inferred from first use:

```rust
fn main() {
    let closure = |x| x;

    let s = closure(String::from("hello"));  // Now typed as String -> String
    // let n = closure(5);  // Error! Already inferred as String
}
```

## Capturing Environment

Closures can capture variables from their scope:

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;  // Captures x

    let y = 4;
    assert!(equal_to_x(y));
}
```

### Three Capture Modes

Closures capture in the least restrictive way possible:

1. **By reference (`&T`)** - Immutable borrow
2. **By mutable reference (`&mut T`)** - Mutable borrow
3. **By value (`T`)** - Take ownership

```rust
fn main() {
    // Immutable borrow
    let text = String::from("hello");
    let print = || println!("{}", text);  // Borrows text
    print();
    println!("{}", text);  // text still valid

    // Mutable borrow
    let mut count = 0;
    let mut increment = || count += 1;  // Mutably borrows count
    increment();
    increment();
    println!("{}", count);  // 2

    // Taking ownership
    let data = vec![1, 2, 3];
    let consume = || {
        let moved = data;  // Moves data into closure
        println!("{:?}", moved);
    };
    consume();
    // println!("{:?}", data);  // Error: data was moved
}
```

### The move Keyword

Force a closure to take ownership:

```rust
fn main() {
    let data = vec![1, 2, 3];

    let owns_data = move || {
        println!("{:?}", data);
    };

    owns_data();
    // data is now owned by the closure
}
```

This is essential for threads:

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{:?}", data);  // data moved to new thread
    });

    handle.join().unwrap();
}
```

## Closure Traits

Rust has three closure traits:

### FnOnce

Consumes captured values (can only be called once):

```rust
fn call_once<F: FnOnce()>(f: F) {
    f();
    // f();  // Error: already called
}

fn main() {
    let s = String::from("hello");
    let consume = || drop(s);  // Consumes s
    call_once(consume);
}
```

### FnMut

Mutably borrows captured values:

```rust
fn call_twice<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn main() {
    let mut count = 0;
    let mut counter = || count += 1;
    call_twice(&mut counter);
    println!("{}", count);  // 2
}
```

### Fn

Immutably borrows captured values:

```rust
fn call_many<F: Fn() -> i32>(f: F) {
    println!("{}", f());
    println!("{}", f());
    println!("{}", f());
}

fn main() {
    let x = 5;
    let get_x = || x;  // Just reads x
    call_many(get_x);
}
```

### Trait Hierarchy

```
FnOnce
   ↑
FnMut
   ↑
  Fn
```

All closures implement `FnOnce`. Closures that don't move captured values implement `FnMut`. Closures that only read implement `Fn`.

## Closures as Function Parameters

### Generic Approach

```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let double = |x| x * 2;
    let result = apply(double, 5);
    println!("{}", result);  // 10
}
```

### With impl Trait

```rust
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
```

### Multiple Closures

```rust
fn apply_both<F, G>(f: F, g: G, x: i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    g(f(x))
}

fn main() {
    let double = |x| x * 2;
    let add_one = |x| x + 1;

    let result = apply_both(double, add_one, 5);
    println!("{}", result);  // 11 (5*2 + 1)
}
```

## Returning Closures

Closures have unique, anonymous types. To return them, use `impl Trait` or `Box<dyn Trait>`:

### impl Trait (Single Type)

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let add_five = make_adder(5);
    println!("{}", add_five(10));  // 15
}
```

### `Box<dyn Trait>` (Multiple Types)

```rust
fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "add" => Box::new(|a, b| a + b),
        "sub" => Box::new(|a, b| a - b),
        "mul" => Box::new(|a, b| a * b),
        _ => Box::new(|a, b| a / b),
    }
}

fn main() {
    let add = make_operation("add");
    let mul = make_operation("mul");

    println!("{}", add(3, 4));  // 7
    println!("{}", mul(3, 4));  // 12
}
```

## Storing Closures in Structs

```rust
struct Cached<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cached<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cached<T> {
        Cached {
            calculation,
            value: None,
        }
    }

    fn get(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut expensive = Cached::new(|x| {
        println!("Computing...");
        x * x
    });

    println!("{}", expensive.get(4));  // Computing... 16
    println!("{}", expensive.get(4));  // 16 (cached)
}
```

## Closures with Iterators

Closures are commonly used with iterator methods:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // map
    let doubled: Vec<_> = numbers.iter()
        .map(|x| x * 2)
        .collect();

    // filter
    let evens: Vec<_> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .collect();

    // fold
    let sum = numbers.iter()
        .fold(0, |acc, x| acc + x);

    // for_each
    numbers.iter()
        .for_each(|x| println!("{}", x));

    // find
    let first_even = numbers.iter()
        .find(|x| *x % 2 == 0);

    // any / all
    let has_large = numbers.iter().any(|x| *x > 3);
    let all_positive = numbers.iter().all(|x| *x > 0);

    // sort_by
    let mut v = vec![3, 1, 4, 1, 5];
    v.sort_by(|a, b| b.cmp(a));  // Descending
}
```

## Closure Patterns

### Builder Pattern

```rust
struct RequestBuilder {
    url: String,
    headers: Vec<(String, String)>,
    on_success: Option<Box<dyn Fn(&str)>>,
    on_error: Option<Box<dyn Fn(&str)>>,
}

impl RequestBuilder {
    fn new(url: &str) -> Self {
        RequestBuilder {
            url: url.to_string(),
            headers: Vec::new(),
            on_success: None,
            on_error: None,
        }
    }

    fn on_success<F: Fn(&str) + 'static>(mut self, f: F) -> Self {
        self.on_success = Some(Box::new(f));
        self
    }

    fn on_error<F: Fn(&str) + 'static>(mut self, f: F) -> Self {
        self.on_error = Some(Box::new(f));
        self
    }
}
```

### Event Handlers

```rust
struct Button {
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    fn new() -> Self {
        Button { on_click: None }
    }

    fn set_on_click<F: Fn() + 'static>(&mut self, handler: F) {
        self.on_click = Some(Box::new(handler));
    }

    fn click(&self) {
        if let Some(ref handler) = self.on_click {
            handler();
        }
    }
}

fn main() {
    let mut button = Button::new();

    let counter = std::cell::Cell::new(0);
    button.set_on_click(move || {
        counter.set(counter.get() + 1);
        println!("Clicked {} times", counter.get());
    });

    button.click();  // Clicked 1 times
    button.click();  // Clicked 2 times
}
```

### Lazy Initialization

```rust
use std::cell::OnceCell;

struct Config {
    value: OnceCell<String>,
}

impl Config {
    fn new() -> Self {
        Config { value: OnceCell::new() }
    }

    fn get_or_init<F: FnOnce() -> String>(&self, init: F) -> &str {
        self.value.get_or_init(init)
    }
}

fn main() {
    let config = Config::new();

    let value = config.get_or_init(|| {
        println!("Initializing...");
        String::from("expensive value")
    });

    println!("{}", value);

    // Second call doesn't reinitialize
    let value = config.get_or_init(|| String::from("new value"));
    println!("{}", value);  // Still "expensive value"
}
```

## Performance

Closures have zero overhead when monomorphized:

```rust
// These are equally fast:
fn process_with_fn(v: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    v.iter().map(|&x| f(x)).collect()
}

fn process_with_closure<F: Fn(i32) -> i32>(v: &[i32], f: F) -> Vec<i32> {
    v.iter().map(|&x| f(x)).collect()
}
```

Use `dyn Fn` only when you need runtime polymorphism.

## Summary

| Feature | Description |
|---------|-------------|
| Syntax | `|params| body` |
| Type inference | Inferred from first use |
| Capture | By reference, mutable reference, or value |
| `move` | Force ownership capture |
| `Fn` | Immutable borrow (callable multiple times) |
| `FnMut` | Mutable borrow (callable multiple times) |
| `FnOnce` | Takes ownership (callable once) |
| `impl Fn` | Return single closure type |
| `Box<dyn Fn>` | Return different closure types |

Key points:
- Closures capture the minimum necessary
- Use `move` for threading or returning closures
- Choose `Fn`/`FnMut`/`FnOnce` based on needs
- Closures are zero-cost when generic
- Essential for iterator chains
