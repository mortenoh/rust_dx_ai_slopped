# dx-datagen Library

Fast, comprehensive fake data generation library for Rust. Powers the `dx polars random` command.

## Overview

dx-datagen provides 150+ generators across 20+ categories for generating realistic test data. All generators are:

- **Reproducible** with seeding support
- **Trait object compatible** for dynamic dispatch
- **Feature-gated** for optional dependencies

## Installation

```toml
[dependencies]
dx-datagen = { path = "crates/datagen" }

# With optional features
dx-datagen = { path = "crates/datagen", features = ["chrono", "geojson"] }
```

## Quick Start

```rust
use dx_datagen::{personal, network, numeric, commerce};
use rand::SeedableRng;
use rand::rngs::StdRng;

let mut rng = StdRng::seed_from_u64(42);

// Personal data
let name = personal::full_name(&mut rng);
let email = personal::email(&mut rng);
let phone = personal::phone(&mut rng);

// Network data
let ip = network::ipv4(&mut rng);
let domain = network::domain(&mut rng);

// Commerce data
let company = commerce::company::company_name(&mut rng);
let product = commerce::product::product_name(&mut rng);
```

## Generator Categories

### Personal (`personal`)

```rust
use dx_datagen::personal;

personal::first_name(&mut rng);      // "Emma"
personal::last_name(&mut rng);       // "Johnson"
personal::full_name(&mut rng);       // "Emma Johnson"
personal::email(&mut rng);           // "emma.johnson@example.com"
personal::username(&mut rng);        // "cool_user42"
personal::phone(&mut rng);           // "(555) 123-4567"
personal::password(&mut rng, 12);    // "Kx9#mPq2!aB3"
```

### Address (`address`)

```rust
use dx_datagen::address;

address::street_address(&mut rng);   // "123 Main Street"
address::city(&mut rng);             // "New York"
address::country(&mut rng);          // "United States"
address::zip_code(&mut rng);         // "10001"
```

### Network (`network`)

```rust
use dx_datagen::network;

network::ipv4(&mut rng);             // "192.168.1.100"
network::ipv6(&mut rng);             // "2001:db8::1"
network::mac_address(&mut rng);      // "00:1A:2B:3C:4D:5E"
network::domain(&mut rng);           // "example.com"
network::url(&mut rng);              // "https://example.com/page"
network::user_agent(&mut rng);       // "Mozilla/5.0..."
```

### Numeric (`numeric`)

```rust
use dx_datagen::numeric;

numeric::credit_card(&mut rng);      // "4532015112830366"
numeric::credit_card_cvv(&mut rng);  // "123"
numeric::iban(&mut rng);             // "DE89370400440532013000"
numeric::isbn13(&mut rng);           // "978-3-16-148410-0"
numeric::ssn(&mut rng);              // "123-45-6789"
```

### Finance (`numeric::finance`)

```rust
use dx_datagen::numeric::finance;

finance::bitcoin_address(&mut rng);  // "1A1zP1eP5..."
finance::ethereum_address(&mut rng); // "0x742d35Cc..."
finance::swift_code(&mut rng);       // "CHASUS33"
finance::routing_number(&mut rng);   // "021000021"
finance::account_number(&mut rng);   // "1234567890"
```

### Commerce (`commerce`)

```rust
use dx_datagen::commerce::{company, product, job, currency};

// Company
company::company_name(&mut rng);     // "Acme Corporation"
company::company_suffix(&mut rng);   // "Inc"
company::industry(&mut rng);         // "Technology"
company::catch_phrase(&mut rng);     // "Innovative solutions"

// Product
product::product_name(&mut rng);     // "Ergonomic Steel Chair"
product::product_adjective(&mut rng);// "Handcrafted"
product::product_material(&mut rng); // "Steel"
product::product_category(&mut rng); // "Electronics"
product::price(&mut rng, 10.0, 100.0); // 42.99

// Job
job::job_title(&mut rng);            // "Senior Software Engineer"
job::department(&mut rng);           // "Engineering"
job::job_type(&mut rng);             // "Full-time"

// Currency
currency::currency_code(&mut rng);   // "USD"
currency::currency_name(&mut rng);   // "US Dollar"
currency::currency_symbol(&mut rng); // "$"
```

### Vehicle (`vehicle`)

