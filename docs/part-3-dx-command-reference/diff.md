# diff - Text Diffing

Compare files and show differences in various formats.

## Usage

```bash
dx diff <FILE1> <FILE2> [OPTIONS]
```

## Examples

### Basic Diff

```bash
# Compare two files (unified format)
dx diff old.txt new.txt

# Output:
# --- old.txt
# +++ new.txt
# @@ -1,3 +1,4 @@
#  line 1
# -line 2
# +line 2 modified
# +line 2.5 added
#  line 3
```

### Different Formats

```bash
# Unified diff (default)
dx diff file1.txt file2.txt --format unified

# Inline diff (side-by-side style)
dx diff file1.txt file2.txt --format inline

# Compact diff (minimal output)
dx diff file1.txt file2.txt --format compact
```

### Context Lines

```bash
# Show 5 lines of context
dx diff old.txt new.txt -C 5

# Show no context
dx diff old.txt new.txt -C 0
```

### Comparing Code

```bash
# Compare source files
dx diff main.rs.old main.rs

# Compare configs
dx diff config.yaml.bak config.yaml
```

## Output Formats

| Format | Description |
|--------|-------------|
| `unified` | Standard unified diff format (default) |
| `inline` | Inline highlighting of changes |
| `compact` | Minimal output showing only changes |

## Options

| Option | Description |
|--------|-------------|
| `--format` | Output format (unified, inline, compact) |
| `-C, --context` | Number of context lines (default: 3) |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-o, --output` | Output format (text, json, quiet) |

## See Also

- [grep](./grep.md) - Text search
- [text](./text.md) - Text transformations
