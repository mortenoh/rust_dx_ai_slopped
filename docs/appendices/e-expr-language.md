# The dx Expression Language

A complete guide to the dx expression language - from basic arithmetic to user-defined functions, lambdas, and closures.

## Introduction

The dx expression language is a powerful yet simple mathematical expression evaluator built into the `dx` CLI. It supports:

- Basic arithmetic with proper operator precedence
- Mathematical constants and 30+ built-in functions
- Variables and multi-statement programs
- Comparison and logical operators
- Conditional expressions (`if-then-else`)
- User-defined functions with recursion
- Lambda expressions and closures
- Comments for documentation

Whether you need quick calculations on the command line, reusable mathematical scripts, or a complete mini-language for complex computations, the expr language has you covered.

---

## Quick Start

```bash
# Basic math
dx expr eval "2 + 3 * 4"           # 14

# Power and roots
dx expr eval "2 ^ 10"              # 1024
dx expr eval "sqrt(144)"           # 12

# Built-in functions
dx expr eval "sin(pi / 2)"         # 1
dx expr eval "log2(1024)"          # 10

# Variables
dx expr eval "x = 5; y = 3; x * y" # 15

# Multi-line interactive mode (Ctrl+D to evaluate)
dx expr eval
> radius = 5
> area = pi * radius ^ 2
> area
> ^D
78.53981633974483
```

---

# Part 1: Fundamentals

## Numbers

All numbers in the expr language are 64-bit floating point values (`f64`).

```bash
# Integers
dx expr eval "42"                  # 42
dx expr eval "-17"                 # -17

# Decimals
dx expr eval "3.14159"             # 3.14159
dx expr eval "0.001"               # 0.001

# Scientific notation (via expressions)
dx expr eval "1.5 * 10^6"          # 1500000
```

## Arithmetic Operators

| Operator | Description | Example | Result |
|----------|-------------|---------|--------|
| `+` | Addition | `5 + 3` | `8` |
| `-` | Subtraction | `10 - 4` | `6` |
| `*` | Multiplication | `6 * 7` | `42` |
| `/` | Division | `20 / 4` | `5` |
| `%` | Modulo (remainder) | `17 % 5` | `2` |
| `^` | Power | `2 ^ 10` | `1024` |
| `**` | Power (Python-style) | `2 ** 10` | `1024` |
| `-x` | Unary negation | `-5` | `-5` |

### Examples

```bash
# Basic arithmetic
dx expr eval "10 + 5"              # 15
dx expr eval "10 - 5"              # 5
dx expr eval "10 * 5"              # 50
dx expr eval "10 / 5"              # 2

# Modulo - remainder after division
dx expr eval "17 % 5"              # 2 (17 = 5*3 + 2)
dx expr eval "20 % 4"              # 0 (evenly divisible)

# Power
dx expr eval "2 ^ 8"               # 256
dx expr eval "10 ** 3"             # 1000
dx expr eval "27 ^ (1/3)"          # 3 (cube root)
```

## Operator Precedence

From lowest to highest priority:

1. Logical OR (`or`, `||`)
2. Logical AND (`and`, `&&`)
3. Equality (`==`, `!=`)
4. Comparison (`<`, `>`, `<=`, `>=`)
5. Addition, Subtraction (`+`, `-`)
6. Multiplication, Division, Modulo (`*`, `/`, `%`)
7. Power (`^`, `**`) - **right-associative**
8. Unary (`-`, `not`, `!`)
9. Function calls, parentheses

### Precedence Examples

```bash
# Multiplication before addition
dx expr eval "2 + 3 * 4"           # 14 (not 20)

# Power before multiplication
dx expr eval "2 * 3 ^ 2"           # 18 (not 36)

# Right-associativity of power
dx expr eval "2 ^ 3 ^ 2"           # 512 (= 2^9, not 8^2=64)

# Use parentheses for clarity
dx expr eval "(2 + 3) * 4"         # 20
dx expr eval "2 * (3 + 4)"         # 14
dx expr eval "(2 ^ 3) ^ 2"         # 64
```

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `pi` | 3.141592653589793 | Circle ratio (circumference/diameter) |
| `e` | 2.718281828459045 | Euler's number (natural log base) |
| `tau` | 6.283185307179586 | 2 * pi (full circle in radians) |
| `true` | 1.0 | Boolean true |
| `false` | 0.0 | Boolean false |