```rust
use dx_datagen::vehicle;

vehicle::vehicle_make(&mut rng);     // "Toyota"
vehicle::vehicle_model(&mut rng);    // "Camry"
vehicle::vehicle_type(&mut rng);     // "Sedan"
vehicle::fuel_type(&mut rng);        // "Gasoline"
vehicle::vehicle_year(&mut rng);     // 2023
vehicle::vehicle_full(&mut rng);     // "2023 Toyota Camry"
vehicle::vin(&mut rng);              // "1HGBH41JXMN109186"
vehicle::license_plate(&mut rng);    // "ABC-1234"
```

### Color (`color`)

```rust
use dx_datagen::color;

color::hex_color(&mut rng);          // "#FF5733"
color::hex_color_alpha(&mut rng);    // "#FF5733CC"
color::rgb(&mut rng);                // (255, 87, 51)
color::rgba(&mut rng);               // (255, 87, 51, 0.8)
color::hsl(&mut rng);                // (9, 100, 60)
color::color_name(&mut rng);         // "Cerulean"
color::css_rgb(&mut rng);            // "rgb(255, 87, 51)"
```

### File (`file`)

```rust
use dx_datagen::file;

file::file_extension(&mut rng);      // "pdf"
file::mime_type(&mut rng);           // "application/pdf"
file::file_name(&mut rng);           // "report.pdf"
file::file_path(&mut rng);           // "/home/user/docs/report.pdf"
file::directory_path(&mut rng);      // "/home/user/documents"
file::semver(&mut rng);              // "1.2.3"
file::user_agent(&mut rng);          // "Mozilla/5.0..."
```

### Science (`science`)

```rust
use dx_datagen::science;

science::chemical_element(&mut rng); // "Hydrogen"
science::chemical_symbol(&mut rng);  // "H"
science::element_full(&mut rng);     // ("Hydrogen", "H", 1)
science::unit(&mut rng);             // "kilogram"
science::unit_symbol(&mut rng);      // "kg"
science::derived_unit(&mut rng);     // "newton"
science::scientific_notation(&mut rng); // "3.142e+2"
```

### UUID (`uuid`)

```rust
use dx_datagen::uuid;

uuid::v4();                          // Random UUID
uuid::v7();                          // Time-based UUID (sortable)
uuid::ulid();                        // ULID
uuid::ulid_from_timestamp(1703980800000); // ULID with timestamp
uuid::ulid_with_rng(&mut rng);       // Deterministic ULID
```

### Temporal (`temporal`) [feature: `chrono`]

```rust
use dx_datagen::temporal;

temporal::date(&mut rng);            // NaiveDate
temporal::time(&mut rng);            // NaiveTime
temporal::datetime(&mut rng);        // NaiveDateTime

// Timestamps (no chrono required)
temporal::timestamp_range(&mut rng, min, max);
temporal::timestamp_recent(&mut rng, days);
temporal::timestamp_future(&mut rng, days);
temporal::timestamp_recent_ms(&mut rng, days);
```

### Geo (`geo`) [feature: `geojson`]

```rust
use dx_datagen::geo;

geo::latitude(&mut rng);             // 40.7128
geo::longitude(&mut rng);            // -74.0060
geo::coordinate(&mut rng);           // (40.7128, -74.0060)

// With bounds
geo::latitude_in_range(&mut rng, 40.0, 41.0);
geo::longitude_in_range(&mut rng, -75.0, -73.0);
geo::coordinate_in_bounds(&mut rng, 40.0, 41.0, -75.0, -73.0);

// GeoJSON
geo::geojson_point(&mut rng);        // GeoJSON Point object
geo::geojson_point_in_bounds(&mut rng, 40.0, 41.0, -75.0, -73.0);
```

### Text (`text`)

```rust
use dx_datagen::text;

text::word(&mut rng);                // Random word
text::sentence(&mut rng);            // Random sentence
text::paragraph(&mut rng);           // Random paragraph
text::lorem_words(&mut rng, 5);      // 5 lorem ipsum words
text::lorem_sentences(&mut rng, 3);  // 3 lorem ipsum sentences
```

### Categories (`categories`)

```rust
use dx_datagen::categories;

categories::fruit(&mut rng);         // "apple"
categories::vegetable(&mut rng);     // "carrot"
categories::animal(&mut rng);        // "dog"
categories::planet(&mut rng);        // "Mars"
```

### Entertainment (`entertainment`)

