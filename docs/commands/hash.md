# hash

Calculate cryptographic hashes of files or stdin.

## Usage

```bash
dx hash [OPTIONS] <FILE>
dx hash [OPTIONS] -
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<FILE>` | Path to file, or `-` for stdin |

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `-a, --algorithm <ALG>` | `sha256` | Algorithm: `md5`, `sha256`, `sha512` |
| `-o, --output <FMT>` | `hex` | Output: `hex`, `base64` |
| `--verify <HASH>` | | Verify against expected hash |
| `-q, --quiet` | | Only output the hash |

## Examples

```bash
# Hash a file
dx hash myfile.txt

# Use MD5
dx hash -a md5 myfile.txt

# Hash from stdin
echo "hello" | dx hash -

# Verify a hash
dx hash --verify abc123... myfile.txt
```