### Constant Examples

```bash
# Mathematical constants
dx expr eval "pi"                  # 3.141592653589793
dx expr eval "e"                   # 2.718281828459045
dx expr eval "tau"                 # 6.283185307179586
dx expr eval "tau / 2"             # 3.141592653589793 (= pi)

# Boolean constants
dx expr eval "true"                # 1
dx expr eval "false"               # 0
dx expr eval "true + true"         # 2 (they're just numbers)
```

## Parentheses

Use parentheses to:
- Override operator precedence
- Group expressions for clarity
- Pass arguments to functions

```bash
dx expr eval "(2 + 3) * 4"         # 20
dx expr eval "2 * (3 + 4)"         # 14
dx expr eval "((1 + 2) * (3 + 4))" # 21
dx expr eval "sqrt((3^2) + (4^2))" # 5 (Pythagorean)
```

---

# Part 2: Built-in Functions

## Trigonometric Functions (Radians)

| Function | Description | Domain | Example |
|----------|-------------|--------|---------|
| `sin(x)` | Sine | All real | `sin(pi/2)` = 1 |
| `cos(x)` | Cosine | All real | `cos(0)` = 1 |
| `tan(x)` | Tangent | x != pi/2 + n*pi | `tan(pi/4)` = 1 |
| `asin(x)` | Arc sine | [-1, 1] | `asin(1)` = pi/2 |
| `acos(x)` | Arc cosine | [-1, 1] | `acos(0)` = pi/2 |
| `atan(x)` | Arc tangent | All real | `atan(1)` = pi/4 |

### Trig Examples

```bash
# Basic trig
dx expr eval "sin(0)"              # 0
dx expr eval "sin(pi/2)"           # 1
dx expr eval "cos(0)"              # 1
dx expr eval "cos(pi)"             # -1

# Trig identity: sin^2 + cos^2 = 1
dx expr eval "sin(1)^2 + cos(1)^2" # 1

# Inverse trig
dx expr eval "asin(1)"             # 1.5707963... (pi/2)
dx expr eval "acos(0)"             # 1.5707963... (pi/2)
dx expr eval "atan(1)"             # 0.7853981... (pi/4)
```

### Converting Degrees to Radians

```bash
# Degrees to radians: multiply by pi/180
dx expr eval "deg = 45; rad = deg * pi / 180; sin(rad)"   # 0.7071...
dx expr eval "deg = 90; sin(deg * pi / 180)"              # 1
dx expr eval "deg = 180; cos(deg * pi / 180)"             # -1
```

## Hyperbolic Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sinh(x)` | Hyperbolic sine | `sinh(0)` = 0 |
| `cosh(x)` | Hyperbolic cosine | `cosh(0)` = 1 |
| `tanh(x)` | Hyperbolic tangent | `tanh(0)` = 0 |

```bash
dx expr eval "sinh(1)"             # 1.1752011936438014
dx expr eval "cosh(1)"             # 1.5430806348152437
dx expr eval "tanh(1)"             # 0.7615941559557649

# Identity: cosh^2 - sinh^2 = 1
dx expr eval "cosh(1)^2 - sinh(1)^2"  # 1
```

## Root Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sqrt(x)` | Square root | `sqrt(16)` = 4 |
| `cbrt(x)` | Cube root | `cbrt(27)` = 3 |

```bash
dx expr eval "sqrt(16)"            # 4
dx expr eval "sqrt(2)"             # 1.4142135623730951
dx expr eval "cbrt(27)"            # 3
dx expr eval "cbrt(-8)"            # -2

# Alternative: x^(1/n) for nth root
dx expr eval "81 ^ 0.25"           # 3 (fourth root)
dx expr eval "32 ^ 0.2"            # 2 (fifth root)
```

## Rounding Functions

| Function | Description | Example |
|----------|-------------|---------|
| `floor(x)` | Round toward -infinity | `floor(3.7)` = 3 |
| `ceil(x)` | Round toward +infinity | `ceil(3.2)` = 4 |
| `round(x)` | Round to nearest integer | `round(3.5)` = 4 |
| `trunc(x)` | Truncate toward zero | `trunc(-3.7)` = -3 |

