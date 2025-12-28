# dx-expr

A full-featured arithmetic expression evaluator with support for variables, user-defined functions, lambdas, closures, conditionals, and more.

## Features

- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^`, `**`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `and`/`&&`, `or`/`||`, `not`/`!`
- **Conditionals**: `if cond then a else b`
- **Variables**: `x = 5`
- **User-defined functions**: `def f(x) = x * 2`
- **Lambda expressions**: `f = x => x * 2` or `f = (a, b) => a + b`
- **Closures**: Functions capture their environment
- **Comments**: `# comment to end of line`
- **30+ built-in functions**: Trig, hyperbolic, rounding, multi-arg, variadic
- **AST serialization**: Export/import expressions as JSON with serde

## Quick Start

```rust
use dx_expr::{eval, eval_program, Context};

// Basic arithmetic
assert_eq!(eval("2 + 3 * 4").unwrap(), 14.0);

// Power and functions
assert_eq!(eval("2 ^ 10").unwrap(), 1024.0);
assert_eq!(eval("sqrt(16)").unwrap(), 4.0);

// Multi-statement programs with variables
let result = eval_program("x = 5; y = x + 3; y * 2").unwrap();
assert_eq!(result, 16.0);

// User-defined functions
let result = eval_program(r#"
    def factorial(n) = if n <= 1 then 1 else n * factorial(n - 1)
    factorial(5)
"#).unwrap();
assert_eq!(result, 120.0);

// Lambda expressions
let result = eval_program(r#"
    double = x => x * 2
    double(5)
"#).unwrap();
assert_eq!(result, 10.0);

// Closures
let result = eval_program(r#"
    multiplier = 10
    scale = x => x * multiplier
    scale(5)
"#).unwrap();
assert_eq!(result, 50.0);
```

## Constants

| Name | Value | Description |
|------|-------|-------------|
| `pi` | 3.14159... | Circle constant |
| `e` | 2.71828... | Euler's number |
| `tau` | 6.28318... | 2 * pi |
| `true` | 1.0 | Boolean true |
| `false` | 0.0 | Boolean false |

## Built-in Functions

### Single Argument
- **Trigonometric**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
- **Hyperbolic**: `sinh`, `cosh`, `tanh`
- **Roots**: `sqrt`, `cbrt`
- **Rounding**: `floor`, `ceil`, `round`, `trunc`
- **Other**: `abs`, `sign`, `fract`, `exp`, `ln`, `log2`, `log10`, `print`

### Multiple Arguments
- **Two args**: `max`, `min`, `pow`, `atan2`, `hypot`, `log`, `mod`
- **Three args**: `clamp`, `lerp`
- **Variadic**: `sum`, `avg`

## Operator Precedence

From lowest to highest:

1. Logical OR (`or`, `||`)
2. Logical AND (`and`, `&&`)
3. Equality (`==`, `!=`)
4. Comparison (`<`, `>`, `<=`, `>=`)
5. Addition, Subtraction (`+`, `-`)
6. Multiplication, Division, Modulo (`*`, `/`, `%`)
7. Power (`^`, `**`) - right-associative
8. Unary (`-`, `not`, `!`)
9. Function calls, parentheses

## AST Inspection

```rust
use dx_expr::{parse, Expr, BinOp};

let ast = parse("2 + 3 * 4").unwrap();

// Serialize to JSON
let json = serde_json::to_string_pretty(&ast).unwrap();
println!("{}", json);

// Pattern match on structure
match ast {
    Expr::BinOp { op: BinOp::Add, left, right } => {
        println!("Addition of {:?} and {:?}", left, right);
    }
    _ => {}
}
```

## Context API

```rust
use dx_expr::{eval_with_context, Context};

let mut ctx = Context::new();
ctx.set("x", 10.0);

// Use predefined variable
let result = eval_with_context("y = x * 2; y + 5", &mut ctx).unwrap();
assert_eq!(result, 25.0);

// Context now has 'y' set
assert_eq!(ctx.get("y"), Some(20.0));
```

## License

MIT
