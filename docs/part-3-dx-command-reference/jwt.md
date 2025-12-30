# jwt - JWT Token Handling

Decode, encode, and verify JSON Web Tokens.

## Subcommands

| Command | Description |
|---------|-------------|
| `decode` | Decode a JWT token (without verification) |
| `encode` | Create a JWT token |
| `verify` | Verify a JWT token |

## Usage

```bash
# Decode a JWT (no verification)
dx jwt decode "eyJhbGciOiJIUzI1NiIs..."

# Create a JWT
dx jwt encode --secret "my-secret" --payload '{"sub":"1234","name":"John"}'

# Verify a JWT
dx jwt verify "eyJhbG..." --secret "my-secret"
```

## Examples

### Decode JWT

```bash
# Decode without verification
dx jwt decode "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"

# Output:
# Header:
#   alg: HS256
#   typ: JWT
# Payload:
#   sub: 1234567890
#   name: John Doe
#   iat: 1516239022

# Output as JSON
dx jwt decode "eyJhbG..." -o json
```

### Create JWT

```bash
# Create with HS256 (default)
dx jwt encode --secret "my-secret-key" --payload '{"user_id":123,"role":"admin"}'

# Create with expiration
dx jwt encode --secret "key" --payload '{"sub":"user"}' --exp 3600

# Create with custom algorithm
dx jwt encode --secret "key" --payload '{}' --algorithm HS384
```

### Verify JWT

```bash
# Verify token signature
dx jwt verify "eyJhbG..." --secret "my-secret-key"

# Verify and check claims
dx jwt verify "eyJhbG..." --secret "key" --validate-exp
```

## JWT Structure

A JWT consists of three parts separated by dots:

```
header.payload.signature
```

- **Header**: Algorithm and token type
- **Payload**: Claims (data)
- **Signature**: Verification hash

## Options

| Option | Description |
|--------|-------------|
| `--secret` | Secret key for signing/verification |
| `--payload` | JSON payload for encoding |
| `--algorithm` | Algorithm (HS256, HS384, HS512) |
| `--exp` | Expiration time in seconds |
| `--validate-exp` | Validate expiration during verify |
| `--no-color` | Disable colored output |
| `-o, --output` | Output format (text, json, quiet) |

## See Also

- [encode](./encode.md) - Base64 encoding
- [hash](./hash.md) - Cryptographic hashing