```bash
# floor - always rounds down
dx expr eval "floor(3.9)"          # 3
dx expr eval "floor(-3.1)"         # -4

# ceil - always rounds up
dx expr eval "ceil(3.1)"           # 4
dx expr eval "ceil(-3.9)"          # -3

# round - to nearest (banker's rounding)
dx expr eval "round(3.4)"          # 3
dx expr eval "round(3.5)"          # 4
dx expr eval "round(-3.5)"         # -4

# trunc - toward zero
dx expr eval "trunc(3.9)"          # 3
dx expr eval "trunc(-3.9)"         # -3
```

## Logarithm and Exponential Functions

| Function | Description | Example |
|----------|-------------|---------|
| `ln(x)` | Natural log (base e) | `ln(e)` = 1 |
| `log2(x)` | Log base 2 | `log2(8)` = 3 |
| `log10(x)` | Log base 10 | `log10(100)` = 2 |
| `log(x, b)` | Log base b | `log(8, 2)` = 3 |
| `exp(x)` | e^x | `exp(1)` = e |

```bash
# Natural logarithm
dx expr eval "ln(1)"               # 0
dx expr eval "ln(e)"               # 1
dx expr eval "ln(e^5)"             # 5

# Common logarithms
dx expr eval "log2(8)"             # 3
dx expr eval "log2(1024)"          # 10
dx expr eval "log10(100)"          # 2
dx expr eval "log10(1000000)"      # 6

# Custom base
dx expr eval "log(81, 3)"          # 4 (3^4 = 81)
dx expr eval "log(32, 2)"          # 5 (2^5 = 32)

# Exponential
dx expr eval "exp(0)"              # 1
dx expr eval "exp(1)"              # 2.718281828459045
dx expr eval "exp(ln(5))"          # 5
```

## Two-Argument Functions

| Function | Description | Example |
|----------|-------------|---------|
| `max(a, b)` | Maximum of two values | `max(3, 7)` = 7 |
| `min(a, b)` | Minimum of two values | `min(3, 7)` = 3 |
| `pow(x, y)` | x raised to power y | `pow(2, 10)` = 1024 |
| `atan2(y, x)` | Two-argument arctangent | `atan2(1, 1)` = pi/4 |
| `hypot(a, b)` | sqrt(a^2 + b^2) | `hypot(3, 4)` = 5 |
| `mod(a, b)` | Modulo (same as %) | `mod(10, 3)` = 1 |

```bash
# max and min
dx expr eval "max(10, 5)"          # 10
dx expr eval "min(10, 5)"          # 5
dx expr eval "max(-3, -7)"         # -3

# pow - explicit power function
dx expr eval "pow(2, 10)"          # 1024
dx expr eval "pow(10, -2)"         # 0.01

# atan2 - arctangent with two arguments
dx expr eval "atan2(1, 1)"         # 0.7853981... (pi/4 = 45 degrees)
dx expr eval "atan2(0, -1)"        # 3.1415926... (pi = 180 degrees)

# hypot - hypotenuse (Pythagorean)
dx expr eval "hypot(3, 4)"         # 5
dx expr eval "hypot(5, 12)"        # 13

# mod - modulo function
dx expr eval "mod(17, 5)"          # 2
```

## Three-Argument Functions

| Function | Description | Example |
|----------|-------------|---------|
| `clamp(x, lo, hi)` | Constrain x to [lo, hi] | `clamp(15, 0, 10)` = 10 |
| `lerp(a, b, t)` | Linear interpolation | `lerp(0, 100, 0.5)` = 50 |

```bash
# clamp - constrain to range
dx expr eval "clamp(5, 0, 10)"     # 5 (within range)
dx expr eval "clamp(-5, 0, 10)"    # 0 (below min)
dx expr eval "clamp(15, 0, 10)"    # 10 (above max)

# lerp - linear interpolation between a and b
dx expr eval "lerp(0, 100, 0)"     # 0 (at a)
dx expr eval "lerp(0, 100, 1)"     # 100 (at b)
dx expr eval "lerp(0, 100, 0.5)"   # 50 (halfway)
dx expr eval "lerp(0, 100, 0.25)"  # 25 (quarter way)
```