```rust
use dx_datagen::entertainment;

// Books
entertainment::book_title(&mut rng);     // "The Great Adventure"
entertainment::book_author(&mut rng);    // "Jane Austen"
entertainment::book_genre(&mut rng);     // "Mystery"
entertainment::book_publisher(&mut rng); // "Penguin Books"

// Movies
entertainment::movie_title(&mut rng);    // "The Last Journey"
entertainment::movie_director(&mut rng); // "Steven Spielberg"
entertainment::movie_genre(&mut rng);    // "Action"

// Music
entertainment::music_artist(&mut rng);   // "The Beatles"
entertainment::music_album(&mut rng);    // "Greatest Hits"
entertainment::music_song(&mut rng);     // "Summer Nights"
entertainment::music_genre(&mut rng);    // "Rock"
entertainment::music_instrument(&mut rng); // "Guitar"

// TV & Games
entertainment::tv_show(&mut rng);        // "Breaking News"
entertainment::game_title(&mut rng);     // "Epic Quest"
entertainment::game_platform(&mut rng);  // "PlayStation"
entertainment::game_genre(&mut rng);     // "RPG"
```

### Food (`food`)

```rust
use dx_datagen::food;

food::dish(&mut rng);                // "Pasta Carbonara"
food::cuisine(&mut rng);             // "Italian"
food::ingredient(&mut rng);          // "Olive Oil"
food::spice(&mut rng);               // "Paprika"
food::vegetable(&mut rng);           // "Broccoli"
food::beverage(&mut rng);            // "Orange Juice"
food::coffee(&mut rng);              // "Cappuccino"
food::beer(&mut rng);                // "IPA"
food::wine(&mut rng);                // "Chardonnay"
food::restaurant(&mut rng);          // "The Golden Fork"
```

### Animals (`animals`)

```rust
use dx_datagen::animals;

animals::animal(&mut rng);           // "Lion"
animals::dog_breed(&mut rng);        // "Golden Retriever"
animals::cat_breed(&mut rng);        // "Siamese"
animals::bird(&mut rng);             // "Eagle"
animals::fish(&mut rng);             // "Salmon"
animals::pet_name(&mut rng);         // "Buddy"
```

### Travel (`travel`)

```rust
use dx_datagen::travel;

travel::airline(&mut rng);           // "United Airlines"
travel::flight_number(&mut rng);     // "UA1234"
travel::airport_code(&mut rng);      // "JFK"
travel::airport_name(&mut rng);      // "John F. Kennedy International"
travel::aircraft_type(&mut rng);     // "Boeing 737"
travel::seat(&mut rng);              // "12A"
travel::hotel_chain(&mut rng);       // "Marriott"
travel::room_type(&mut rng);         // "Suite"
travel::landmark(&mut rng);          // "Eiffel Tower"
travel::destination(&mut rng);       // "Paris, France"
```

### Healthcare (`healthcare`)

```rust
use dx_datagen::healthcare;

healthcare::condition(&mut rng);     // "Hypertension"
healthcare::medication(&mut rng);    // "Aspirin"
healthcare::blood_type(&mut rng);    // "O+"
healthcare::hospital(&mut rng);      // "General Hospital"
healthcare::specialty(&mut rng);     // "Cardiology"
```

### Sports (`sports`)

```rust
use dx_datagen::sports;

sports::sport(&mut rng);             // "Basketball"
sports::team(&mut rng);              // "Lakers"
sports::league(&mut rng);            // "NBA"
sports::position(&mut rng);          // "Point Guard"
sports::tournament(&mut rng);        // "World Cup"
```

### Hacker (`hacker`)

```rust
use dx_datagen::hacker;

hacker::hacker_phrase(&mut rng);     // "Compiling quantum flux..."
hacker::programming_language(&mut rng); // "Rust"
hacker::framework(&mut rng);         // "React"
hacker::database(&mut rng);          // "PostgreSQL"
hacker::cloud_provider(&mut rng);    // "AWS"
hacker::git_branch(&mut rng);        // "feature/login"
hacker::git_commit_message(&mut rng); // "Fix null pointer exception"
hacker::git_sha(&mut rng);           // "a1b2c3d4e5f6"
```

### Education (`education`)

```rust
use dx_datagen::education;

education::university(&mut rng);     // "MIT"
education::degree(&mut rng);         // "Bachelor of Science"
education::major(&mut rng);          // "Computer Science"
education::course_name(&mut rng);    // "Data Structures"
education::gpa(&mut rng);            // 3.85
```

