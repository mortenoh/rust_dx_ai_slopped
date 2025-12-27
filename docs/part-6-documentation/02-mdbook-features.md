# mdbook Features

Built-in features for rich documentation.

## Code Blocks

### Syntax Highlighting

```markdown
    ```rust
    fn main() {
        println!("Highlighted Rust code");
    }
    ```
```

Supported: rust, bash, toml, json, yaml, python, javascript, and more.

### Hiding Lines

```markdown
    ```rust
    # // Hidden line (starts with #)
    fn main() {
        println!("Visible code");
    }
    ```
```

### Editable Code (Rust Playground)

```markdown
    ```rust,editable
    fn main() {
        // Users can edit and run this
        println!("Try changing me!");
    }
    ```
```

### Line Numbers

```markdown
    ```rust,line_numbers
    fn first() {}
    fn second() {}
    fn third() {}
    ```
```

### Ignoring/No Run

```markdown
    ```rust,ignore
    fn incomplete() {
        // Won't be tested
    ```

    ```rust,no_run
    fn compiles_but_dont_run() {
        std::process::exit(1);
    }
    ```
```

## Including Files

### Include Entire File

```markdown
\{{#include ../examples/demo.rs}}
```

### Include Specific Lines

```markdown
\{{#include ../examples/demo.rs:5:10}}
```

### Include by Anchor

In source file:
```rust
// ANCHOR: example
fn important_code() {
    println!("This gets included");
}
// ANCHOR_END: example
```

In markdown:
```markdown
\{{#include ../examples/demo.rs:example}}
```

## Search

Enabled by default. Configure in book.toml:

```toml
[output.html.search]
enable = true
limit-results = 30
teaser-word-count = 30
use-hierarchical-headings = true
boost-title = 2
boost-hierarchy = 1
boost-paragraph = 1
expand = true
heading-split-level = 3
```

## Themes

Built-in themes:
- `light` - Light theme
- `rust` - Rust documentation style
- `coal` - Dark theme
- `navy` - Navy dark theme
- `ayu` - Ayu dark theme

```toml
[output.html]
default-theme = "rust"
preferred-dark-theme = "ayu"
```

## Math Support (MathJax)

```toml
[output.html]
mathjax-support = true
```

```markdown
Inline: \\( x^2 + y^2 = z^2 \\)

Block:
\\[ \sum_{i=1}^{n} i = \frac{n(n+1)}{2} \\]
```

## Admonitions

Using HTML:

```markdown
<div class="warning">

This is a warning box.

</div>
```

## Links and References

### Internal Links

```markdown
[See installation](./guide/install.md)
[Hash command](../commands/hash.md#options)
```

### External Links

```markdown
[Rust Book](https://doc.rust-lang.org/book/)
```

## Print Support

Single-page printable version at `print.html`.

```toml
[output.html.print]
enable = true
```

## Multi-language

```toml
[book]
language = "en"
multilingual = false

[language.en]
name = "English"
title = "My Book"

[language.es]
name = "Español"
title = "Mi Libro"
```
