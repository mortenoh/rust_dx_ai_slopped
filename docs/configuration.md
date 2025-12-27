# Configuration

## Config File Location

dx looks for configuration in:

| Platform | Path |
|----------|------|
| Linux | `~/.config/dx/config.toml` |
| macOS | `~/Library/Application Support/com.example.dx/config.toml` |
| Windows | `C:\Users\<User>\AppData\Roaming\example\dx\config\config.toml` |

## Config File Format

```toml
# Default hash algorithm
default_algorithm = "sha256"

# Output format
output_format = "hex"

# Enable colors
colors = true

[profiles.dev]
api_url = "http://localhost:3000"
debug = true

[profiles.prod]
api_url = "https://api.example.com"
debug = false
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `DX_CONFIG_DIR` | Override config directory |
| `DX_DEFAULT_ALGORITHM` | Default hash algorithm |
| `NO_COLOR` | Disable colored output |

## Managing Config

```bash
# Show config path
dx config path

# Get a value
dx config get default_algorithm

# Set a value
dx config set default_algorithm sha512

# List all settings
dx config list
```