### Government (`government`)

```rust
use dx_datagen::government;

government::us_agency(&mut rng);     // "FBI"
government::passport_number(&mut rng); // "AB1234567"
government::drivers_license(&mut rng); // "D1234567890123"
government::tax_id(&mut rng);        // "12-3456789"
```

### Weather (`weather`)

```rust
use dx_datagen::weather;

weather::condition(&mut rng);        // "Sunny"
weather::temperature_celsius(&mut rng); // 22
weather::season(&mut rng);           // "Summer"
weather::forecast_summary(&mut rng); // "Partly cloudy with chance of rain"
```

### Astrology (`astrology`)

```rust
use dx_datagen::astrology;

astrology::zodiac_sign(&mut rng);    // "Aries"
astrology::chinese_zodiac(&mut rng); // "Dragon"
astrology::birthstone(&mut rng);     // "Ruby"
astrology::horoscope(&mut rng);      // "Today brings new opportunities..."
```

## Locale Support

Generate locale-specific data for names, addresses, and phone numbers:

```rust
use dx_datagen::locale::{Locale, LocaleData};

let mut rng = StdRng::seed_from_u64(42);

// Use locale enum
let locale = Locale::DeDe;  // German
let name = locale.first_name(&mut rng);
let city = locale.city(&mut rng);
let phone = locale.phone(&mut rng);

// Or use locale modules directly
use dx_datagen::locale::{de_de, fr_fr, es_es};

de_de::first_name(&mut rng);         // German first name
de_de::city(&mut rng);               // German city
de_de::postal_code(&mut rng);        // German postal code

fr_fr::first_name(&mut rng);         // French first name
fr_fr::siret(&mut rng);              // French business ID

es_es::first_name(&mut rng);         // Spanish first name
es_es::dni(&mut rng);                // Spanish national ID
```

### Available Locales

| Code | Locale | Special Features |
|------|--------|------------------|
| `en_US` | English (US) | Default, states, zip+4 |
| `no_NO` | Norwegian | Counties, org numbers |
| `de_DE` | German | Bundesländer (states) |
| `fr_FR` | French | Regions, SIRET/SIREN |
| `es_ES` | Spanish | Communities, DNI/NIE |
| `ja_JP` | Japanese | Prefectures, Japanese names |
| `zh_CN` | Chinese (Simplified) | Provinces, Chinese names |
| `pt_BR` | Portuguese (Brazil) | States, Brazilian names |
| `it_IT` | Italian | Regions, Italian names |
| `nl_NL` | Dutch | Provinces, Dutch names |
| `sv_SE` | Swedish | Counties, Swedish names |

## Schema Module [feature: `schema`]

The schema module provides schema-based data generation from various formats.

### JSON Schema → Data Generation

```rust
use dx_datagen::schema::{from_json_schema, from_json_schema_with_options, JsonSchemaOptions};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde_json::json;

let mut rng = StdRng::seed_from_u64(42);

let schema = json!({
    "type": "object",
    "properties": {
        "name": { "type": "string", "minLength": 5 },
        "age": { "type": "integer", "minimum": 0, "maximum": 120 },
        "email": { "type": "string", "format": "email" },
        "active": { "type": "boolean" }
    },
    "required": ["name", "age"]
});

let data = from_json_schema(&mut rng, &schema);
// {"name": "xK9mPq2aB3", "age": 42, "email": "user@example.com", "active": true}

// With options
let options = JsonSchemaOptions {
    default_string_length: 10,
    default_max_items: 5,
    include_optional: true,
    ..Default::default()
};
let data = from_json_schema_with_options(&mut rng, &schema, &options);
```

Supported JSON Schema features:
- Types: `string`, `integer`, `number`, `boolean`, `null`, `array`, `object`
- Formats: `email`, `uuid`, `uri`, `date`, `time`, `date-time`, `ipv4`, `ipv6`, `hostname`
- Constraints: `minimum`, `maximum`, `minLength`, `maxLength`, `minItems`, `maxItems`, `pattern`
- Composition: `enum`, `const`, `oneOf`, `anyOf`, `allOf`
- References: `$ref` to definitions

### Data → JSON Schema Inference

