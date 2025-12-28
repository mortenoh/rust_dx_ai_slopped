# hash

Calculate cryptographic hashes of files, strings, or stdin. Supports both standard hash algorithms (MD5, SHA) and password hashing algorithms (Bcrypt, Argon2).

## Usage

```bash
dx hash [OPTIONS] <FILE>
dx hash [OPTIONS] -s <STRING>
dx hash [OPTIONS] -
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<FILE>` | Path to file, or `-` for stdin |

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `-a, --algorithm <ALG>` | `sha256` | Algorithm: `md5`, `sha256`, `sha512`, `bcrypt`, `argon2` |
| `-s, --string <TEXT>` | | Hash a string instead of a file |
| `--cost <N>` | `12` | Cost factor for bcrypt (4-31) or Argon2 |
| `--verify <HASH>` | | Verify against expected hash |
| `-q, --quiet` | | Only output the hash |

## Algorithms

### Standard Hash Functions

| Algorithm | Output | Use Case |
|-----------|--------|----------|
| `md5` | 128-bit hex | Checksums (not secure) |
| `sha256` | 256-bit hex | General purpose, file verification |
| `sha512` | 512-bit hex | Higher security margin |

### Password Hashing

| Algorithm | Output | Use Case |
|-----------|--------|----------|
| `bcrypt` | PHC format | Password storage, configurable cost |
| `argon2` | PHC format | Modern password hashing, memory-hard |

**Note:** Password hashing algorithms (bcrypt, argon2) include random salt in the output, so the same input produces different hashes each time. Use `--verify` to check passwords.

## Examples

### Standard Hashing

```bash
# Hash a file (SHA-256, default)
dx hash myfile.txt

# Use MD5
dx hash -a md5 myfile.txt

# SHA-512
dx hash -a sha512 myfile.txt

# Hash a string
dx hash -s "hello world"

# Hash from stdin
echo "hello" | dx hash -

# Verify a hash
dx hash --verify e3b0c44298fc1c14... myfile.txt
```

### Password Hashing

```bash
# Generate bcrypt hash (default cost 12)
dx hash -a bcrypt -s "mypassword"

# Bcrypt with custom cost (higher = slower/more secure)
dx hash -a bcrypt --cost 14 -s "mypassword"

# Generate argon2 hash
dx hash -a argon2 -s "mypassword"

# Verify a password against bcrypt hash
dx hash -a bcrypt -s "mypassword" --verify '$2b$12$LQv3c1yqBw...'

# Verify a password against argon2 hash
dx hash -a argon2 -s "mypassword" --verify '$argon2id$v=19$m=19456...'
```

## Output Format

Default output includes source and algorithm:
```
myfile.txt (SHA256) = e3b0c44298fc1c14...
```

With `--quiet`:
```
e3b0c44298fc1c14...
```

Bcrypt/Argon2 output is in PHC (Password Hashing Competition) format:
```
$2b$12$LQv3c1yqBwEHv2X9kx...  (bcrypt)
$argon2id$v=19$m=19456,t=2,p=1$...  (argon2)
```

## Verification

Returns exit code 0 if hash matches, 1 if not:

```bash
# Standard hash verification
dx hash --verify abc123... file.txt && echo "OK" || echo "MISMATCH"

# Password verification
dx hash -a bcrypt -s "password" --verify '$2b$12$...' && echo "Valid" || echo "Invalid"
```

## Security Notes

- **MD5**: Cryptographically broken. Use only for non-security checksums.
- **SHA-256/512**: Secure for file integrity and general hashing.
- **Bcrypt**: Good for passwords. Cost 10-12 is reasonable, 14+ for high security.
- **Argon2**: Modern standard for password hashing. Memory-hard, resistant to GPU attacks.
