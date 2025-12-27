# uuid

Generate universally unique identifiers.

## Usage

```bash
dx uuid [OPTIONS]
```

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `-t, --type <VERSION>` | `v4` | UUID version: `v4`, `v7` |
| `-c, --count <N>` | `1` | Number of UUIDs to generate |
| `-u, --uppercase` | | Output in uppercase |
| `--no-hyphens` | | Remove hyphens |

## UUID Versions

### v4 (Random)
Fully random UUID. Best for most use cases.

```bash
dx uuid
# 550e8400-e29b-41d4-a716-446655440000
```

### v7 (Time-ordered)
Time-based with random suffix. Sortable by creation time.

```bash
dx uuid -t v7
# 018e5e4c-8f3a-7000-8000-000000000001
```

## Examples

```bash
# Generate one UUID
dx uuid

# Generate 5 UUIDs
dx uuid -c 5

# Time-ordered UUIDs
dx uuid -t v7 -c 3

# Uppercase without hyphens
dx uuid -u --no-hyphens
# 550E8400E29B41D4A716446655440000

# For scripts
ID=$(dx uuid)
echo "Created resource $ID"
```

## When to Use Each Version

| Version | Use Case |
|---------|----------|
| v4 | General purpose, unpredictable |
| v7 | Database keys, sortable, time-based |
