# Quick Start

## Basic Usage

```bash
# Get help
dx --help

# Get help for a command
dx hash --help
```

## Common Tasks

### Hashing Files

```bash
# SHA-256 (default)
dx hash myfile.txt

# MD5
dx hash -a md5 myfile.txt

# Hash from stdin
echo "hello" | dx hash -
```

### Encoding/Decoding

```bash
# Encode to base64
dx encode base64 "hello world"

# Decode from base64
dx encode base64 -d "aGVsbG8gd29ybGQ="

# Hex encoding
dx encode hex "hello"
```

### UUID Generation

```bash
# Random UUID (v4)
dx uuid

# Time-ordered UUID (v7)
dx uuid -t v7

# Generate multiple
dx uuid -c 5
```

### JSON Operations

```bash
# Format JSON
dx json format data.json

# Validate JSON
dx json validate data.json

# Minify JSON
dx json minify data.json
```

### Timestamps

```bash
# Current time
dx time now

# Parse timestamp
dx time parse "2024-01-15T10:30:00Z"

# Convert formats
dx time convert 1705315800 --to rfc3339
```
