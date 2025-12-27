# net

Network-related utilities for developers.

## Usage

```bash
dx net <SUBCOMMAND>
```

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `ip` | Show IP addresses |
| `url` | Parse URL components |
| `port` | Check if port is open |
| `lookup` | DNS lookup |

---

## ip

Display local or public IP addresses.

```bash
dx net ip [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--public` | Show public IP address (requires internet) |

### Local IPs (default)

Shows all non-loopback network interfaces:

```bash
dx net ip
# local: 192.168.1.100
# en0: 192.168.1.100
# en1: 10.0.0.50
```

### Public IP

Fetches your public IP from an external service:

```bash
dx net ip --public
# public: fetching...
# public: 203.0.113.42
```

---

## url

Parse and display URL components.

```bash
dx net url <URL>
```

### Output

Breaks down the URL into its components:

```bash
dx net url "https://user:pass@example.com:8080/path/to/page?q=search&page=1#section"
# scheme: https
# host: example.com
# port: 8080
# path: /path/to/page
# query: q=search&page=1
#   q: search
#   page: 1
# fragment: section
# username: user
```

### Default Ports

Shows default ports for known schemes when not specified:

```bash
dx net url "https://example.com/api"
# scheme: https
# host: example.com
# port: 443 (default)
# path: /api
```

| Scheme | Default Port |
|--------|--------------|
| http | 80 |
| https | 443 |
| ftp | 21 |
| ssh | 22 |

### Examples

```bash
# API endpoint
dx net url "https://api.example.com/v1/users?limit=10"

# Local development
dx net url "http://localhost:3000/dashboard"

# Complex URL
dx net url "postgresql://user:secret@db.host:5432/mydb?ssl=true"
```

---

## port

Check if a TCP port is open/listening.

```bash
dx net port <PORT> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `PORT` | Port number to check |
| `--host` | Host to check (default: localhost) |

### Examples

```bash
# Check local port
dx net port 8080
# ✓ Port 8080 is open on localhost

dx net port 9999
# ✗ Port 9999 is closed on localhost

# Check remote host
dx net port 443 --host google.com
# ✓ Port 443 is open on google.com

dx net port 22 --host github.com
# ✓ Port 22 is open on github.com
```

### Use Cases

```bash
# Check if web server is running
dx net port 80

# Check if database is accessible
dx net port 5432 --host db.example.com

# Check SSH access
dx net port 22 --host myserver.com
```

---

## lookup

Perform DNS lookup for a domain.

```bash
dx net lookup <DOMAIN>
```

### Examples

```bash
dx net lookup google.com
# domain: google.com
#   142.250.185.78
#   2607:f8b0:4004:800::200e

dx net lookup localhost
# domain: localhost
#   127.0.0.1

dx net lookup nonexistent.invalid
# ✗ Could not resolve nonexistent.invalid: ...
```

---

## Examples

```bash
# Development workflow
dx net ip                      # Find your local IP
dx net port 3000               # Check if dev server is running
dx net url "$API_URL"          # Parse API endpoint

# Debugging
dx net lookup api.example.com  # Check DNS resolution
dx net port 443 --host api.example.com  # Check connectivity

# Network info
dx net ip --public             # Get public IP for firewall rules
dx net lookup mysite.com       # Verify DNS propagation
```
