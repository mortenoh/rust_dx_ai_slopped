# calc

Evaluate math expressions and convert units.

## Usage

```bash
dx calc <SUBCOMMAND>
```

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `eval` | Evaluate math expression |
| `bytes` | Convert byte sizes |
| `time` | Convert time durations |
| `percent` | Calculate percentages |
| `base` | Convert number bases |

---

## eval

Evaluate mathematical expressions.

```bash
dx calc eval <EXPRESSION>
```

### Supported Operations

| Operation | Example |
|-----------|---------|
| Basic math | `2 + 2`, `10 - 3`, `4 * 5`, `15 / 3` |
| Exponents | `2^10`, `pow(2, 10)` |
| Square root | `sqrt(16)` |
| Trigonometry | `sin(0)`, `cos(0)`, `tan(0)` |
| Logarithms | `ln(e)`, `log10(100)` |
| Constants | `pi`, `e` |
| Parentheses | `(2 + 3) * 4` |

### Examples

```bash
dx calc eval "2 + 2"           # 4
dx calc eval "sqrt(16) * 2"    # 8
dx calc eval "2^10"            # 1024
dx calc eval "pi * 2"          # 6.283185...
dx calc eval "(100 - 20) / 4"  # 20
dx calc eval "sin(pi/2)"       # 1
```

---

## bytes

Convert between byte size units.

```bash
dx calc bytes <VALUE>
```

### Supported Formats

| Format | Example |
|--------|---------|
| Plain bytes | `1024` |
| Kilobytes | `1kb`, `1KB` |
| Megabytes | `1mb`, `1MB` |
| Gigabytes | `1.5gb`, `1.5GB` |
| Terabytes | `1tb`, `1TB` |

### Output

Shows the value in all common units:

```bash
dx calc bytes 1gb
# bytes: 1073741824
# KB: 1048576.00
# MB: 1024.00
# GB: 1.00
# TB: 0.0010
# human: 1.0 GiB
```

### Examples

```bash
dx calc bytes 1024             # 1024 bytes
dx calc bytes 1.5gb            # 1.5 gigabytes
dx calc bytes 500mb            # 500 megabytes
dx calc bytes 2tb              # 2 terabytes
```

---

## time

Convert between time duration units.

```bash
dx calc time <VALUE>
```

### Supported Formats

| Format | Example |
|--------|---------|
| Seconds | `3600`, `3600s` |
| Minutes | `90m` |
| Hours | `2h` |
| Days | `1d` |
| Combined | `1h30m`, `1d12h` |
| Humantime | `"2 hours 30 minutes"` |

### Output

Shows the value in multiple units plus human-readable:

```bash
dx calc time 3665s
# seconds: 3665
# minutes: 61.083...
# hours: 1.018...
# days: 0.042...
# human: 1h 1m 5s
```

### Examples

```bash
dx calc time 3600              # 3600 seconds
dx calc time 90m               # 90 minutes
dx calc time 1h30m             # 1 hour 30 minutes
dx calc time 86400s            # 1 day in seconds
dx calc time "2 days 3 hours"  # Humantime format
```

---

## percent

Calculate what percentage one value is of another.

```bash
dx calc percent <VALUE> of <TOTAL>
```

```bash
dx calc percent 15 of 200      # 7.50%
dx calc percent 25 of 100      # 25.00%
dx calc percent 1 of 3         # 33.33%
dx calc percent 750 of 1000    # 75.00%
```

---

## base

Convert numbers between different bases (radixes).

```bash
dx calc base <NUMBER> <FROM_BASE> <TO_BASE>
```

Supports bases 2 through 36.

| Base | Name |
|------|------|
| 2 | Binary |
| 8 | Octal |
| 10 | Decimal |
| 16 | Hexadecimal |

### Examples

```bash
# Decimal to hexadecimal
dx calc base 255 10 16         # ff

# Hexadecimal to decimal
dx calc base ff 16 10          # 255

# Decimal to binary
dx calc base 42 10 2           # 101010

# Binary to decimal
dx calc base 1010 2 10         # 10

# Octal to hexadecimal
dx calc base 777 8 16          # 1ff
```

---

## Examples

```bash
# Quick calculations
dx calc eval "1920 * 1080"     # Screen pixels
dx calc eval "365 * 24"        # Hours in a year

# Storage conversions
dx calc bytes 4.7gb            # DVD size
dx calc bytes 25gb             # Blu-ray size

# Time conversions
dx calc time 7200s             # Movie length
dx calc time 40h               # Work week

# Percentage calculations
dx calc percent 85 of 100      # Test score
dx calc percent 35 of 200      # Discount amount

# Programmer conversions
dx calc base 192 10 16         # IP octet to hex
dx calc base deadbeef 16 10    # Hex to decimal
```