## Variadic Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sum(...)` | Sum all arguments | `sum(1, 2, 3, 4)` = 10 |
| `avg(...)` | Average of arguments | `avg(2, 4, 6)` = 4 |

```bash
# sum - add all arguments
dx expr eval "sum(1, 2, 3)"        # 6
dx expr eval "sum(1, 2, 3, 4, 5)"  # 15
dx expr eval "sum(10)"             # 10

# avg - arithmetic mean
dx expr eval "avg(2, 4, 6)"        # 4
dx expr eval "avg(1, 2, 3, 4, 5)"  # 3
dx expr eval "avg(100)"            # 100
```

## Other Functions

| Function | Description | Example |
|----------|-------------|---------|
| `abs(x)` | Absolute value | `abs(-5)` = 5 |
| `sign(x)` | Sign of number (-1, 0, 1) | `sign(-5)` = -1 |
| `fract(x)` | Fractional part | `fract(3.75)` = 0.75 |
| `print(x)` | Print and return | `print(42)` = 42 |

```bash
# abs - absolute value
dx expr eval "abs(-42)"            # 42
dx expr eval "abs(42)"             # 42
dx expr eval "abs(0)"              # 0

# sign - returns -1, 0, or 1
dx expr eval "sign(-42)"           # -1
dx expr eval "sign(42)"            # 1
dx expr eval "sign(0)"             # 1 (Rust's signum behavior)

# fract - fractional part
dx expr eval "fract(3.75)"         # 0.75
dx expr eval "fract(-3.75)"        # -0.75
dx expr eval "fract(5.0)"          # 0

# print - debugging helper
dx expr eval "x = 5; print(x); x * 2"
# Output:
# 5
# 10
```

---

# Part 3: Variables

## Variable Assignment

Assign values to variables with `=`:

```bash
# Simple assignment
dx expr eval "x = 42"              # 42

# Use the variable
dx expr eval "x = 5; x + 10"       # 15

# Multiple assignments
dx expr eval "a = 3; b = 4; a + b" # 7
```

## Variable Naming Rules

- Must start with a letter (a-z, A-Z)
- Can contain letters, digits, and underscores
- Case-sensitive (`x` and `X` are different)
- Cannot use reserved names (constants, functions, keywords)

```bash
# Valid names
dx expr eval "x = 1; X = 2; x + X"        # 3
dx expr eval "myVar = 10; myVar"          # 10
dx expr eval "value_1 = 5; value_1"       # 5

# Invalid - reserved names
dx expr eval "pi = 3"                     # Error: Cannot assign to constant
dx expr eval "sin = 42"                   # Error: Cannot assign to function
dx expr eval "if = 1"                     # Error: Cannot assign to keyword
```

## Variable Scope

Variables exist within the program's execution context:

```bash
# Variables persist through the program
dx expr eval "x = 5; y = x + 3; z = y * 2; z"  # 16

# Shadowing - same name, new value
dx expr eval "x = 5; x = 10; x = x + 1; x"     # 11
```

## Multi-Statement Programs

Separate statements with semicolons or newlines:

```bash
# Using semicolons
dx expr eval "x = 5; y = 10; x * y"            # 50

# Using newlines (multi-line input)
dx expr eval "x = 5
y = 10
x * y"                                          # 50

# Mixed
dx expr eval "x = 5; y = 10
z = x + y; z * 2"                               # 30
```

## Interactive Mode

Enter multi-line programs interactively:

```bash
dx expr eval
> base = 10
> height = 5
> area = base * height / 2
> area
> ^D
25
```

---

# Part 4: Comparison and Logic

## Comparison Operators

All comparisons return `1.0` for true, `0.0` for false:

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `5 == 5` = 1 |
| `!=` | Not equal | `5 != 3` = 1 |
| `<` | Less than | `3 < 5` = 1 |
| `>` | Greater than | `5 > 3` = 1 |
| `<=` | Less or equal | `5 <= 5` = 1 |
| `>=` | Greater or equal | `5 >= 3` = 1 |

