# expr - Expression Evaluator

A powerful mathematical expression evaluator with variables, user-defined functions, lambdas, closures, and multi-line program support.

> For a complete tutorial and language reference, see the [Expression Language Guide](../appendices/e-expr-language.md).

## Quick Examples

```bash
# Simple math
dx expr eval "2 + 3 * 4"           # 14
dx expr eval "2 ^ 10"               # 1024
dx expr eval "sqrt(16) + pi"        # 7.141592653589793

# Comparisons and logic
dx expr eval "5 > 3 and 2 < 4"      # 1 (true)
dx expr eval "if 5 > 3 then 100 else 200"  # 100

# With variables
dx expr eval "x = 5; y = x + 3; y * 2"   # 16

# User-defined functions
dx expr eval "def square(x) = x * x; square(5)"  # 25

# Lambdas
dx expr eval "double = x => x * 2; double(10)"   # 20

# Multi-line input (Ctrl+D to evaluate)
dx expr eval
> def factorial(n) = if n <= 1 then 1 else n * factorial(n-1)
> factorial(5)
> ^D
120

# Run from file
dx expr run script.dx

# Pipe from stdin
echo "x = 5; pi * x ^ 2" | dx expr run -
```

## Subcommands

| Command | Alias | Description |
|---------|-------|-------------|
| `eval`  | `e`   | Evaluate expression(s) |
| `run`   | `r`   | Run program from file or stdin |
| `ast`   | -     | Show AST as JSON |
| `list`  | `l`   | Show available functions/constants |

---

## Operators

### Arithmetic

| Operator | Description | Example | Result |
|----------|-------------|---------|--------|
| `+` | Addition | `5 + 3` | `8` |
| `-` | Subtraction | `10 - 4` | `6` |
| `*` | Multiplication | `6 * 7` | `42` |
| `/` | Division | `20 / 4` | `5` |
| `%` | Modulo (remainder) | `17 % 5` | `2` |
| `^` | Power (right-associative) | `2 ^ 10` | `1024` |
| `**` | Power (Python-style) | `2 ** 10` | `1024` |
| `-x` | Unary minus | `-5` | `-5` |
| `()` | Grouping | `(2 + 3) * 4` | `20` |

### Comparison

| Operator | Description | Example | Result |
|----------|-------------|---------|--------|
| `==` | Equal | `5 == 5` | `1` |
| `!=` | Not equal | `5 != 3` | `1` |
| `<` | Less than | `3 < 5` | `1` |
| `>` | Greater than | `5 > 3` | `1` |
| `<=` | Less or equal | `5 <= 5` | `1` |
| `>=` | Greater or equal | `5 >= 3` | `1` |

### Logical

| Operator | Alternative | Description |
|----------|-------------|-------------|
| `and` | `&&` | Logical AND |
| `or` | `\|\|` | Logical OR |
| `not` | `!` | Logical NOT |

```bash
dx expr eval "5 > 3 and 2 < 4"     # 1 (true)
dx expr eval "1 or 0"              # 1
dx expr eval "not 0"               # 1
dx expr eval "!(5 == 3)"           # 1
```

### Operator Precedence (lowest to highest)

1. Logical OR (`or`, `||`)
2. Logical AND (`and`, `&&`)
3. Equality (`==`, `!=`)
4. Comparison (`<`, `>`, `<=`, `>=`)
5. Addition, Subtraction (`+`, `-`)
6. Multiplication, Division, Modulo (`*`, `/`, `%`)
7. Power (`^`, `**`) - right-associative
8. Unary (`-`, `not`, `!`)
9. Function calls, parentheses

---

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `pi` | 3.14159... | Circle ratio (circumference/diameter) |
| `e` | 2.71828... | Euler's number |
| `tau` | 6.28318... | 2 * pi |
| `true` | 1 | Boolean true |
| `false` | 0 | Boolean false |

---

## Built-in Functions

### Single Argument

| Category | Functions |
|----------|-----------|
| Trigonometric | `sin`, `cos`, `tan`, `asin`, `acos`, `atan` |
| Hyperbolic | `sinh`, `cosh`, `tanh` |
| Roots | `sqrt`, `cbrt` |
| Rounding | `floor`, `ceil`, `round`, `trunc` |
| Logarithms | `ln`, `log2`, `log10`, `exp` |
| Other | `abs`, `sign`, `fract`, `print` |

### Multi-Argument

| Function | Description | Example |
|----------|-------------|---------|
| `max(a, b)` | Maximum | `max(3, 7)` = 7 |
| `min(a, b)` | Minimum | `min(3, 7)` = 3 |
| `pow(x, y)` | Power | `pow(2, 10)` = 1024 |
| `atan2(y, x)` | Two-arg arctangent | `atan2(1, 1)` = pi/4 |
| `hypot(a, b)` | Hypotenuse | `hypot(3, 4)` = 5 |
| `log(x, base)` | Log with base | `log(8, 2)` = 3 |
| `mod(a, b)` | Modulo | `mod(10, 3)` = 1 |
| `clamp(x, lo, hi)` | Constrain | `clamp(15, 0, 10)` = 10 |
| `lerp(a, b, t)` | Interpolate | `lerp(0, 100, 0.5)` = 50 |

