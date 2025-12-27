//! # Config Commands (get/set/list)
//!
//! This example shows how to implement config management commands.
//!
//! Run with: `cargo run --example 0303_config_commands`

#![allow(dead_code)]

use std::collections::BTreeMap;

// =========================================================================
// IN-MEMORY CONFIG STORE
// =========================================================================

#[derive(Debug, Default)]
pub struct ConfigStore {
    values: BTreeMap<String, String>,
}

impl ConfigStore {
    pub fn new() -> Self {
        let mut store = Self::default();
        // Set defaults
        store.set("general.color", "true");
        store.set("general.verbose", "false");
        store.set("output.format", "text");
        store.set("defaults.algorithm", "sha256");
        store
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn unset(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }

    pub fn list(&self) -> impl Iterator<Item = (&String, &String)> {
        self.values.iter()
    }

    pub fn list_prefix(&self, prefix: &str) -> impl Iterator<Item = (&String, &String)> {
        self.values
            .iter()
            .filter(move |(k, _)| k.starts_with(prefix))
    }
}

// =========================================================================
// CONFIG COMMANDS
// =========================================================================

fn cmd_get(store: &ConfigStore, key: &str) {
    match store.get(key) {
        Some(value) => println!("{}", value),
        None => {
            eprintln!("Key not found: {}", key);
            std::process::exit(1);
        }
    }
}

fn cmd_set(store: &mut ConfigStore, key: &str, value: &str) {
    store.set(key, value);
    println!("Set {} = {}", key, value);
}

fn cmd_unset(store: &mut ConfigStore, key: &str) {
    if store.unset(key) {
        println!("Removed {}", key);
    } else {
        eprintln!("Key not found: {}", key);
    }
}

fn cmd_list(store: &ConfigStore, prefix: Option<&str>) {
    let iter: Box<dyn Iterator<Item = _>> = match prefix {
        Some(p) => Box::new(store.list_prefix(p)),
        None => Box::new(store.list()),
    };

    for (key, value) in iter {
        println!("{} = {}", key, value);
    }
}

fn main() {
    println!("=== Config Commands ===\n");

    // =========================================================================
    // DEMO CONFIG OPERATIONS
    // =========================================================================

    let mut store = ConfigStore::new();

    println!("--- Initial Config ---");
    cmd_list(&store, None);

    println!();

    println!("--- Get Value ---");
    print!("  general.color: ");
    cmd_get(&store, "general.color");

    println!();

    println!("--- Set Value ---");
    print!("  ");
    cmd_set(&mut store, "output.format", "json");

    println!();

    println!("--- List by Prefix ---");
    println!("  [general.*]");
    for (k, v) in store.list_prefix("general.") {
        println!("    {} = {}", k, v);
    }

    println!();

    println!("--- Updated Config ---");
    cmd_list(&store, None);

    println!();

    // =========================================================================
    // CLI DESIGN
    // =========================================================================

    println!("--- CLI Design ---");
    println!(
        r#"
Config subcommands:

dx config get <key>         # Get single value
dx config set <key> <value> # Set value
dx config unset <key>       # Remove value
dx config list              # List all
dx config list --prefix general  # Filter by prefix
dx config path              # Show config file path
dx config edit              # Open in $EDITOR
dx config reset             # Reset to defaults

Examples:
  dx config get output.format
  dx config set defaults.algorithm sha512
  dx config list --prefix defaults
"#
    );

    println!();

    // =========================================================================
    // DOT NOTATION
    // =========================================================================

    println!("--- Dot Notation for Nested Keys ---");
    println!(
        r#"
Using dot notation (like git config):

  general.color = true
  general.verbose = false
  output.format = text
  defaults.algorithm = sha256

Maps to TOML:

  [general]
  color = true
  verbose = false

  [output]
  format = "text"

  [defaults]
  algorithm = "sha256"
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Config command features:");
    println!("  1. get/set/unset/list operations");
    println!("  2. Dot notation for nested keys");
    println!("  3. Prefix filtering for list");
    println!("  4. path/edit/reset utilities");
    println!("  5. Persist to TOML file");
}
