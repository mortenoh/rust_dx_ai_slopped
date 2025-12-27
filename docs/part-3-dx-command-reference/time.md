# time

Convert and format timestamps.

## Usage

```bash
dx time [COMMAND] [OPTIONS]
```

## Subcommands

| Command | Description |
|---------|-------------|
| `now` | Show current time |
| `parse <INPUT>` | Parse a timestamp |
| `convert <TIMESTAMP>` | Convert between formats |

## Options

| Option | Description |
|--------|-------------|
| `-f, --format <FMT>` | Output format |
| `-z, --timezone <TZ>` | Timezone (default: local) |
| `--utc` | Output in UTC |

## Formats

| Format | Example |
|--------|---------|
| `rfc3339` | `2024-01-15T10:30:00Z` |
| `rfc2822` | `Mon, 15 Jan 2024 10:30:00 +0000` |
| `unix` | `1705315800` |
| `unix-ms` | `1705315800000` |
| `human` | `2 hours ago` |
| Custom | `%Y-%m-%d %H:%M:%S` |

## Examples

```bash
# Current time
dx time now
dx time now -f unix
dx time now -f rfc3339 --utc

# Parse timestamp
dx time parse "2024-01-15T10:30:00Z"
dx time parse "1705315800"
dx time parse "Jan 15, 2024"

# Convert formats
dx time convert 1705315800 -f rfc3339
dx time convert "2024-01-15T10:30:00Z" -f unix

# With timezone
dx time now -z "America/New_York"
dx time convert 1705315800 -z "Europe/London"

# Custom format
dx time now -f "%Y-%m-%d"
dx time now -f "%H:%M:%S"
```

## Format Specifiers

| Specifier | Meaning | Example |
|-----------|---------|---------|
| `%Y` | Year | 2024 |
| `%m` | Month | 01 |
| `%d` | Day | 15 |
| `%H` | Hour (24h) | 14 |
| `%M` | Minute | 30 |
| `%S` | Second | 45 |
| `%z` | Timezone | +0000 |
