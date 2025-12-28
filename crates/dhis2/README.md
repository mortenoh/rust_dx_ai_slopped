# dx-dhis2

A Rust client library for interacting with DHIS2 (District Health Information System 2) instances.

## Features

- DHIS2 UID generation and validation
- System information fetching
- Organisation unit queries
- Data element queries
- Data set queries
- Optional TUI browser (with `tui` feature)

## Quick Start

```rust
use dx_dhis2::{Dhis2Client, uid};

// Generate DHIS2 UIDs
let uid = uid::generate();
println!("Generated UID: {}", uid);

// Validate a UID
assert!(uid::validate("AbCdEfGhIjK").is_ok());

// Connect to a DHIS2 instance
let client = Dhis2Client::new(
    "https://play.im.dhis2.org/demo",
    "admin",
    "district"
)?;

// Fetch system info
let info = dx_dhis2::info::fetch(&client)?;
println!("DHIS2 version: {:?}", info.version);

// Fetch organisation units
let org_units = dx_dhis2::org_units::fetch(&client, None, 10, false)?;
for ou in &org_units {
    println!("{}: {} (Level {})", ou.id, ou.display_name, ou.level);
}
```

## DHIS2 UID Format

DHIS2 UIDs are 11-character alphanumeric identifiers:
- First character: a-zA-Z (must be a letter)
- Remaining 10: a-zA-Z0-9 (alphanumeric)

Example: `AbCdEfGhIjK`

## TUI Browser

Enable the `tui` feature for an interactive terminal browser:

```toml
[dependencies]
dx-dhis2 = { version = "0.1", features = ["tui"] }
```

```rust
use dx_dhis2::{Dhis2Client, tui};

let client = Dhis2Client::new(
    "https://play.im.dhis2.org/demo",
    "admin",
    "district"
)?;

tui::run(client)?;
```

## Default Demo Server

The library defaults to the DHIS2 demo instance:
- URL: `https://play.im.dhis2.org/stable-2-42-3-1`
- Username: `admin`
- Password: `district`

## License

MIT
