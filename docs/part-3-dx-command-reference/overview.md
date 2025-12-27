# Commands Overview

dx is a developer toolkit providing common utilities for everyday tasks.

## Available Commands

| Command | Description |
|---------|-------------|
| [hash](./hash.md) | Calculate cryptographic file hashes |
| [encode](./encode.md) | Base64/hex encoding and decoding |
| [uuid](./uuid.md) | Generate UUIDs (v4, v7) |
| [time](./time.md) | Timestamp conversion and formatting |
| [json](./json.md) | JSON formatting and validation |
| [env](./env.md) | Environment variable management |
| [config](./config.md) | Configuration management |

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