```bash
# Equality
dx expr eval "5 == 5"              # 1 (true)
dx expr eval "5 == 3"              # 0 (false)
dx expr eval "5 != 3"              # 1 (true)

# Ordering
dx expr eval "3 < 5"               # 1
dx expr eval "5 > 3"               # 1
dx expr eval "5 <= 5"              # 1
dx expr eval "5 >= 3"              # 1

# Use in expressions
dx expr eval "x = 10; y = 5; x > y"        # 1
dx expr eval "(3 < 5) + (7 > 2)"           # 2 (both true)
```

## Logical Operators

| Operator | Alternative | Description |
|----------|-------------|-------------|
| `and` | `&&` | Logical AND |
| `or` | `||` | Logical OR |
| `not` | `!` | Logical NOT |

Truthiness: `0` is false, any non-zero value is true.

```bash
# AND - both must be true
dx expr eval "1 and 1"             # 1
dx expr eval "1 and 0"             # 0
dx expr eval "1 && 1"              # 1

# OR - at least one must be true
dx expr eval "1 or 0"              # 1
dx expr eval "0 or 0"              # 0
dx expr eval "1 || 0"              # 1

# NOT - invert truthiness
dx expr eval "not 0"               # 1
dx expr eval "not 1"               # 0
dx expr eval "!0"                  # 1
dx expr eval "!42"                 # 0

# Combined
dx expr eval "5 > 3 and 2 < 4"     # 1
dx expr eval "5 > 3 or 2 > 4"      # 1
dx expr eval "not (5 < 3)"         # 1
```

## Short-Circuit Evaluation

Logical operators use short-circuit evaluation:
- `and`: if left is false, right is not evaluated
- `or`: if left is true, right is not evaluated

```bash
# Short-circuit prevents errors
dx expr eval "0 and (1/0)"         # 0 (no division by zero error)
dx expr eval "1 or (1/0)"          # 1 (no division by zero error)
```

---

# Part 5: Conditional Expressions

## If-Then-Else Syntax

```
if condition then expr1 else expr2
```

- If `condition` is non-zero (true), evaluates to `expr1`
- If `condition` is zero (false), evaluates to `expr2`

```bash
# Basic conditional
dx expr eval "if 1 then 100 else 200"      # 100
dx expr eval "if 0 then 100 else 200"      # 200

# With comparison
dx expr eval "x = 10; if x > 5 then 1 else 0"      # 1

# Return different values
dx expr eval "x = -5; if x < 0 then -x else x"     # 5 (absolute value)
```

## Nested Conditionals

```bash
# if-else if-else pattern
dx expr eval "x = 75; if x >= 90 then 4 else if x >= 80 then 3 else if x >= 70 then 2 else 1"
# Result: 2

# Nested in both branches
dx expr eval "a = 3; b = 5; if a > b then if a > 10 then 1 else 2 else if b > 10 then 3 else 4"
# Result: 4
```

## Common Patterns

```bash
# Absolute value
dx expr eval "x = -42; if x < 0 then -x else x"                # 42

# Maximum of two values
dx expr eval "a = 7; b = 12; if a > b then a else b"           # 12

# Minimum of two values
dx expr eval "a = 7; b = 12; if a < b then a else b"           # 7

# Sign function
dx expr eval "x = -5; if x > 0 then 1 else if x < 0 then -1 else 0"  # -1

# Clamp (manual)
dx expr eval "x = 15; lo = 0; hi = 10; if x < lo then lo else if x > hi then hi else x"  # 10
```

---

# Part 6: User-Defined Functions

## Function Definition Syntax

```
def name(param1, param2, ...) = expression
```

```bash
# Single parameter
dx expr eval "def square(x) = x * x; square(5)"                # 25

# Two parameters
dx expr eval "def add(a, b) = a + b; add(3, 4)"                # 7

# No parameters
dx expr eval "def answer() = 42; answer()"                     # 42
```

## Functions with Expressions

```bash
# Using built-in functions
dx expr eval "def hypotenuse(a, b) = sqrt(a^2 + b^2); hypotenuse(3, 4)"  # 5

# Using conditionals
dx expr eval "def myabs(x) = if x < 0 then -x else x; myabs(-42)"        # 42

# Using other functions
dx expr eval "def square(x) = x * x; def cube(x) = x * square(x); cube(3)"  # 27
```

