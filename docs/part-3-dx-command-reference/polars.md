# polars

DataFrame operations for CSV and Parquet files using the Polars library.

## Usage

```bash
dx polars <COMMAND> [OPTIONS]
```

**Alias:** `dx pl`

## Subcommands

| Command | Description |
|---------|-------------|
| `view` | View data from CSV or Parquet file |
| `random` | Generate random test data |

---

## view

View and inspect data from CSV or Parquet files with various display options.

### Usage

```bash
dx polars view <FILE> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-n, --rows <N>` | Number of rows to display (default: 20) |
| `--tail` | Show last N rows instead of first |
| `-c, --columns <COLS>` | Select specific columns (comma-separated) |
| `--schema` | Show schema only (no data) |
| `--stats` | Show summary statistics |
| `--json` | Output as JSON |

### Examples

```bash
# View first 20 rows (default)
dx polars view data.parquet

# View last 10 rows
dx polars view data.csv -n 10 --tail

# Select specific columns
dx polars view data.parquet -c "id,name,amount"

# Show schema only
dx polars view data.parquet --schema

# Show statistics (count, mean, min, max)
dx polars view data.csv --stats

# Output as JSON (for piping to other tools)
dx polars view data.parquet --json | jq '.data[0]'
```

### Output Formats

**Default table view:**
```
┌data.parquet (1000 rows, 4 cols)────────────────────────┐
│id   store       item         value                     │
│1    Toronto     grape        222                       │
│2    Paris       banana       272                       │
│3    Singapore   mango        146                       │
└────────────────────────────────────────────────────────┘
```

**Schema view (`--schema`):**
```
Schema
══════════════════════════════════════════════════

File: data.parquet
Format: Parquet
Rows: 1,000
Columns: 4
Size: 12.5 KB

Columns:
  id                   Int64           0 nulls (0.0%)
  store                String          0 nulls (0.0%)
  item                 String          0 nulls (0.0%)
  value                Int64           0 nulls (0.0%)
```

**Statistics view (`--stats`):**
```
Statistics
══════════════════════════════════════════════════════════════════════

Column               Type           Nulls         Mean          Min          Max
──────────────────────────────────────────────────────────────────────
id                   Int64              0       500.50         1.00      1000.00
store                String             0            -            -            -
item                 String             0            -            -            -
value                Int64              0       498.23         0.00       999.00
```

---

## random

Generate random test data and output to screen or file (CSV/Parquet).

### Usage

```bash
dx polars random [FILE] [OPTIONS]
```

If no file is specified, data is displayed on screen.

### Options

| Option | Description |
|--------|-------------|
| `-n, --rows <N>` | Number of rows to generate (default: 20) |
| `-c, --columns <COLS>` | Column definitions (see Column Types below) |
| `--categories <N>` | Number of categories for `category` type (default: 10) |
| `--string-len <N>` | Length for `string` columns (default: 10) |
| `--min <N>` | Minimum value for numeric columns (default: 0) |
| `--max <N>` | Maximum value for numeric columns (default: 1000) |
| `--null-prob <P>` | Null probability 0.0-1.0 (default: 0.0) |
| `--seed <N>` | Random seed for reproducibility |

### Column Types

Define columns using the format `name:type`. Available types:

#### Basic Types

| Type | Description | Example Values |
|------|-------------|----------------|
| `id` | Sequential integers 1..n (never null) | 1, 2, 3, ... |
| `int`, `integer`, `i64` | Random integers | 42, 789, 156 |
| `float`, `f64`, `double` | Random floats | 123.4567, 89.1234 |
| `string`, `str`, `text` | Random alphanumeric strings | "xK9mPq2bNw" |
| `bool`, `boolean` | Random true/false | true, false |
| `date` | Random dates (2020-2025) | 2023-05-15 |

#### Category Types

Pre-defined value sets for realistic test data:

| Type | Values |
|------|--------|
| `category`, `cat` | cat_0, cat_1, ..., cat_N (uses `--categories`) |
| `fruit` | apple, banana, orange, grape, mango, strawberry, pineapple, kiwi, peach, cherry |
| `color` | red, blue, green, yellow, purple, orange, pink, brown, black, white |
| `city` | New York, London, Paris, Tokyo, Sydney, Berlin, Rome, Toronto, Dubai, Singapore |
| `country` | USA, UK, France, Germany, Japan, Canada, Australia, Brazil, India, China |
| `status` | pending, active, completed, cancelled, archived |
| `priority` | low, medium, high, critical |
| `department`, `dept` | Engineering, Marketing, Sales, HR, Finance, Support, Operations, Legal |
| `day` | Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday |
| `size` | XS, S, M, L, XL, XXL |

### Examples

```bash
# Generate with defaults (id, store, item, value)
dx polars random

# Output:
# ┌Generated Data (20 rows, 4 cols)──────────────────────┐
# │id   store       item         value                   │
# │1    Toronto     grape        222                     │
# │2    Paris       banana       272                     │
# └──────────────────────────────────────────────────────┘

# Generate to Parquet file
dx polars random data.parquet -n 10000

# Generate to CSV file
dx polars random data.csv -n 5000

# Custom columns with category generators
dx polars random sales.parquet -n 10000 \
  -c "id:id,date:date,product:fruit,store:city,quantity:int,price:float,status:status"

# Specify value ranges
dx polars random prices.parquet -n 1000 \
  -c "id:id,amount:float" --min 10 --max 500

# Add nulls to test null handling
dx polars random test.parquet -n 1000 \
  -c "id:id,name:string,value:float" --null-prob 0.1

# Reproducible data with seed
dx polars random test.parquet -n 100 --seed 42

# All column types demo
dx polars random -n 5 -c "id:id,name:string,score:float,active:bool,created:date,fruit:fruit,city:city,status:status,priority:priority,dept:department"
```

### File Formats

The output format is determined by the file extension:

| Extension | Format |
|-----------|--------|
| `.parquet`, `.pq` | Apache Parquet (columnar, compressed) |
| `.csv` (or any other) | CSV (comma-separated values) |

**Parquet advantages:**
- Smaller file size (compressed)
- Faster read performance
- Preserves column types
- Best for large datasets

**CSV advantages:**
- Human-readable
- Universal compatibility
- Easy to inspect with text editors

---

## Use Cases

### Test Data Generation

```bash
# E-commerce order data
dx polars random orders.parquet -n 100000 \
  -c "order_id:id,customer_city:city,product:fruit,quantity:int,unit_price:float,status:status,priority:priority"

# Employee directory
dx polars random employees.parquet -n 500 \
  -c "emp_id:id,department:department,hire_date:date,active:bool"

# IoT sensor readings
dx polars random sensors.csv -n 1000000 \
  -c "timestamp:date,sensor_id:id,temperature:float,humidity:float,status:status" \
  --min -20 --max 50
```

### Data Inspection

```bash
# Quick peek at a dataset
dx polars view huge_dataset.parquet -n 5

# Check for data quality issues
dx polars view data.csv --stats

# Verify schema matches expectations
dx polars view import.parquet --schema

# Export subset to JSON for API testing
dx polars view data.parquet -n 100 -c "id,name" --json > test_data.json
```

### Pipeline Integration

```bash
# Generate test data and pipe to analysis
dx polars random -n 1000 -c "id:id,value:float" | grep "value"

# View stats as JSON for programmatic use
dx polars view data.parquet --stats --json | jq '.statistics'
```

---

## See Also

- [csv](./csv.md) - CSV formatting and conversion
- [json](./json.md) - JSON processing
- [Polars documentation](https://pola.rs/) - Full Polars library documentation
