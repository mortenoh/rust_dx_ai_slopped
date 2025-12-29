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

Generate random test data with 100+ generator types and output to screen or file.

### Usage

```bash
dx polars random [FILE] [OPTIONS]
```

If no file is specified, data is displayed on screen.

### Options

| Option | Description |
|--------|-------------|
| `-n, --rows <N>` | Number of rows to generate (default: 20) |
| `-c, --columns <COLS>` | Column definitions (see Generator Types below) |
| `-c list` | Show all available generator types |
| `-f, --format <FMT>` | Output format: table, csv, json, jsonl |
| `--categories <N>` | Number of categories for `category` type (default: 10) |
| `--string-len <N>` | Length for `string` columns (default: 10) |
| `--min <N>` | Minimum value for numeric columns (default: 0) |
| `--max <N>` | Maximum value for numeric columns (default: 1000) |
| `--null-prob <P>` | Null probability 0.0-1.0 (default: 0.0) |
| `--seed <N>` | Random seed for reproducibility |

### Column Definition Syntax

Define columns using the format `name:type` or `name:type[params]`:

```bash
# Basic syntax
-c "column_name:generator_type"

# With parameters (separated by semicolons)
-c "column_name:generator_type[param1;param2]"

# Multiple columns
-c "id:id,name:name,score:float[0;100]"
```

### Generator Types

#### Basic Types

| Type | Parameters | Description | Example Values |
|------|------------|-------------|----------------|
| `id` | `[start;step]` | Sequential integers (never null) | 1, 2, 3, ... |
| `int`, `integer`, `i64` | `[min;max]` | Random integers | 42, 789, 156 |
| `float`, `f64`, `double` | `[min;max]` | Random floats | 123.4567, 89.1234 |
| `string`, `str`, `text` | `[len]` | Random alphanumeric strings | "xK9mPq2bNw" |
| `bool`, `boolean` | `[prob]` | Random true/false (prob = true %) | true, false |
| `hex` | `[len]` | Hexadecimal strings | "a4f2c8" |
| `date` | - | Random dates (2020-2025) | 2023-05-15 |

**Examples:**
```bash
dx polars random -c "id:id[100;2]"           # IDs: 100, 102, 104, ...
dx polars random -c "score:int[0;100]"       # Integers 0-100
dx polars random -c "price:float[9.99;99.99]" # Floats in range
dx polars random -c "code:string[6]"          # 6-char strings
dx polars random -c "active:bool[0.8]"        # 80% true
```

#### Personal Data

| Type | Parameters | Description | Example Values |
|------|------------|-------------|----------------|
| `name` | `[locale]` | Full name | "John Smith" |
| `first_name` | `[locale]` | First name | "Emma" |
| `last_name` | `[locale]` | Last name | "Johnson" |
| `email` | - | Email address | "user@example.com" |
| `username` | - | Username | "cool_user42" |
| `phone` | - | Phone number | "(555) 123-4567" |
| `address` | - | Street address | "123 Main St" |
| `password` | `[len]` | Random password | "Kx9#mPq2!" |

**Supported locales:** `en` (default), `de`, `fr`, `es`, `no`, `ja`, `zh`, `pt`, `it`, `nl`, `sv`

**Examples:**
```bash
dx polars random -c "customer:name"           # English names
dx polars random -c "kunde:name[de]"          # German names
dx polars random -c "client:name[fr]"         # French names
dx polars random -c "cliente:name[es]"        # Spanish names
dx polars random -c "kontakt:first_name[no]"  # Norwegian first names
```

#### Network

| Type | Description | Example Values |
|------|-------------|----------------|
| `ipv4` | IPv4 address | "192.168.1.100" |
| `ipv6` | IPv6 address | "2001:db8::1" |
| `domain` | Domain name | "example.com" |
| `url` | Full URL | "https://example.com/page" |
| `mac` | MAC address | "00:1A:2B:3C:4D:5E" |

#### Identifiers

