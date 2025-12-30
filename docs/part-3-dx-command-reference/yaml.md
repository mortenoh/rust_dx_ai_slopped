# yaml - YAML Processing

Format, validate, and convert YAML files.

## Subcommands

| Command | Description |
|---------|-------------|
| `format` | Pretty-print YAML |
| `validate` | Validate YAML syntax |
| `to-json` | Convert YAML to JSON |
| `from-json` | Convert JSON to YAML |

## Usage

```bash
# Pretty-print YAML
dx yaml format config.yaml
cat config.yaml | dx yaml format -

# Validate YAML syntax
dx yaml validate config.yaml

# Convert YAML to JSON
dx yaml to-json config.yaml
dx yaml to-json config.yaml > config.json

# Convert JSON to YAML
dx yaml from-json config.json
```

## Examples

### Format YAML

```bash
# Format a configuration file
dx yaml format docker-compose.yaml

# Format from stdin
echo "name: test\nvalue: 123" | dx yaml format -
```

### Validate YAML

```bash
# Check if file is valid YAML
dx yaml validate config.yaml
# Output: Valid YAML

# Check multiple files
for f in *.yaml; do dx yaml validate "$f"; done
```

### Convert Between Formats

```bash
# YAML to JSON
dx yaml to-json kubernetes.yaml > kubernetes.json

# JSON to YAML
dx yaml from-json package.json > package.yaml

# Pipeline conversions
cat data.yaml | dx yaml to-json - | jq '.items[]' | dx yaml from-json -
```

## Options

| Option | Description |
|--------|-------------|
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-o, --output` | Output format (text, json, quiet) |

## See Also

- [json](./json.md) - JSON processing
- [xml](./xml.md) - XML processing
