# env

Manage and display environment variables.

## Usage

```bash
dx env [COMMAND] [OPTIONS]
```

## Subcommands

| Command | Description |
|---------|-------------|
| `list` | List all variables |
| `get <NAME>` | Get a variable |
| `set <NAME> <VALUE>` | Set a variable (current shell) |
| `export` | Export as shell commands |

## Options

| Option | Description |
|--------|-------------|
| `-f, --filter <PATTERN>` | Filter by pattern |
| `--format <FMT>` | Output format: `table`, `json`, `shell` |
| `--sort` | Sort alphabetically |

## Examples

### List Variables

```bash
# List all
dx env list

# Filter by prefix
dx env list -f "PATH*"
dx env list -f "AWS_*"

# As JSON
dx env list --format json

# Sorted
dx env list --sort
```

### Get Variable

```bash
# Get specific variable
dx env get HOME
# /Users/alice

# With default
dx env get MY_VAR --default "not set"

# Check if set
if dx env get MY_VAR >/dev/null 2>&1; then
    echo "MY_VAR is set"
fi
```

### Export Format

```bash
# Export for shell
dx env list -f "MY_*" --format shell
# export MY_VAR="value"
# export MY_OTHER="other"

# Source in script
eval "$(dx env list -f 'CONFIG_*' --format shell)"
```

### Working with .env Files

```bash
# Load .env file
dx env load .env

# Show what would be loaded
dx env load .env --dry-run

# Export .env format
dx env list -f "APP_*" --format dotenv > .env
```

## Integration

```bash
# Pass to subprocess
dx env list -f "AWS_*" --format shell | source /dev/stdin && aws s3 ls

# Compare environments
diff <(dx env list --sort) <(ssh server 'env | sort')
```