| Type | Description | Example Values |
|------|-------------|----------------|
| `uuid` | UUID v4 | "550e8400-e29b-41d4-a716-446655440000" |
| `ulid` | ULID (sortable) | "01ARZ3NDEKTSV4RRFFQ69G5FAV" |
| `credit_card` | Credit card number | "4532015112830366" |
| `iban` | International Bank Account Number | "DE89370400440532013000" |
| `isbn` | ISBN-13 | "978-3-16-148410-0" |
| `ssn` | US Social Security Number | "123-45-6789" |

#### Color

| Type | Description | Example Values |
|------|-------------|----------------|
| `hex_color` | Hex color code | "#FF5733" |
| `color_name` | Color name | "Cerulean" |
| `rgb` | RGB tuple | "(255, 87, 51)" |
| `hsl` | HSL tuple | "(9, 100%, 60%)" |

#### File/System

| Type | Description | Example Values |
|------|-------------|----------------|
| `mime` | MIME type | "application/pdf" |
| `file_ext` | File extension | ".pdf", ".docx" |
| `file_name` | File name | "report.pdf" |
| `file_path` | Full file path | "/home/user/docs/report.pdf" |
| `semver` | Semantic version | "1.2.3" |
| `user_agent` | Browser user agent | "Mozilla/5.0..." |

#### Commerce

| Type | Description | Example Values |
|------|-------------|----------------|
| `company` | Company name | "Acme Corporation" |
| `product` | Product name | "Ergonomic Steel Chair" |
| `job` | Job title | "Senior Software Engineer" |
| `industry` | Industry name | "Technology" |
| `currency` | Currency code | "USD", "EUR" |
| `price` | Formatted price | "$19.99" |

#### Vehicle

| Type | Description | Example Values |
|------|-------------|----------------|
| `vehicle` | Vehicle make | "Toyota", "Ford" |
| `vehicle_model` | Vehicle model | "Camry", "F-150" |
| `vehicle_full` | Year + Make + Model | "2023 Toyota Camry" |
| `vin` | Vehicle Identification Number | "1HGBH41JXMN109186" |
| `license_plate` | License plate | "ABC-1234" |

#### Finance/Crypto

| Type | Description | Example Values |
|------|-------------|----------------|
| `btc` | Bitcoin address | "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" |
| `eth` | Ethereum address | "0x742d35Cc6634C0532925a3b844Bc9e7595f..." |
| `swift` | SWIFT/BIC code | "CHASUS33" |
| `routing` | US routing number | "021000021" |
| `account` | Bank account number | "1234567890" |

#### Science

| Type | Description | Example Values |
|------|-------------|----------------|
| `element` | Chemical element name | "Hydrogen", "Carbon" |
| `element_symbol` | Element symbol | "H", "C", "Fe" |
| `unit` | SI unit | "kilogram", "meter" |

#### Temporal

| Type | Description | Example Values |
|------|-------------|----------------|
| `timestamp` | Unix timestamp (seconds) | 1703980800 |
| `timestamp_ms` | Unix timestamp (milliseconds) | 1703980800000 |

#### Geographic

| Type | Parameters | Description | Example Values |
|------|------------|-------------|----------------|
| `lat` | `[min;max]` | Latitude | 40.7128 |
| `lon` | `[min;max]` | Longitude | -74.0060 |
| `coords` | `[minlat;maxlat;minlon;maxlon]` | Lat,Lon pair | "40.7,-74.0" |
| `point` | `[minlat;maxlat;minlon;maxlon]` | GeoJSON Point | `{"type":"Point",...}` |

**Examples:**
```bash
# New York City bounding box
dx polars random -c "location:coords[40.4;41.0;-74.3;-73.7]"

# GeoJSON points for mapping
dx polars random -c "geometry:point[40.4;41.0;-74.3;-73.7]" -f json
```

#### Categories (Pre-defined Value Sets)

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

#### Entertainment

