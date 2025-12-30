# Commands Overview

dx is a developer toolkit providing common utilities for everyday tasks.

## Available Commands

### Data & Encoding

| Command | Alias | Description |
|---------|-------|-------------|
| [hash](./hash.md) | `h` | Cryptographic hashing (MD5, SHA, Bcrypt, Argon2) |
| [encode](./encode.md) | `e` | Base64/hex encoding and decoding |
| [uuid](./uuid.md) | `u` | Generate UUIDs (v4, v7) and ULIDs |
| [rand](./rand.md) | `r` | Random generation (numbers, strings, passwords) |
| [encrypt](./encrypt.md) | - | Encrypt/decrypt with AES-GCM or ChaCha20 |
| [jwt](./jwt.md) | - | JWT token handling |

### Data Formats

| Command | Alias | Description |
|---------|-------|-------------|
| [json](./json.md) | `j` | JSON formatting, validation, querying |
| [yaml](./yaml.md) | `y` | YAML formatting and conversion |
| [csv](./csv.md) | - | CSV formatting and conversion |
| [xml](./xml.md) | - | XML formatting and conversion |

### Text Processing

| Command | Alias | Description |
|---------|-------|-------------|
| [text](./text.md) | - | Text transformations (case, slugify) |
| [grep](./grep.md) | `g` | Regex search in files with context |
| [diff](./diff.md) | - | Text diffing (unified, inline) |
| [template](./template.md) | - | Jinja2-style template rendering |
| [markdown](./markdown.md) | `md` | Markdown to HTML, TOC extraction |
| [compress](./compress.md) | - | Gzip/Zstd compression |

### Time & Math

| Command | Alias | Description |
|---------|-------|-------------|
| [time](./time.md) | `t` | Timestamp conversion and formatting |
| [calc](./calc.md) | `c` | Unit conversions (bytes, time, base) |
| [expr](./expr.md) | `x` | Expression evaluator with functions |

### System & Network

| Command | Alias | Description |
|---------|-------|-------------|
| [env](./env.md) | - | Environment variable management |
| [config](./config.md) | `cfg` | Configuration management |
| [system](./system.md) | `sys` | System information (CPU, memory, OS) |
| [net](./net.md) | - | Network utilities (IP, DNS, ports) |
| [http](./http.md) | - | HTTP client (GET, POST, PUT, DELETE) |

### Files & Development

| Command | Alias | Description |
|---------|-------|-------------|
| [watch](./watch.md) | `w` | Watch files and run commands |
| [polars](./polars.md) | `pl` | DataFrame operations and data generation |
| [chat](./chat.md) | - | gRPC-based real-time chat |

### Specialized

| Command | Alias | Description |
|---------|-------|-------------|
| [dhis2](./dhis2.md) | - | DHIS2 API utilities |
| [fun](./fun.md) | - | Terminal effects (matrix, life, qr, clock) |
| [ui](./ui.md) | - | Interactive TUI dashboard |
| [egui](./egui.md) | - | Native GUI demos |
| [completions](./completions.md) | - | Generate shell completions |

## Global Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Show help information |
| `-V, --version` | Show version |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Increase verbosity |
| `-q, --quiet` | Suppress non-error output |

## Getting Help

```bash
dx --help              # General help
dx <command> --help    # Command-specific help
dx help <command>      # Alternative syntax
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | I/O error |