## Recursive Functions

Functions can call themselves:

```bash
# Factorial
dx expr eval "def factorial(n) = if n <= 1 then 1 else n * factorial(n - 1); factorial(5)"
# Result: 120

# Fibonacci (exponential time - for demonstration)
dx expr eval "def fib(n) = if n <= 1 then n else fib(n-1) + fib(n-2); fib(10)"
# Result: 55

# GCD (greatest common divisor)
dx expr eval "def gcd(a, b) = if b == 0 then a else gcd(b, mod(a, b)); gcd(48, 18)"
# Result: 6
```

## Function Scope

Functions see other functions defined before or after them:

```bash
dx expr eval "
def iseven(n) = if n == 0 then 1 else isodd(n - 1)
def isodd(n) = if n == 0 then 0 else iseven(n - 1)
iseven(10)
"
# Result: 1
```

---

# Part 7: Lambda Expressions

## Lambda Syntax

```
param => expression           # Single parameter
(param1, param2) => expression  # Multiple parameters
() => expression              # No parameters
```

```bash
# Single parameter
dx expr eval "double = x => x * 2; double(5)"              # 10

# Multiple parameters
dx expr eval "add = (a, b) => a + b; add(3, 4)"            # 7

# No parameters
dx expr eval "answer = () => 42; answer()"                 # 42
```

## Lambdas vs Def Functions

Both create callable functions, but with different syntax:

```bash
# Equivalent definitions
dx expr eval "def square1(x) = x * x; square1(5)"          # 25
dx expr eval "square2 = x => x * x; square2(5)"            # 25

# Lambdas are expressions - can be used inline
dx expr eval "apply = (f, x) => f(x); sq = n => n * n; apply(sq, 5)"  # 25
```

## Lambda with Conditionals

```bash
dx expr eval "mymax = (a, b) => if a > b then a else b; mymax(3, 7)"  # 7
dx expr eval "clamp = (x, lo, hi) => if x < lo then lo else if x > hi then hi else x; clamp(15, 0, 10)"  # 10
```

---

# Part 8: Closures

## What Are Closures?

A closure is a function that captures variables from its surrounding scope. When a lambda or function references a variable that isn't a parameter, it "closes over" that variable.

```bash
# Lambda captures 'multiplier' from outer scope
dx expr eval "
multiplier = 10
scale = x => x * multiplier
scale(5)
"
# Result: 50
```

## Closure Behavior

Closures see the current value of captured variables:

```bash
dx expr eval "
factor = 2
double = x => x * factor
result1 = double(5)          # 10
factor = 3
result2 = double(5)          # 15
result1 + result2
"
# Result: 25
```

## Practical Closure Examples

```bash
# Counter pattern
dx expr eval "
count = 0
increment = () => { count = count + 1; count }
a = increment()    # 1
b = increment()    # 2
c = increment()    # 3
a + b + c
"
# Result: 6

# Configurable function factory
dx expr eval "
makeAdder = n => x => x + n
add5 = makeAdder(5)
add10 = makeAdder(10)
add5(3) + add10(3)
"
# Result: 21 (8 + 13)
```

---

# Part 9: Comments

## Comment Syntax

Comments start with `#` and continue to the end of the line:

```bash
# Full line comment
x = 5  # Inline comment
y = 10 # Another comment
x + y
```

## Using Comments

```bash
dx expr eval "
# Calculate area of a circle
radius = 5        # in meters
area = pi * radius ^ 2  # formula: pi * r^2
area  # return the result
"
# Result: 78.53981633974483
```

---

# Part 10: Running Programs

## From Command Line

```bash
# Single expression
dx expr eval "2 + 2"

# Multi-statement with semicolons
dx expr eval "x = 5; y = 10; x * y"

# Multi-line with quotes
dx expr eval "x = 5
y = 10
x * y"
```

## From Files

Create a file `circle.dx`:
```
# Calculate properties of a circle
radius = 5

diameter = radius * 2
circumference = pi * diameter
area = pi * radius ^ 2

print(circumference)
area
```