| Type | Description | Example Values |
|------|-------------|----------------|
| `book_title` | Book title | "The Great Adventure" |
| `book_author` | Author name | "Jane Austen" |
| `book_genre` | Book genre | "Mystery" |
| `book_publisher` | Publisher name | "Penguin Books" |
| `movie_title` | Movie title | "The Last Journey" |
| `movie_director` | Director name | "Steven Spielberg" |
| `movie_genre` | Movie genre | "Action" |
| `music_artist` | Music artist | "The Beatles" |
| `music_album` | Album name | "Greatest Hits" |
| `music_song` | Song title | "Summer Nights" |
| `music_genre` | Music genre | "Rock" |
| `instrument` | Musical instrument | "Guitar" |
| `tv_show` | TV show name | "Breaking News" |
| `game_title` | Video game title | "Epic Quest" |
| `game_platform` | Gaming platform | "PlayStation" |
| `game_genre` | Game genre | "RPG" |

#### Food

| Type | Description | Example Values |
|------|-------------|----------------|
| `dish` | Dish name | "Pasta Carbonara" |
| `cuisine` | Cuisine type | "Italian" |
| `ingredient` | Cooking ingredient | "Olive Oil" |
| `spice` | Spice name | "Paprika" |
| `vegetable` | Vegetable name | "Broccoli" |
| `beverage` | Beverage name | "Orange Juice" |
| `coffee` | Coffee drink | "Cappuccino" |
| `beer` | Beer style | "IPA" |
| `wine` | Wine variety | "Chardonnay" |
| `restaurant` | Restaurant name | "The Golden Fork" |

#### Animals

| Type | Description | Example Values |
|------|-------------|----------------|
| `animal` | Animal name | "Lion" |
| `dog_breed` | Dog breed | "Golden Retriever" |
| `cat_breed` | Cat breed | "Siamese" |
| `bird` | Bird species | "Eagle" |
| `fish` | Fish species | "Salmon" |
| `pet_name` | Pet name | "Buddy" |

#### Travel

| Type | Description | Example Values |
|------|-------------|----------------|
| `airline` | Airline name | "United Airlines" |
| `flight` | Flight number | "UA1234" |
| `airport` | Airport code | "JFK" |
| `airport_name` | Airport full name | "John F. Kennedy International" |
| `aircraft` | Aircraft type | "Boeing 737" |
| `seat` | Seat number | "12A" |
| `hotel` | Hotel chain | "Marriott" |
| `room_type` | Room type | "Suite" |
| `landmark` | Landmark name | "Eiffel Tower" |
| `destination` | Travel destination | "Paris, France" |

#### Healthcare

| Type | Description | Example Values |
|------|-------------|----------------|
| `condition` | Medical condition | "Hypertension" |
| `medication` | Medication name | "Aspirin" |
| `blood_type` | Blood type | "O+" |
| `hospital` | Hospital name | "General Hospital" |
| `specialty` | Medical specialty | "Cardiology" |

#### Sports

| Type | Description | Example Values |
|------|-------------|----------------|
| `sport` | Sport name | "Basketball" |
| `team` | Team name | "Lakers" |
| `league` | Sports league | "NBA" |
| `position` | Player position | "Point Guard" |
| `tournament` | Tournament name | "World Cup" |

#### Hacker/Developer

| Type | Description | Example Values |
|------|-------------|----------------|
| `hacker` | Hacker phrase | "Compiling quantum flux..." |
| `programming_language` | Programming language | "Rust" |
| `framework` | Software framework | "React" |
| `database` | Database name | "PostgreSQL" |
| `cloud_provider` | Cloud provider | "AWS" |
| `git_branch` | Git branch name | "feature/login" |
| `git_commit` | Git commit message | "Fix null pointer exception" |
| `git_sha` | Git SHA | "a1b2c3d4" |

#### Education

| Type | Description | Example Values |
|------|-------------|----------------|
| `university` | University name | "MIT" |
| `degree` | Degree type | "Bachelor of Science" |
| `major` | Field of study | "Computer Science" |
| `course` | Course name | "Data Structures" |
| `gpa` | GPA score | 3.85 |

#### Government

| Type | Description | Example Values |
|------|-------------|----------------|
| `us_agency` | US government agency | "FBI" |
| `passport` | Passport number | "AB1234567" |
| `drivers_license` | Driver's license | "D1234567890123" |
| `tax_id` | Tax ID number | "12-3456789" |

