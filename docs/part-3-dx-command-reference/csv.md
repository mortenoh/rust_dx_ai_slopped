# csv - CSV Processing

Format, query, and convert CSV files.

## Subcommands

| Command | Description |
|---------|-------------|
| `format` | Pretty-print CSV as a table |
| `to-json` | Convert CSV to JSON array |
| `from-json` | Convert JSON array to CSV |
| `query` | Select specific columns from CSV |

## Usage

```bash
# Pretty-print CSV as table
dx csv format data.csv
cat data.csv | dx csv format -

# Convert CSV to JSON
dx csv to-json data.csv

# Convert JSON to CSV
dx csv from-json data.json

# Select specific columns
dx csv query data.csv --columns name,email,age
```

## Examples

### Format as Table

```bash
# Display CSV as formatted table
dx csv format users.csv
# +----+-------+-----+
# | id | name  | age |
# +----+-------+-----+
# | 1  | Alice | 30  |
# | 2  | Bob   | 25  |
# +----+-------+-----+

# From stdin
echo "name,value\ntest,123" | dx csv format -
```

### Convert to JSON

```bash
# Convert to JSON array
dx csv to-json users.csv
# [{"id":"1","name":"Alice","age":"30"},{"id":"2","name":"Bob","age":"25"}]

# Pretty-print JSON output
dx csv to-json users.csv | jq '.'
```

### Convert from JSON

```bash
# JSON array to CSV
echo '[{"name":"Alice","age":30},{"name":"Bob","age":25}]' | dx csv from-json -

# From file
dx csv from-json data.json > data.csv
```

### Query Columns

```bash
# Select specific columns
dx csv query employees.csv --columns name,department,salary

# Reorder columns
dx csv query data.csv --columns col3,col1,col2
```

## Options

| Option | Description |
|--------|-------------|
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-o, --output` | Output format (text, json, quiet) |
| `--columns` | Columns to select (for query) |

## See Also

- [json](./json.md) - JSON processing
- [polars](./polars.md) - DataFrame operations with more features