Run it:
```bash
dx expr run circle.dx
# Output:
# 31.41592653589793
# 78.53981633974483
```

## From Stdin

```bash
# Pipe a program
echo "x = 10; y = 20; x * y" | dx expr run -

# Here document
dx expr run - <<'EOF'
def factorial(n) = if n <= 1 then 1 else n * factorial(n - 1)
factorial(6)
EOF
# Result: 720
```

## Interactive Mode

```bash
dx expr eval
> x = 10
> def square(n) = n * n
> square(x) + x
> ^D
110
```

---

# Part 11: Practical Examples

## Mathematical Calculations

### Circle Properties
```bash
dx expr eval "r = 5; area = pi * r^2; circ = 2 * pi * r; area + circ"
```

### Pythagorean Theorem
```bash
dx expr eval "a = 3; b = 4; c = sqrt(a^2 + b^2); c"  # 5
```

### Quadratic Formula
```bash
dx expr eval "
# Solve x^2 - 5x + 6 = 0
a = 1; b = -5; c = 6
discriminant = b^2 - 4*a*c
x1 = (-b + sqrt(discriminant)) / (2*a)
x2 = (-b - sqrt(discriminant)) / (2*a)
print(x1)  # 3
x2         # 2
"
```

### Compound Interest
```bash
dx expr eval "
# A = P(1 + r/n)^(nt)
P = 1000      # Principal
r = 0.05      # 5% annual rate
n = 12        # Compounded monthly
t = 10        # 10 years
A = P * (1 + r/n)^(n*t)
A
"
# Result: 1647.0094976902823
```

## Unit Conversions

### Temperature
```bash
# Celsius to Fahrenheit
dx expr eval "c = 100; f = c * 9/5 + 32; f"  # 212

# Fahrenheit to Celsius
dx expr eval "f = 98.6; c = (f - 32) * 5/9; c"  # 37
```

### Distance
```bash
# Miles to kilometers
dx expr eval "miles = 26.2; km = miles * 1.60934; km"  # 42.164708

# Kilometers to miles
dx expr eval "km = 42; miles = km / 1.60934; miles"  # 26.097...
```

## Algorithms

### Factorial
```bash
dx expr eval "
def factorial(n) = if n <= 1 then 1 else n * factorial(n - 1)
factorial(10)
"
# Result: 3628800
```

### Fibonacci
```bash
dx expr eval "
def fib(n) = if n <= 1 then n else fib(n-1) + fib(n-2)
fib(12)
"
# Result: 144
```

### GCD and LCM
```bash
dx expr eval "
def gcd(a, b) = if b == 0 then a else gcd(b, mod(a, b))
def lcm(a, b) = abs(a * b) / gcd(a, b)
print(gcd(48, 18))   # 6
lcm(48, 18)          # 144
"
```

### Power (Integer Exponent)
```bash
dx expr eval "
def power(base, exp) = if exp == 0 then 1 else base * power(base, exp - 1)
power(2, 10)
"
# Result: 1024
```

## Statistics

### Mean, Variance, Standard Deviation
```bash
dx expr eval "
# Sample data
a = 10; b = 20; c = 30; d = 40; e = 50

# Mean
mean = (a + b + c + d + e) / 5

# Variance
variance = ((a-mean)^2 + (b-mean)^2 + (c-mean)^2 + (d-mean)^2 + (e-mean)^2) / 5

# Standard deviation
stddev = sqrt(variance)

print(mean)      # 30
print(variance)  # 200
stddev           # 14.142135623730951
"
```

## Physics

### Kinematic Equations
```bash
dx expr eval "
# v = v0 + at
# s = v0*t + 0.5*a*t^2
v0 = 0        # Initial velocity (m/s)
a = 9.8       # Acceleration (m/s^2)
t = 5         # Time (seconds)

v = v0 + a * t
s = v0 * t + 0.5 * a * t^2

print(v)      # 49 m/s
s             # 122.5 meters
"
```

### Energy
```bash
dx expr eval "
# Kinetic energy: KE = 0.5 * m * v^2
m = 1000      # Mass in kg
v = 30        # Velocity in m/s
KE = 0.5 * m * v^2
KE            # 450000 Joules
"
```

---

# Reference

## Complete Operator Table

