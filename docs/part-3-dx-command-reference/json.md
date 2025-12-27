# json

Format, validate, and query JSON data.

## Usage

```bash
dx json <COMMAND> [OPTIONS] [FILE]
```

## Subcommands

| Command | Description |
|---------|-------------|
| `format` | Pretty-print JSON |
| `minify` | Compact JSON |
| `validate` | Check if valid JSON |
| `query` | Query with JSONPath |

## Options

| Option | Description |
|--------|-------------|
| `-i, --indent <N>` | Indentation spaces (default: 2) |
| `-s, --sort-keys` | Sort object keys |
| `-c, --color` | Colorize output |

## Examples

### Format (Pretty Print)

```bash
# Format file
dx json format data.json

# Format with 4-space indent
dx json format -i 4 data.json

# Sort keys
dx json format -s data.json

# From stdin
echo '{"b":2,"a":1}' | dx json format -s
# {
#   "a": 1,
#   "b": 2
# }
```

### Minify

```bash
# Minify file
dx json minify data.json

# From stdin
cat data.json | dx json minify
```

### Validate

```bash
# Check if valid
dx json validate data.json
# Valid JSON

# Invalid JSON
dx json validate broken.json
# Error: Invalid JSON at line 5, column 12

# Use in scripts
if dx json validate data.json; then
    echo "Valid"
fi
```

### Query (JSONPath)

```bash
# Get nested value
dx json query '.users[0].name' data.json

# Get all names
dx json query '.users[*].name' data.json

# Filter
dx json query '.users[?(@.age > 30)]' data.json
```

## Piping with Other Commands

```bash
# Format API response
curl -s api.example.com/data | dx json format

# Extract and process
curl -s api.example.com/users | dx json query '.users[*].email'
```
