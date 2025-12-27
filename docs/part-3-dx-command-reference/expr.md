# expr - Expression Evaluator

A powerful mathematical expression evaluator with variables, functions, and multi-line program support.

## Quick Examples

```bash
# Simple math
dx expr eval "2 + 3 * 4"           # 14
dx expr eval "2 ^ 10"               # 1024
dx expr eval "sqrt(16) + pi"        # 7.141592653589793

# With variables
dx expr eval "x = 5; y = x + 3; y * 2"   # 16

# Multi-line input (Ctrl+D to evaluate)
dx expr eval
> x = 5
> y = x + 3
> y * 2
> ^D
16

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

### Operator Precedence (lowest to highest)

1. Addition, Subtraction (`+`, `-`)
2. Multiplication, Division, Modulo (`*`, `/`, `%`)
3. Power (`^`) - right-associative
4. Unary minus (`-`)
5. Function calls, parentheses

### Right-Associative Power

```bash
dx expr eval "2 ^ 3 ^ 2"    # 512 (= 2^9, not 64)
dx expr eval "4 ^ 0.5"      # 2 (square root)
dx expr eval "27 ^ (1/3)"   # 3 (cube root)
```

---

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `pi` | 3.14159... | Circle ratio (circumference/diameter) |
| `e` | 2.71828... | Euler's number |
| `tau` | 6.28318... | 2 * pi |

```bash
dx expr eval "pi"                  # 3.141592653589793
dx expr eval "e"                   # 2.718281828459045
dx expr eval "tau"                 # 6.283185307179586
dx expr eval "2 * pi"              # 6.283185307179586 (same as tau)
```

---

## Functions

### Trigonometric (radians)

| Function | Description | Example |
|----------|-------------|---------|
| `sin(x)` | Sine | `sin(pi/2)` = 1 |
| `cos(x)` | Cosine | `cos(0)` = 1 |
| `tan(x)` | Tangent | `tan(pi/4)` = 1 |
| `asin(x)` | Arc sine | `asin(1)` = pi/2 |
| `acos(x)` | Arc cosine | `acos(0)` = pi/2 |
| `atan(x)` | Arc tangent | `atan(1)` = pi/4 |

```bash
dx expr eval "sin(pi/2)"           # 1
dx expr eval "cos(pi)"             # -1
dx expr eval "sin(0)^2 + cos(0)^2" # 1 (identity)
```

### Hyperbolic

| Function | Description | Example |
|----------|-------------|---------|
| `sinh(x)` | Hyperbolic sine | `sinh(0)` = 0 |
| `cosh(x)` | Hyperbolic cosine | `cosh(0)` = 1 |
| `tanh(x)` | Hyperbolic tangent | `tanh(0)` = 0 |

### Roots

| Function | Description | Example |
|----------|-------------|---------|
| `sqrt(x)` | Square root | `sqrt(16)` = 4 |
| `cbrt(x)` | Cube root | `cbrt(27)` = 3 |

```bash
dx expr eval "sqrt(2)"             # 1.4142135623730951
dx expr eval "sqrt(9 + 16)"        # 5 (Pythagorean)
dx expr eval "cbrt(-8)"            # -2
```

### Rounding

| Function | Description | Example |
|----------|-------------|---------|
| `floor(x)` | Round down | `floor(3.7)` = 3 |
| `ceil(x)` | Round up | `ceil(3.2)` = 4 |
| `round(x)` | Round to nearest | `round(3.5)` = 4 |
| `trunc(x)` | Truncate to integer | `trunc(-3.7)` = -3 |

```bash
dx expr eval "floor(3.7)"          # 3
dx expr eval "ceil(3.2)"           # 4
dx expr eval "round(2.5)"          # 3 (banker's rounding)
dx expr eval "trunc(-3.7)"         # -3
```

### Logarithms & Exponential

| Function | Description | Example |
|----------|-------------|---------|
| `ln(x)` | Natural log (base e) | `ln(e)` = 1 |
| `log2(x)` | Log base 2 | `log2(8)` = 3 |
| `log10(x)` | Log base 10 | `log10(100)` = 2 |
| `exp(x)` | e^x | `exp(1)` = e |

```bash
dx expr eval "ln(e)"               # 1
dx expr eval "log2(1024)"          # 10
dx expr eval "log10(1000000)"      # 6
dx expr eval "exp(2)"              # 7.38905609893065
```

### Other

| Function | Description | Example |
|----------|-------------|---------|
| `abs(x)` | Absolute value | `abs(-5)` = 5 |
| `print(x)` | Print and return value | `print(42)` = 42 |

```bash
dx expr eval "abs(-42)"            # 42
dx expr eval "abs(5 - 10)"         # 5
```

---

## Variables

Assign values to variables and use them in expressions.

### Basic Assignment

```bash
# Single statement
dx expr eval "x = 42"              # 42

# Multiple statements with semicolons
dx expr eval "x = 5; y = 10; x + y"   # 15

# Variable reference
dx expr eval "x = 5; x * x"        # 25
```

### Multi-line Programs

```bash
# Using newlines
dx expr eval "x = 5
y = x + 3
y * 2"                             # 16