#### Weather

| Type | Description | Example Values |
|------|-------------|----------------|
| `weather` | Weather condition | "Sunny" |
| `temperature` | Temperature (Celsius) | 22 |
| `season` | Season name | "Summer" |
| `forecast` | Weather forecast | "Partly cloudy with chance of rain" |

#### Astrology

| Type | Description | Example Values |
|------|-------------|----------------|
| `zodiac` | Zodiac sign | "Aries" |
| `chinese_zodiac` | Chinese zodiac | "Dragon" |
| `birthstone` | Birthstone | "Ruby" |
| `horoscope` | Horoscope text | "Today brings new opportunities..." |

### Output Formats

| Format | Flag | Description |
|--------|------|-------------|
| Table | `-f table` (default) | Pretty-printed table |
| CSV | `-f csv` | Comma-separated values |
| JSON | `-f json` | JSON array of objects |
| JSONL | `-f jsonl` | Newline-delimited JSON |

**Examples:**
```bash
dx polars random -n 5 -f table   # Pretty table (default)
dx polars random -n 5 -f csv     # CSV output
dx polars random -n 5 -f json    # JSON array
dx polars random -n 5 -f jsonl   # One JSON object per line
```

### Examples by Use Case

#### E-commerce Test Data
```bash
dx polars random orders.parquet -n 100000 \
  -c "order_id:id,customer:name,email:email,city:city,product:product,quantity:int[1;10],price:float[9.99;999.99],status:status"
```

#### Employee Directory
```bash
dx polars random employees.csv -n 500 \
  -c "emp_id:id,name:name,email:email,department:department,title:job,hire_date:date,active:bool"
```

#### IoT Sensor Readings
```bash
dx polars random sensors.parquet -n 1000000 \
  -c "ts:timestamp_ms,sensor_id:id,lat:lat,lon:lon,temp:float[-20;50],humidity:float[0;100],status:status"
```

#### Financial Transactions
```bash
dx polars random transactions.parquet -n 50000 \
  -c "tx_id:ulid,from_account:account,to_account:account,amount:float[0.01;10000],currency:currency,timestamp:timestamp"
```

#### Multi-locale Customer Data
```bash
# German customers
dx polars random de_customers.csv -n 1000 \
  -c "id:id,name:name[de],email:email,phone:phone"

# French customers
dx polars random fr_customers.csv -n 1000 \
  -c "id:id,name:name[fr],email:email,phone:phone"

# Spanish customers
dx polars random es_customers.csv -n 1000 \
  -c "id:id,name:name[es],email:email,phone:phone"
```

#### Vehicle Fleet
```bash
dx polars random fleet.parquet -n 5000 \
  -c "id:id,vehicle:vehicle_full,vin:vin,plate:license_plate,fuel:category,status:status"
```

#### Color Palette Generator
```bash
dx polars random colors.json -n 20 -f json \
  -c "name:color_name,hex:hex_color,rgb:rgb"
```

#### API Mock Data with GeoJSON
```bash
dx polars random locations.json -n 100 -f json \
  -c "id:ulid,name:company,geometry:point[40.4;41.0;-74.3;-73.7]"
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
# Generate reproducible test data
dx polars random test.parquet -n 10000 --seed 42 \
  -c "id:id,name:name,email:email,score:float[0;100]"

# Add null values for testing null handling
dx polars random nulls_test.parquet -n 1000 \
  -c "id:id,value:float" --null-prob 0.1
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
# Generate and pipe to jq for filtering
dx polars random -n 100 -c "id:id,status:status" -f json | jq '.[] | select(.status == "active")'

# Generate CSV and import to database
dx polars random -n 10000 -c "id:id,name:name,email:email" -f csv | psql -c "COPY users FROM STDIN CSV"
```

---

## See Also

- [csv](./csv.md) - CSV formatting and conversion
- [json](./json.md) - JSON processing
- [dx-datagen Library](../appendices/l-dx-datagen.md) - Full data generation library documentation
- [Polars documentation](https://pola.rs/) - Full Polars library documentation
