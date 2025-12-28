# http - HTTP Client

The `http` command makes HTTP requests from the command line.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `get` | Make GET request |
| `post` | Make POST request |
| `put` | Make PUT request |
| `delete` | Make DELETE request |
| `head` | Make HEAD request |

## Common Options

| Option | Description |
|--------|-------------|
| `-H`, `--header` | Add header (can be repeated) |
| `-d`, `--data` | Request body data |
| `-t`, `--timeout` | Request timeout in seconds |
| `-v`, `--verbose` | Show request/response details |
| `--json` | Set Content-Type to application/json |

## Examples

### GET Requests
```bash
# Simple GET
dx http get https://api.github.com/zen

# With headers
dx http get https://api.example.com/data \
  -H "Authorization: Bearer token123"

# With timeout
dx http get https://slow-api.com/data -t 30
```

### POST Requests
```bash
# POST with JSON data
dx http post https://httpbin.org/post \
  -d '{"name": "test", "value": 42}' \
  --json

# POST with form data
dx http post https://httpbin.org/post \
  -d "name=test&value=42" \
  -H "Content-Type: application/x-www-form-urlencoded"

# POST with custom headers
dx http post https://api.example.com/users \
  -d '{"email": "user@example.com"}' \
  -H "Authorization: Bearer token" \
  -H "X-Custom-Header: value" \
  --json
```

### PUT Requests
```bash
dx http put https://api.example.com/users/123 \
  -d '{"name": "Updated Name"}' \
  --json
```

### DELETE Requests
```bash
dx http delete https://api.example.com/users/123 \
  -H "Authorization: Bearer token"
```

### HEAD Requests
```bash
# Check if resource exists
dx http head https://example.com/file.zip

# Check headers without downloading body
dx http head https://api.example.com/large-file
```

### Verbose Mode
```bash
dx http get https://httpbin.org/get -v
```

Shows:
- Request method and URL
- Request headers
- Response status
- Response headers
- Response body

## Output

By default, the command prints the response body. Use global `-o json` for structured output:

```bash
dx http get https://api.github.com/users/octocat -o json
```

## Error Handling

The command exits with non-zero status on:
- Connection errors
- Timeout
- HTTP 4xx/5xx responses (unless using `--allow-errors`)

```bash
# Check if API is available
if dx http head https://api.example.com/health; then
  echo "API is up"
fi
```
