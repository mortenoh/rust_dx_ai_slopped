# dx - Developer Toolkit

**dx** is a comprehensive command-line toolkit for common developer tasks.

## Features

- **hash** - Calculate file hashes (MD5, SHA-256, SHA-512)
- **encode** - Base64 and hex encoding/decoding
- **uuid** - Generate UUIDs (v4 random, v7 time-ordered)
- **time** - Timestamp parsing and conversion
- **json** - Format, validate, and query JSON
- **env** - Environment variable management
- **config** - Application configuration

## Quick Example

```bash
# Hash a file
dx hash myfile.txt

# Encode to base64
dx encode base64 "hello world"

# Generate a UUID
dx uuid

# Format JSON
dx json format data.json
```

## Installation

```bash
cargo install dx
```

Or download pre-built binaries from the [releases page](https://github.com/user/dx/releases).
