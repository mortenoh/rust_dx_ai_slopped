# dhis2 - DHIS2 API Utilities

Interact with DHIS2 health information systems via API.

## Overview

DHIS2 is a health management information system platform. This command provides utilities for interacting with DHIS2 instances.

## Configuration

Set connection details via environment variables or options:

```bash
# Environment variables
export DHIS2_URL="https://play.dhis2.org/40"
export DHIS2_USER="admin"
export DHIS2_PASSWORD="district"

# Or use command options
dx dhis2 --url https://play.dhis2.org/40 --user admin --password district ...
```

## Subcommands

| Command | Description |
|---------|-------------|
| `orgunits` | List organization units |
| `dataelements` | List data elements |
| `datavalues` | Query data values |
| `me` | Get current user info |
| `system` | Get system info |

## Examples

### Organization Units

```bash
# List all organization units
dx dhis2 orgunits

# Filter by level
dx dhis2 orgunits --level 2

# Filter by parent
dx dhis2 orgunits --parent "ImspTQPwCqd"

# Search by name
dx dhis2 orgunits --query "District"
```

### Data Elements

```bash
# List all data elements
dx dhis2 dataelements

# Filter by domain
dx dhis2 dataelements --domain AGGREGATE

# Search by name
dx dhis2 dataelements --query "Malaria"
```

### Data Values

```bash
# Query data values
dx dhis2 datavalues --orgunit "ImspTQPwCqd" --period 202301

# Query specific data element
dx dhis2 datavalues --dataelement "fbfJHSPpUQD" --period 2023

# Export as JSON
dx dhis2 datavalues --orgunit "..." -o json
```

### System Information

```bash
# Get current user
dx dhis2 me

# Get system info
dx dhis2 system

# Get system version
dx dhis2 system --version
```

## Options

| Option | Description |
|--------|-------------|
| `--url` | DHIS2 server URL (env: DHIS2_URL) |
| `--user` | Username (env: DHIS2_USER, default: admin) |
| `--password` | Password (env: DHIS2_PASSWORD, default: district) |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-o, --output` | Output format (text, json, quiet) |

## DHIS2 Resources

- [DHIS2 Documentation](https://docs.dhis2.org/)
- [DHIS2 Web API](https://docs.dhis2.org/en/develop/develop.html)
- [Demo Server](https://play.dhis2.org/)

## See Also

- [http](./http.md) - HTTP client for custom API calls
- [json](./json.md) - JSON processing
