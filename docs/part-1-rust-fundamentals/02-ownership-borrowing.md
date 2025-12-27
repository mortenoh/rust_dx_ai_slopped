# Ownership and Borrowing

Ownership is Rust's most unique feature and enables memory safety without a garbage collector. Understanding ownership is essential for writing Rust code.

## The Ownership Rules

Rust has three fundamental ownership rules:

1. **Each value in Rust has an owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped**

These rules are enforced at compile time, not runtime.

## Scope and Drop

```rust
fn main() {
    {                      // s is not valid here, it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s

    }                      // this scope is now over, and s is no longer valid
}
```

When a variable goes out of scope, Rust automatically calls `drop`, freeing the memory.

## The String Type

To understand ownership, we need a type that stores data on the heap. `String` is different from string literals:

```rust
fn main() {
    let s1 = "hello";           // String literal - fixed, immutable, on stack
    let s2 = String::from("hello");  // String - growable, mutable, on heap

    let mut s = String::from("hello");
    s.push_str(", world!");     // Modify the string
    println!("{}", s);          // "hello, world!"
}
```

String literals are embedded in the binary. `String` allocates memory on the heap.

## Move Semantics

When you assign a heap-allocated value to another variable, ownership moves:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;                    // s1's value moves to s2

    // println!("{}", s1);          // Error! s1 is no longer valid
    println!("{}", s2);             // Works fine
}
```

This prevents double-free errors. Only `s2` owns the data now.

### Visualizing the Move

Before the move:
```
s1: [ptr, len, capacity]
         |
         v
    heap: "hello"
```

After `let s2 = s1`:
```
s1: [invalid]

s2: [ptr, len, capacity]
         |
         v
    heap: "hello"
```

## Clone (Deep Copy)

If you want to deeply copy heap data, use `clone`:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();           // Deep copy

    println!("s1 = {}, s2 = {}", s1, s2);  // Both valid
}
```

This creates a complete copy of the heap data.

## Copy Trait (Stack-Only Data)

Simple types stored entirely on the stack implement the `Copy` trait:

```rust
fn main() {
    let x = 5;
    let y = x;      // Copy, not move

    println!("x = {}, y = {}", x, y);  // Both valid!
}
```

Types that implement `Copy`:
- All integer types (`i32`, `u64`, etc.)
- Boolean (`bool`)
- Floating point types (`f32`, `f64`)
- Character (`char`)
- Tuples containing only `Copy` types
- Arrays of `Copy` types

## Ownership and Functions

Passing a value to a function moves or copies it, just like assignment:

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);             // s moves into the function
    // println!("{}", s);           // Error! s is no longer valid

    let x = 5;
    makes_copy(x);                  // x is copied
    println!("{}", x);              // x is still valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}   // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}   // some_integer goes out of scope, nothing special happens
```

## Return Values and Ownership

Returning a value transfers ownership out of the function:

```rust
fn main() {
    let s1 = gives_ownership();         // Move from function to s1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 moves in, return value moves to s3

    // s2 is invalid, s1 and s3 are valid
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string                         // Return value moves to caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string                            // Return it right back
}
```

## References and Borrowing

Constantly moving values is cumbersome. References let you use a value without taking ownership:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);    // Pass a reference

    println!("The length of '{}' is {}.", s1, len);  // s1 still valid!
}

fn calculate_length(s: &String) -> usize {  // Takes a reference
    s.len()
}   // s goes out of scope but doesn't drop what it refers to
```

Creating a reference is called **borrowing**. You're borrowing the value, not owning it.

### Reference Diagram

```
s: [ptr] ───────────────┐
                        v
s1: [ptr, len, cap] ─── heap: "hello"
```

## Mutable References

By default, references are immutable. For mutation, use `&mut`:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);  // "hello, world!"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```

### Mutable Reference Restriction

You can have only ONE mutable reference to a value in a particular scope:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;  // Error! Cannot borrow as mutable more than once

    println!("{}, {}", r1, r2);
}
```

This prevents data races at compile time. A data race occurs when:
- Two or more pointers access the same data at the same time
- At least one pointer is writing
- No synchronization mechanism

### Scoping Mutable References

You can have multiple mutable references, just not simultaneous ones:

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    }   // r1 goes out of scope

    let r2 = &mut s;  // This is fine now
    println!("{}", r2);
}
```

## Combining Mutable and Immutable References

You cannot have a mutable reference while immutable references exist:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;      // Immutable borrow
    let r2 = &s;      // Another immutable borrow - fine
    let r3 = &mut s;  // Error! Cannot borrow as mutable while immutable borrows exist

    println!("{}, {}, {}", r1, r2, r3);
}
```

### Non-Lexical Lifetimes (NLL)

The compiler is smart about when references are actually used:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;  // This is fine - r1 and r2 are done
    println!("{}", r3);
}
```

## Dangling References

Rust prevents dangling references at compile time:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {            // Error! Returns reference to dropped value
    let s = String::from("hello");
    &s                               // s is dropped when function ends
}
```

The fix is to return the owned value:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s                                // Ownership moves out
}
```

## The Slice Type

Slices let you reference a contiguous sequence of elements without owning them:

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];    // &str slice
    let world = &s[6..11];

    println!("{} {}", hello, world);
}
```

### String Slices

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[..5];     // Start from beginning
    let world = &s[6..];     // Go to end
    let whole = &s[..];      // Entire string

    // String literals are slices
    let literal: &str = "hello";
}
```

### Using Slices for Safety

Slices keep references valid:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear();  // Error! Cannot mutate while borrowed

    println!("first word: {}", word);
}
```

### Other Slices

Slices work with any sequential collection:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
```

## Ownership in Practice

### Pattern: Take and Return

When you need to modify owned data:

```rust
fn process(mut data: String) -> String {
    data.push_str(" processed");
    data
}

fn main() {
    let data = String::from("hello");
    let data = process(data);
    println!("{}", data);
}
```

### Pattern: Borrow for Reading

When you only need to read:

```rust
fn analyze(data: &str) -> usize {
    data.len()
}

fn main() {
    let data = String::from("hello");
    let len = analyze(&data);
    println!("{} has {} characters", data, len);
}
```

### Pattern: Mutable Borrow for Modification

When you need to modify in place:

```rust
fn append_suffix(data: &mut String) {
    data.push_str("_suffix");
}

fn main() {
    let mut data = String::from("hello");
    append_suffix(&mut data);
    println!("{}", data);  // hello_suffix
}
```

## Summary

| Concept | Description |
|---------|-------------|
| Ownership | Each value has exactly one owner |
| Move | Assignment transfers ownership for heap data |
| Clone | Explicitly deep-copy heap data |
| Copy | Stack-only types are copied automatically |
| Borrow (`&T`) | Immutable reference, read-only access |
| Mutable borrow (`&mut T`) | Mutable reference, exclusive access |
| Slice (`&[T]`, `&str`) | Reference to a contiguous sequence |

Key rules:
- At any given time, you can have either one mutable reference OR any number of immutable references
- References must always be valid

These rules might feel restrictive at first, but they prevent entire classes of bugs at compile time. Once you internalize them, they become second nature.
