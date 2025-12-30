# xml - XML Processing

Format, validate, and convert XML files.

## Subcommands

| Command | Description |
|---------|-------------|
| `format` | Pretty-print XML |
| `validate` | Validate XML syntax |
| `to-json` | Convert XML to JSON |

## Usage

```bash
# Pretty-print XML
dx xml format document.xml
cat document.xml | dx xml format -

# Validate XML syntax
dx xml validate document.xml

# Convert XML to JSON
dx xml to-json document.xml
```

## Examples

### Format XML

```bash
# Pretty-print an XML file
dx xml format config.xml

# Format minified XML
echo '<root><item>test</item></root>' | dx xml format -
# <root>
#   <item>test</item>
# </root>
```

### Validate XML

```bash
# Check if file is valid XML
dx xml validate document.xml
# Output: Valid XML

# Validate with verbose output
dx xml validate broken.xml -v
```

### Convert to JSON

```bash
# Convert XML to JSON
dx xml to-json data.xml

# Pretty-print with jq
dx xml to-json config.xml | jq '.'

# Save to file
dx xml to-json document.xml > document.json
```

## Options

| Option | Description |
|--------|-------------|
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-o, --output` | Output format (text, json, quiet) |

## See Also

- [json](./json.md) - JSON processing
- [yaml](./yaml.md) - YAML processing
