# compress - Compression Utilities

Compress and decompress files using Gzip or Zstd.

## Subcommands

| Command | Description |
|---------|-------------|
| `compress` | Compress a file |
| `decompress` | Decompress a file |

## Usage

```bash
# Compress a file (creates file.gz)
dx compress compress file.txt

# Decompress a file
dx compress decompress file.txt.gz

# Compress with Zstd
dx compress compress file.txt --format zstd
```

## Examples

### Compress Files

```bash
# Compress with Gzip (default)
dx compress compress document.txt
# Creates: document.txt.gz

# Compress with Zstd (faster, better compression)
dx compress compress document.txt --format zstd
# Creates: document.txt.zst

# Compress with custom output name
dx compress compress data.json -o data.json.compressed

# Compress from stdin
cat large.log | dx compress compress - -o logs.gz
```

### Decompress Files

```bash
# Decompress Gzip
dx compress decompress document.txt.gz
# Creates: document.txt

# Decompress Zstd
dx compress decompress document.txt.zst

# Decompress to stdout
dx compress decompress data.gz -o -
```

### Compression Levels

```bash
# Fast compression (less compression, faster)
dx compress compress file.txt --level 1

# Best compression (more compression, slower)
dx compress compress file.txt --level 9

# Default level (balanced)
dx compress compress file.txt --level 6
```

## Compression Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| `gzip` | `.gz` | Standard gzip (default), widely compatible |
| `zstd` | `.zst` | Zstandard, faster with better compression |

### Comparison

| Metric | Gzip | Zstd |
|--------|------|------|
| Speed | Good | Excellent |
| Ratio | Good | Better |
| Memory | Low | Medium |
| Support | Universal | Growing |

## Options

| Option | Description |
|--------|-------------|
| `--format` | Compression format (gzip, zstd) |
| `--level` | Compression level (1-9, default: 6) |
| `-o, --output` | Output file (- for stdout) |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |

## See Also

- [encode](./encode.md) - Base64 encoding