# Interactive input
dx expr eval
> radius = 5
> area = pi * radius ^ 2
> area
> ^D
78.53981633589793
```

### Variable Shadowing

Variables can be reassigned:

```bash
dx expr eval "x = 5; x = x + 1; x" # 6
dx expr eval "x = 10; x = x * 2; x" # 20
```

### Reserved Names

Cannot assign to constants or function names:

```bash
dx expr eval "pi = 3"              # Error: Cannot assign to constant: pi
dx expr eval "sin = 42"            # Error: Cannot assign to function name: sin
```

---

## Running Programs

### From File

Create a file `circle.dx`:
```
radius = 5
diameter = radius * 2
circumference = pi * diameter
area = pi * radius ^ 2
print(area)
```

Run it:
```bash
dx expr run circle.dx
# Output:
# 78.53981633974483
# 78.53981633974483
```

### From Stdin

```bash
# Pipe a program
echo "x = 10; y = 20; x * y" | dx expr run -

# Here document
dx expr run - <<'EOF'
base = 10
height = 5
area = base * height / 2
area
EOF
```

---

## Practical Examples

### Circle Calculations

```bash
dx expr eval "r = 5; pi * r ^ 2"              # Area: 78.54
dx expr eval "r = 5; 2 * pi * r"              # Circumference: 31.42
dx expr eval "d = 10; pi * d"                 # Circumference from diameter
```

### Pythagorean Theorem

```bash
dx expr eval "a = 3; b = 4; sqrt(a^2 + b^2)"  # 5
dx expr eval "sqrt(3^2 + 4^2)"                # 5
dx expr eval "sqrt(5^2 + 12^2)"               # 13
```

### Quadratic Formula

```bash
# x^2 - 5x + 6 = 0 (roots: 2, 3)
dx expr eval "a=1; b=-5; c=6; (-b + sqrt(b^2 - 4*a*c)) / (2*a)"  # 3
dx expr eval "a=1; b=-5; c=6; (-b - sqrt(b^2 - 4*a*c)) / (2*a)"  # 2
```

### Compound Interest

```bash
# A = P(1 + r/n)^(nt)
dx expr eval "P=1000; r=0.05; n=12; t=10; P * (1 + r/n)^(n*t)"
# 1647.0094976902823
```

### Temperature Conversion

```bash
# Celsius to Fahrenheit
dx expr eval "c = 100; c * 9/5 + 32"          # 212

# Fahrenheit to Celsius
dx expr eval "f = 212; (f - 32) * 5/9"        # 100
```

### Distance Formula

```bash
dx expr eval "x1=0; y1=0; x2=3; y2=4; sqrt((x2-x1)^2 + (y2-y1)^2)"  # 5
```

### Logarithmic Calculations

```bash
# How many bits to represent a number?
dx expr eval "n = 1000; ceil(log2(n))"        # 10

# Decibels
dx expr eval "p1 = 1; p2 = 100; 10 * log10(p2/p1)"  # 20 dB

# pH calculation
dx expr eval "h = 0.0001; -log10(h)"          # 4
```

### Trigonometry

```bash
# Convert degrees to radians
dx expr eval "deg = 45; rad = deg * pi / 180; sin(rad)"

# Sine wave values
dx expr eval "t = 0; sin(2 * pi * t)"         # 0
dx expr eval "t = 0.25; sin(2 * pi * t)"      # 1
dx expr eval "t = 0.5; sin(2 * pi * t)"       # 0
dx expr eval "t = 0.75; sin(2 * pi * t)"      # -1
```

### Debugging with print

```bash
dx expr eval "x = 5; print(x); y = x * 2; print(y); y + 10"
# Output:
# 5
# 10
# 20
```

---

## AST Output

View the Abstract Syntax Tree for an expression:

```bash
dx expr ast "2 + 3 * 4" --pretty
```

```json
{
  "type": "binop",
  "op": "add",
  "left": {
    "type": "number",
    "value": 2.0
  },
  "right": {
    "type": "binop",
    "op": "mul",
    "left": {
      "type": "number",
      "value": 3.0
    },
    "right": {
      "type": "number",
      "value": 4.0
    }
  }
}
```

---

## Library Usage

The expression evaluator is available as a library:

```rust
use rust_cli_complete::expr::{eval, eval_program, Context};

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

## Error Handling

```bash
# Division by zero
dx expr eval "1 / 0"
# Error: Division by zero

# Square root of negative
dx expr eval "sqrt(-1)"
# Error: Square root of negative number

# Undefined variable
dx expr eval "x + 5"
# Error: Undefined variable: x

# Unknown function
dx expr eval "unknown(5)"
# Error: Unknown function: unknown
```

---

## Tips & Tricks

### Use Parentheses for Clarity

```bash
# Ambiguous: is it (2^3)^2 or 2^(3^2)?
dx expr eval "2^3^2"               # 512 (= 2^9, right-associative)
dx expr eval "(2^3)^2"             # 64
dx expr eval "2^(3^2)"             # 512
```

### Combine with Shell

```bash
# Use in shell scripts
result=$(dx expr eval "sqrt(2)")
echo "Square root of 2 is $result"

# Loop calculations
for i in 1 2 3 4 5; do
  echo "2^$i = $(dx expr eval "2^$i")"
done
```

### Script Files

Create reusable calculation scripts:

```bash
# quadratic.dx
# Solve ax^2 + bx + c = 0
# Set a, b, c before running

d = b^2 - 4*a*c
x1 = (-b + sqrt(d)) / (2*a)
x2 = (-b - sqrt(d)) / (2*a)
print(x1)
print(x2)
```

```bash
dx expr eval "a=1; b=-5; c=6" | dx expr run quadratic.dx
```
