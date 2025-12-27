//! # Configuration Profiles
//!
//! This example shows how to implement multiple config profiles.
//!
//! Run with: `cargo run --example 0304_profiles`

#![allow(dead_code)]

use std::collections::HashMap;

// =========================================================================
// PROFILE SYSTEM
// =========================================================================

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub api_url: String,
    pub api_key: Option<String>,
    pub debug: bool,
}

impl Profile {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            api_url: "https://api.example.com".to_string(),
            api_key: None,
            debug: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct ProfileManager {
    profiles: HashMap<String, Profile>,
    active: Option<String>,
}

impl ProfileManager {
    pub fn new() -> Self {
        let mut mgr = Self::default();

        // Add default profiles
        let mut dev = Profile::new("dev");
        dev.api_url = "http://localhost:3000".to_string();
        dev.debug = true;

        let mut staging = Profile::new("staging");
        staging.api_url = "https://staging.example.com".to_string();

        let prod = Profile::new("prod");

        mgr.add(dev);
        mgr.add(staging);
        mgr.add(prod);
        mgr.active = Some("dev".to_string());

        mgr
    }

    pub fn add(&mut self, profile: Profile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn get(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn active(&self) -> Option<&Profile> {
        self.active
            .as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    pub fn set_active(&mut self, name: &str) -> bool {
        if self.profiles.contains_key(name) {
            self.active = Some(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &Profile> {
        self.profiles.values()
    }
}

fn main() {
    println!("=== Configuration Profiles ===\n");

    // =========================================================================
    // PROFILE DEMO
    // =========================================================================

    let mut mgr = ProfileManager::new();

    println!("--- Available Profiles ---");
    for profile in mgr.list() {
        let marker = if Some(&profile.name) == mgr.active.as_ref() {
            "*"
        } else {
            " "
        };
        println!("  {} {} - {}", marker, profile.name, profile.api_url);
    }

    println!();

    println!("--- Active Profile ---");
    if let Some(active) = mgr.active() {
        println!("  Name:    {}", active.name);
        println!("  API URL: {}", active.api_url);
        println!("  Debug:   {}", active.debug);
    }

    println!();

    println!("--- Switch Profile ---");
    mgr.set_active("staging");
    println!("  Switched to: staging");
    if let Some(active) = mgr.active() {
        println!("  API URL: {}", active.api_url);
    }

    println!();

    // =========================================================================
    // CLI DESIGN
    // =========================================================================

    println!("--- CLI Design ---");
    println!(
        r#"
Profile commands:

dx config profile list              # List all profiles
dx config profile show <name>       # Show profile details
dx config profile use <name>        # Switch active profile
dx config profile create <name>     # Create new profile
dx config profile delete <name>     # Delete profile
dx config profile copy <src> <dst>  # Copy profile

Environment override:
  DX_PROFILE=staging dx hash ...    # Use specific profile

Config file structure (config.toml):

  active_profile = "dev"

  [profiles.dev]
  api_url = "http://localhost:3000"
  debug = true

  [profiles.staging]
  api_url = "https://staging.example.com"

  [profiles.prod]
  api_url = "https://api.example.com"
"#
    );

    println!();

    // =========================================================================
    // USE CASES
    // =========================================================================

    println!("--- Use Cases ---");
    println!(
        r#"
1. DEVELOPMENT vs PRODUCTION
   Different API endpoints, debug settings

2. MULTIPLE ACCOUNTS
   Different API keys for different services

3. TEAM PROFILES
   Shared settings for team members

4. TESTING
   Isolated config for test runs

5. CI/CD
   Profile per deployment environment
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Profile system features:");
    println!("  1. Named configuration sets");
    println!("  2. Switch between profiles");
    println!("  3. Environment variable override");
    println!("  4. Copy/create/delete operations");
    println!("  5. Common for multi-environment apps");
}
