# encode

Encode and decode data using Base64 or hexadecimal.

## Usage

```bash
dx encode <FORMAT> [OPTIONS] <INPUT>
dx encode <FORMAT> [OPTIONS] -d <ENCODED>
```

## Subcommands

| Format | Description |
|--------|-------------|
| `base64` | Base64 encoding |
| `hex` | Hexadecimal encoding |
| `url` | URL encoding |

## Options

| Option | Description |
|--------|-------------|
| `-d, --decode` | Decode instead of encode |
| `-f, --file <FILE>` | Read input from file |
| `--no-newline` | Don't add trailing newline |

## Examples

### Base64

```bash
# Encode string
dx encode base64 "hello world"
# aGVsbG8gd29ybGQ=

# Decode
dx encode base64 -d "aGVsbG8gd29ybGQ="
# hello world

# Encode file contents
dx encode base64 -f image.png

# Decode to file
dx encode base64 -d "..." > output.bin
```

### Hexadecimal

```bash
# Encode
dx encode hex "hello"
# 68656c6c6f

# Decode
dx encode hex -d "68656c6c6f"
# hello
```

### URL Encoding

```bash
# Encode
dx encode url "hello world!"
# hello%20world%21

# Decode
dx encode url -d "hello%20world%21"
# hello world!
```

## Piping

```bash
# Encode from stdin
echo "secret" | dx encode base64

# Chain encode/decode
echo "test" | dx encode base64 | dx encode base64 -d
```