```rust
use dx_datagen::schema::infer_schema;
use serde_json::json;

let records = vec![
    json!({"name": "Alice", "age": 30, "email": "alice@example.com"}),
    json!({"name": "Bob", "age": 25, "email": "bob@example.com"}),
];

let schema = infer_schema(&records);
// {
//   "type": "object",
//   "properties": {
//     "name": { "type": "string" },
//     "age": { "type": "integer", "minimum": 25, "maximum": 30 },
//     "email": { "type": "string", "format": "email" }
//   },
//   "required": ["name", "age", "email"]
// }
```

Format detection: email, UUID, date, time, datetime, URI, IPv4, IPv6.

### SQL DDL/DML Generation

```rust
use dx_datagen::schema::{to_sql_ddl, to_sql_insert, to_sql_insert_batch, SqlDialect};
use serde_json::json;

let data = json!({
    "id": 1,
    "name": "Alice",
    "balance": 100.50,
    "active": true
});

// CREATE TABLE statement
let ddl = to_sql_ddl(&data, "users", SqlDialect::PostgreSQL);
// CREATE TABLE users (
//     id BIGINT,
//     name TEXT,
//     balance DOUBLE PRECISION,
//     active BOOLEAN
// );

// INSERT statement
let insert = to_sql_insert(&data, "users", SqlDialect::PostgreSQL);
// INSERT INTO users (id, name, balance, active) VALUES (1, 'Alice', 100.5, TRUE);

// Batch insert
let records = vec![data, json!({"id": 2, "name": "Bob", "balance": 200.0, "active": false})];
let batch = to_sql_insert_batch(&records, "users", SqlDialect::MySQL);
```

Supported dialects: `PostgreSQL`, `MySQL`, `SQLite`, `SqlServer`.

### OpenAPI Mock Data

```rust
use dx_datagen::schema::{from_openapi, OpenApiSpec};
use rand::SeedableRng;
use rand::rngs::StdRng;

let mut rng = StdRng::seed_from_u64(42);

// Parse OpenAPI spec (from JSON string)
let spec: OpenApiSpec = serde_json::from_str(openapi_json)?;

// Generate mock response for GET /users with 200 status
let response = from_openapi(&mut rng, &spec, "/users", "get", "200");
```

Features:
- Path and method lookup
- `$ref` reference resolution
- Content-type handling (application/json)
- Response schema generation

### Avro Schema Support

```rust
use dx_datagen::schema::from_avro_schema;
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde_json::json;

let mut rng = StdRng::seed_from_u64(42);

let avro_schema = json!({
    "type": "record",
    "name": "User",
    "fields": [
        { "name": "id", "type": "long" },
        { "name": "name", "type": "string" },
        { "name": "active", "type": "boolean" },
        { "name": "tags", "type": { "type": "array", "items": "string" } }
    ]
});

let data = from_avro_schema(&mut rng, &avro_schema);
// {"id": 123456789, "name": "xK9mPq2aB3", "active": true, "tags": ["abc", "def"]}
```

Supported Avro types:
- Primitives: `null`, `boolean`, `int`, `long`, `float`, `double`, `bytes`, `string`
- Complex: `record`, `enum`, `array`, `map`, `union`, `fixed`
- Logical types: `date`, `time-millis`, `timestamp-millis`, `uuid`, `decimal`

### GraphQL Mock Data

```rust
use dx_datagen::schema::{from_graphql_schema, from_graphql_query};
use rand::SeedableRng;
use rand::rngs::StdRng;

let mut rng = StdRng::seed_from_u64(42);

let schema_sdl = r#"
    type User {
        id: ID!
        name: String!
        email: String
        posts: [Post!]!
    }

    type Post {
        id: ID!
        title: String!
        published: Boolean!
    }

    type Query {
        user(id: ID!): User
        users: [User!]!
    }
"#;

// Generate data for a type
let user = from_graphql_schema(&mut rng, schema_sdl, "User")?;
// {"id": "550e8400-...", "name": "xK9mPq2aB3", "email": "user@example.com", "posts": [...]}

// Execute a query
let query = "{ user { id name posts { title } } }";
let result = from_graphql_query(&mut rng, schema_sdl, query)?;
// {"data": {"user": {"id": "...", "name": "...", "posts": [{"title": "..."}]}}}
```

Supported GraphQL features:
- Types: `ID`, `String`, `Int`, `Float`, `Boolean`
- Custom scalars: `DateTime`, `Date`, `Time`, `JSON`, `UUID`, `Email`, `URL`, `IP`
- Lists: `[Type]`, `[Type!]`, `[Type]!`, `[Type!]!`
- Non-null: `Type!`
- Enums, nested types, type references