### Variadic

| Function | Description | Example |
|----------|-------------|---------|
| `sum(...)` | Sum all | `sum(1, 2, 3, 4)` = 10 |
| `avg(...)` | Average | `avg(2, 4, 6)` = 4 |

---

## Conditional Expressions

```
if condition then expr1 else expr2
```

```bash
dx expr eval "if 5 > 3 then 100 else 200"           # 100
dx expr eval "x = -5; if x < 0 then -x else x"      # 5 (absolute value)
dx expr eval "a = 7; b = 12; if a > b then a else b" # 12 (max)
```

Nested conditionals:
```bash
dx expr eval "x = 75; if x >= 90 then 4 else if x >= 80 then 3 else if x >= 70 then 2 else 1"  # 2
```

---

## User-Defined Functions

### Syntax
```
def name(param1, param2, ...) = expression
```

```bash
# Simple functions
dx expr eval "def square(x) = x * x; square(5)"              # 25
dx expr eval "def add(a, b) = a + b; add(3, 4)"              # 7

# Using conditionals
dx expr eval "def myabs(x) = if x < 0 then -x else x; myabs(-7)"  # 7

# Recursive functions
dx expr eval "def factorial(n) = if n <= 1 then 1 else n * factorial(n-1); factorial(5)"  # 120

# Fibonacci
dx expr eval "def fib(n) = if n <= 1 then n else fib(n-1) + fib(n-2); fib(10)"  # 55
```

---

## Lambda Expressions

```
param => expression            # Single parameter
(p1, p2) => expression         # Multiple parameters
() => expression               # No parameters
```

```bash
dx expr eval "double = x => x * 2; double(5)"              # 10
dx expr eval "add = (a, b) => a + b; add(3, 4)"            # 7
dx expr eval "answer = () => 42; answer()"                 # 42
```

---

## Closures

Lambdas capture variables from their surrounding scope:

```bash
dx expr eval "
multiplier = 10
scale = x => x * multiplier
scale(5)
"
# Result: 50

dx expr eval "
factor = 2
double = x => x * factor
result1 = double(5)
factor = 3
result2 = double(5)
result1 + result2
"
# Result: 25 (10 + 15)
```

---

## Comments

Comments start with `#` and continue to end of line:

```bash
dx expr eval "
# Calculate circle area
radius = 5        # in meters
area = pi * radius ^ 2
area
"
# Result: 78.53981633974483
```

---

## Variables

```bash
# Assignment
dx expr eval "x = 42"                          # 42

# Multiple statements
dx expr eval "x = 5; y = 10; x * y"            # 50

# Shadowing
dx expr eval "x = 5; x = x + 1; x"             # 6
```

### Reserved Names

Cannot assign to constants, functions, or keywords:

```bash
dx expr eval "pi = 3"      # Error: Cannot assign to constant: pi
dx expr eval "sin = 42"    # Error: Cannot assign to function name: sin
dx expr eval "if = 1"      # Error: Cannot assign to keyword: if
```

---

## Running Programs

### From File

```bash
# circle.dx
radius = 5
area = pi * radius ^ 2
area
```

```bash
dx expr run circle.dx      # 78.53981633974483
```

### From Stdin

```bash
echo "x = 10; x * x" | dx expr run -

dx expr run - <<'EOF'
def factorial(n) = if n <= 1 then 1 else n * factorial(n-1)
factorial(6)
EOF
```

---

## AST Output

```bash
dx expr ast "2 + 3 * 4" --pretty
```

```json
{
  "type": "binop",
  "op": "add",
  "left": { "type": "number", "value": 2.0 },
  "right": {
    "type": "binop",
    "op": "mul",
    "left": { "type": "number", "value": 3.0 },
    "right": { "type": "number", "value": 4.0 }
  }
}
```

---

## Error Handling

```bash
dx expr eval "1 / 0"               # Error: Division by zero
dx expr eval "sqrt(-1)"            # Error: Square root of negative number
dx expr eval "x + 5"               # Error: Undefined variable: x
dx expr eval "unknown(5)"          # Error: Unknown function: unknown
```

---

## Library Usage

```rust
use rust_cli_complete::expr::{eval, eval_program, eval_with_context, Context};

// Simple evaluation
let result = eval("2 + 3 * 4")?;  // 14.0

// Multi-statement program
let result = eval_program("x = 5; y = x + 3; y * 2")?;  // 16.0

// With predefined variables
let mut ctx = Context::new();
ctx.set("radius", 5.0);
let area = eval_with_context("pi * radius ^ 2", &mut ctx)?;
```

---

## See Also

- [Expression Language Guide](../appendices/e-expr-language.md) - Complete tutorial and reference
