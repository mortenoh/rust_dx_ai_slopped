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
# Hash a file (SHA-256)
dx hash myfile.txt

# Use MD5
dx hash -a md5 myfile.txt

# SHA-512 with base64 output
dx hash -a sha512 -o base64 myfile.txt

# Hash from stdin
echo "hello" | dx hash -

# Verify a hash
dx hash --verify e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 myfile.txt

# Hash multiple files
for f in *.txt; do dx hash "$f"; done
```

## Output Format

Default output includes filename:
```
sha256: e3b0c44298fc1c14... myfile.txt
```

With `--quiet`:
```
e3b0c44298fc1c14...
```

## Verification

Returns exit code 0 if hash matches, 1 if not:
```bash
dx hash --verify abc123 file.txt && echo "OK" || echo "MISMATCH"
```