## Expression DSL Module

The expression module provides a faker-style expression DSL for generating data.

```rust
use dx_datagen::expression::evaluate;
use rand::SeedableRng;
use rand::rngs::StdRng;

let mut rng = StdRng::seed_from_u64(42);

// Provider calls (faker-style)
let name = evaluate(&mut rng, "#{Name.firstName}")?;           // "Emma"
let email = evaluate(&mut rng, "#{Internet.email}")?;          // "emma@example.com"
let company = evaluate(&mut rng, "#{Company.name}")?;          // "Acme Corp"

// Mixed with literals
let greeting = evaluate(&mut rng, "Hello, #{Name.firstName}!")?;

// Regex generation
let code = evaluate(&mut rng, "#{regexify '[A-Z]{3}-[0-9]{4}'}")?;  // "ABC-1234"

// Numeric ranges
let age = evaluate(&mut rng, "#{Number.between 18, 65}")?;     // "42"
let price = evaluate(&mut rng, "#{Number.decimal 0.0, 100.0}")?; // "42.50"

// Random choice
let color = evaluate(&mut rng, "#{options.option 'red', 'green', 'blue'}")?;

// Weighted choice
let status = evaluate(&mut rng, "#{options.weighted 'active', 80, 'inactive', 20}")?;

// Template patterns
let phone = evaluate(&mut rng, "#{numerify '###-###-####'}")?; // "555-123-4567"
let id = evaluate(&mut rng, "#{letterify '???-###'}")?;        // "ABC-123"
```

Available providers: `Name`, `Internet`, `Address`, `Company`, `Commerce`, `Vehicle`, `Science`, `Color`, `File`, `Lorem`, `Number`, `options`.

## Feature Flags

| Feature | Description |
|---------|-------------|
| `temporal` | Enable temporal generators with chrono types |
| `geo` | Enable GeoJSON point generation |
| `schema` | Enable schema-based generation (JSON Schema, SQL, OpenAPI, Avro, GraphQL) |
| `full` | Enable all features |

```toml
[dependencies]
# Basic usage
dx-datagen = { path = "crates/datagen" }

# With specific features
dx-datagen = { path = "crates/datagen", features = ["temporal", "geo"] }

# With schema support
dx-datagen = { path = "crates/datagen", features = ["schema"] }

# All features
dx-datagen = { path = "crates/datagen", features = ["full"] }
```

## Deterministic Generation

All generators support seeded RNGs for reproducible output:

```rust
use rand::SeedableRng;
use rand::rngs::StdRng;
use dx_datagen::personal;

let mut rng1 = StdRng::seed_from_u64(42);
let mut rng2 = StdRng::seed_from_u64(42);

// Same seed = same output
assert_eq!(personal::first_name(&mut rng1), personal::first_name(&mut rng2));
```

## Trait Object Support

All generators work with trait objects for dynamic dispatch:

```rust
use rand::RngCore;
use rand::rngs::StdRng;
use rand::SeedableRng;
use dx_datagen::personal;

let mut rng: Box<dyn RngCore> = Box::new(StdRng::seed_from_u64(42));
let name = personal::first_name(&mut *rng);
```

## Generator Pattern

All generators follow a consistent pattern:

```rust
pub fn generator_name<R: ?Sized + Rng>(rng: &mut R) -> ReturnType
```

This allows:
- Generic over any `Rng` implementation
- Trait object support via `?Sized`
- Consistent API across all modules

## CLI Integration

dx-datagen powers the `dx polars random` command:

```bash
# Generate random data with various column types
dx polars random -n 1000 \
  -c "id:id,name:name,email:email,company:company,vin:vin"

# Use locale-specific names
dx polars random -n 100 -c "name:name[de]"

# Generate hex colors and timestamps
dx polars random -n 50 -c "color:hex_color,ts:timestamp"

# Output as JSON
dx polars random -n 10 -c "id:ulid,geo:point" -f json
```

See [polars command reference](../part-3-dx-command-reference/polars.md) for full documentation.

## See Also

- [Polars Command](../part-3-dx-command-reference/polars.md) - CLI random data generation
- [Faker.js](https://fakerjs.dev/) - Similar JavaScript library
- [Python Faker](https://faker.readthedocs.io/) - Similar Python library