| Precedence | Operator | Description | Associativity |
|------------|----------|-------------|---------------|
| 1 (lowest) | `or`, `\|\|` | Logical OR | Left |
| 2 | `and`, `&&` | Logical AND | Left |
| 3 | `==`, `!=` | Equality | Left |
| 4 | `<`, `>`, `<=`, `>=` | Comparison | Left |
| 5 | `+`, `-` | Addition, Subtraction | Left |
| 6 | `*`, `/`, `%` | Multiply, Divide, Modulo | Left |
| 7 | `^`, `**` | Power | Right |
| 8 (highest) | `-`, `not`, `!` | Unary | Right |

## Complete Function Table

### Single Argument
| Function | Description |
|----------|-------------|
| `sin(x)`, `cos(x)`, `tan(x)` | Trigonometric |
| `asin(x)`, `acos(x)`, `atan(x)` | Inverse trig |
| `sinh(x)`, `cosh(x)`, `tanh(x)` | Hyperbolic |
| `sqrt(x)`, `cbrt(x)` | Roots |
| `floor(x)`, `ceil(x)`, `round(x)`, `trunc(x)` | Rounding |
| `abs(x)`, `sign(x)`, `fract(x)` | Numeric |
| `exp(x)`, `ln(x)`, `log2(x)`, `log10(x)` | Exponential/Log |
| `print(x)` | Debug output |

### Two Arguments
| Function | Description |
|----------|-------------|
| `max(a, b)`, `min(a, b)` | Maximum, Minimum |
| `pow(x, y)` | Power |
| `atan2(y, x)` | Two-arg arctangent |
| `hypot(a, b)` | Hypotenuse |
| `log(x, base)` | Logarithm with base |
| `mod(a, b)` | Modulo |

### Three Arguments
| Function | Description |
|----------|-------------|
| `clamp(x, lo, hi)` | Constrain to range |
| `lerp(a, b, t)` | Linear interpolation |

### Variadic
| Function | Description |
|----------|-------------|
| `sum(...)` | Sum all arguments |
| `avg(...)` | Average of arguments |

## Reserved Words

### Constants
`pi`, `e`, `tau`, `true`, `false`

### Keywords
`if`, `then`, `else`, `def`, `and`, `or`, `not`

### Built-in Functions
All function names listed above are reserved.

## Error Messages

| Error | Cause |
|-------|-------|
| "Division by zero" | Dividing by 0 |
| "Square root of negative number" | `sqrt(-n)` |
| "Undefined variable: x" | Using undeclared variable |
| "Unknown function: foo" | Calling undefined function |
| "Cannot assign to constant: pi" | Trying to assign to constant |
| "Cannot assign to function name: sin" | Trying to shadow built-in |
| "Cannot assign to keyword: if" | Using keyword as variable |
| "Expected N arguments, got M" | Wrong argument count |
| "Unexpected character: @" | Invalid syntax |
| "Unexpected end of expression" | Incomplete expression |

---

## AST Output

View the parsed structure of any expression:

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

The expression evaluator is available as a Rust library:

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
// area = 78.53981633974483

// Access variables after evaluation
assert_eq!(ctx.get("radius"), Some(5.0));
```

---

## Tips and Tricks

### Use Parentheses for Clarity
```bash
dx expr eval "(2 + 3) * 4"         # Clear: add first, then multiply
dx expr eval "2 ^ (3 ^ 2)"         # Clear: exponent first
```

### Debug with Print
```bash
dx expr eval "
x = 5
print(x)      # See intermediate value
y = x * 2
print(y)      # See intermediate value
y + 10
"
```

### Combine with Shell Scripts
```bash
# Use in calculations
result=$(dx expr eval "sqrt(2)")
echo "Square root of 2 is $result"

# Generate sequences
for i in 1 2 3 4 5; do
  echo "2^$i = $(dx expr eval "2^$i")"
done
```

### Reusable Script Files
Create math libraries in `.dx` files and run them:

```bash
# mathlib.dx
def square(x) = x * x
def cube(x) = x * x * x
def pythag(a, b) = sqrt(a^2 + b^2)

pythag(3, 4)
```

```bash
dx expr run mathlib.dx  # 5
```
