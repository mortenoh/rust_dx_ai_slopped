# dx-datagen

Fast, comprehensive fake data generation library for Rust.

## Features

- **100+ generators** across 15+ categories
- **Reproducible output** with seeding support
- **Trait object support** for dynamic dispatch (`Box<dyn RngCore>`)
- **Optional feature flags** for `chrono` and `geojson`
- **Multiple locales** (en_US, no_NO, de_DE, fr_FR, es_ES)
- **Zero external data files** - all data is compiled in

## Quick Start

```rust
use dx_datagen::{personal, network, numeric};
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

// Financial data
let cc = numeric::credit_card(&mut rng);
let iban = numeric::iban(&mut rng);
```

## Categories

### Personal (`personal`)
- `first_name`, `last_name`, `full_name`
- `email`, `username`
- `phone`, `phone_e164`
- `password`, `password_memorable`

### Address (`address`)
- `street_address`, `city`, `state`
- `zip_code`, `country`
- `latitude`, `longitude`

### Network (`network`)
- `ipv4`, `ipv6`, `mac_address`
- `domain`, `subdomain`, `url`
- `user_agent`

### Numeric (`numeric`)
- `credit_card`, `credit_card_cvv`
- `iban`, `isbn10`, `isbn13`
- `ssn` (US Social Security Number)

### Finance (`numeric::finance`)
- `bitcoin_address`, `ethereum_address`
- `routing_number`, `account_number`
- `swift_code`, `bic`
- `transaction_type`, `transaction_description`

### Commerce (`commerce`)
- **Company**: `company_name`, `company_suffix`, `industry`, `catch_phrase`
- **Product**: `product_name`, `product_adjective`, `product_material`, `product_category`, `price`
- **Job**: `job_title`, `job_descriptor`, `job_area`, `job_type`, `department`
- **Currency**: `currency_code`, `currency_name`, `currency_symbol`

### Vehicle (`vehicle`)
- `vehicle_make`, `vehicle_model`, `vehicle_type`
- `fuel_type`, `vehicle_year`
- `vin` (Vehicle Identification Number)
- `license_plate`

### Color (`color`)
- `hex_color`, `hex_color_alpha`
- `rgb`, `rgba`, `hsl`, `hsla`
- `color_name`, `css_color_name`
- `css_rgb`, `css_rgba`, `css_hsl`, `css_hsla`

### File (`file`)
- `file_name`, `file_extension`, `file_path`, `directory_path`
- `mime_type`, `mime_type_by_category`
- `semver`, `semver_with_prerelease`
- `user_agent`

### Science (`science`)
- `chemical_element`, `chemical_symbol`, `element_full`
- `unit`, `unit_symbol`
- `derived_unit`, `derived_unit_symbol`
- `scientific_notation`

### UUID (`uuid`)
- `v4()` - Random UUID
- `v7()` - Time-based UUID (sortable)
- `ulid()` - ULID (Universally Unique Lexicographically Sortable Identifier)
- `ulid_from_timestamp(ts)` - ULID with specific timestamp
- `format(uuid, fmt)` - Format UUID (hyphenated, simple, urn, braced)

### Text (`text`)
- `word`, `sentence`, `paragraph`
- `lorem_words`, `lorem_sentences`, `lorem_paragraphs`
- Pattern-based generation

### Temporal (`temporal`) [feature: `chrono`]
- `date`, `date_between`, `date_recent`, `date_future`
- `time`, `datetime`
- `timestamp_range`, `timestamp_recent`, `timestamp_future`
- `timestamp_recent_ms`, `timestamp_future_ms`

### Geo (`geo`) [feature: `geojson`]
- `latitude`, `longitude`, `coordinate`
- `geojson_point`

### Categories (`categories`)
- `fruit`, `vegetable`, `animal`
- `color`, `day`, `month`
- `planet`, `element`

## Locales

Locale-specific data for names, addresses, phone numbers:

```rust
use dx_datagen::locale::{Locale, LocaleData};
use rand::SeedableRng;
use rand::rngs::StdRng;

let mut rng = StdRng::seed_from_u64(42);

// Use locale enum
let locale = Locale::DeDe;  // German
let name = locale.first_name(&mut rng);
let city = locale.city(&mut rng);

// Or use locale modules directly
use dx_datagen::locale::fr_fr;
let french_name = fr_fr::first_name(&mut rng);
let french_phone = fr_fr::phone(&mut rng);
```

### Available Locales

| Code | Language | Region |
|------|----------|--------|
| `en_US` | English | United States |
| `no_NO` | Norwegian | Norway |
| `de_DE` | German | Germany |
| `fr_FR` | French | France |
| `es_ES` | Spanish | Spain |

### Locale-Specific Features

Each locale provides:
- Male/female first names
- Last names
- Phone number formats (local and E.164)
- Cities and regions/states
- Street addresses and postal codes

Some locales include additional generators:
- **no_NO**: `org_number` (Norwegian organization number)
- **fr_FR**: `siret`, `siren` (French business identifiers)
- **es_ES**: `dni`, `nie` (Spanish ID numbers)

## Feature Flags

```toml
[dependencies]
dx-datagen = { version = "0.1", features = ["chrono", "geojson"] }
```

| Feature | Description |
|---------|-------------|
| `chrono` | Enable temporal generators with chrono types |
| `geojson` | Enable GeoJSON point generation |

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

## Integration with Polars

dx-datagen integrates with the `dx polars random` command:

```bash
# Generate random data with various column types
dx polars random -n 1000 -c "id:id,name:name,email:email,company:company,vin:vin"

# Use locale-specific names
dx polars random -n 100 -c "name:name[de]"

# Generate hex colors and timestamps
dx polars random -n 50 -c "color:hex_color,ts:timestamp"
```

## License

MIT
