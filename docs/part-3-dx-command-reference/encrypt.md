# encrypt - Encryption/Decryption

Encrypt and decrypt data using modern authenticated encryption.

## Subcommands

| Command | Description |
|---------|-------------|
| `encrypt` | Encrypt data |
| `decrypt` | Decrypt data |

## Usage

```bash
# Encrypt a string
dx encrypt encrypt -s "secret message" --password "mypassword"

# Decrypt a string
dx encrypt decrypt -s "encrypted-data..." --password "mypassword"

# Encrypt a file
dx encrypt encrypt -f secret.txt --password "mypassword" > secret.enc

# Decrypt a file
dx encrypt decrypt -f secret.enc --password "mypassword" > secret.txt
```

## Examples

### Encrypt Data

```bash
# Encrypt a string
dx encrypt encrypt -s "my secret data" --password "strong-password"

# Encrypt with AES-GCM (default)
dx encrypt encrypt -s "secret" --password "pw" --algorithm aes-gcm

# Encrypt with ChaCha20-Poly1305
dx encrypt encrypt -s "secret" --password "pw" --algorithm chacha20

# Encrypt from stdin
echo "secret data" | dx encrypt encrypt - --password "pw"
```

### Decrypt Data

```bash
# Decrypt a string
dx encrypt decrypt -s "base64-encrypted-data" --password "password"

# Decrypt from file
dx encrypt decrypt -f encrypted.bin --password "password"

# Decrypt to file
dx encrypt decrypt -f secret.enc --password "pw" > decrypted.txt
```

### File Encryption

```bash
# Encrypt a file
dx encrypt encrypt -f document.pdf --password "secure123" > document.pdf.enc

# Decrypt a file
dx encrypt decrypt -f document.pdf.enc --password "secure123" > document.pdf

# Pipeline encryption
cat data.json | dx encrypt encrypt - --password "pw" > data.json.enc
```

## Algorithms

| Algorithm | Description |
|-----------|-------------|
| `aes-gcm` | AES-256-GCM (default, NIST standard) |
| `chacha20` | ChaCha20-Poly1305 (modern, fast on mobile) |

Both algorithms provide authenticated encryption with associated data (AEAD).

## Security Notes

- Uses key derivation from password (Argon2 or similar)
- Generates random nonce for each encryption
- Output includes nonce + ciphertext + authentication tag
- Both algorithms are considered secure for modern use

## Options

| Option | Description |
|--------|-------------|
| `-s, --string` | String to encrypt/decrypt |
| `-f, --file` | File to encrypt/decrypt |
| `--password` | Password for key derivation |
| `--algorithm` | Encryption algorithm (aes-gcm, chacha20) |
| `--no-color` | Disable colored output |
| `-o, --output` | Output format (text, json, quiet) |

## See Also

- [hash](./hash.md) - Cryptographic hashing
- [encode](./encode.md) - Base64 encoding
