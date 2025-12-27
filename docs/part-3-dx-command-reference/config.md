# config

Manage dx configuration.

## Usage

```bash
dx config <COMMAND> [OPTIONS]
```

## Subcommands

| Command | Description |
|---------|-------------|
| `path` | Show config file path |
| `list` | List all settings |
| `get <KEY>` | Get a setting |
| `set <KEY> <VALUE>` | Set a setting |
| `reset` | Reset to defaults |

## Config File Location

| Platform | Path |
|----------|------|
| Linux | `~/.config/dx/config.toml` |
| macOS | `~/Library/Application Support/com.example.dx/config.toml` |
| Windows | `C:\Users\<User>\AppData\Roaming\example\dx\config\config.toml` |

## Examples

### View Configuration

```bash
# Show config path
dx config path
# /home/alice/.config/dx/config.toml

# List all settings
dx config list

# Get specific setting
dx config get default_algorithm
# sha256
```

### Modify Configuration

```bash
# Set a value
dx config set default_algorithm sha512

# Set nested value
dx config set profiles.dev.api_url "http://localhost:3000"

# Reset to defaults
dx config reset
dx config reset default_algorithm  # Reset single key
```

## Configuration Options

```toml
# Default hash algorithm
default_algorithm = "sha256"

# Output format preferences
output_format = "hex"

# Enable/disable colors
colors = true

# Profiles for different environments
[profiles.dev]
api_url = "http://localhost:3000"
debug = true

[profiles.prod]
api_url = "https://api.example.com"
debug = false
```

## Environment Overrides

| Variable | Description |
|----------|-------------|
| `DX_CONFIG_DIR` | Override config directory |
| `DX_DEFAULT_ALGORITHM` | Override default algorithm |
| `NO_COLOR` | Disable colored output |

```bash
DX_DEFAULT_ALGORITHM=md5 dx hash file.txt
```
