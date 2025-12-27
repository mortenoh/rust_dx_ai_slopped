//! # Environment Variable Integration
//!
//! This example shows how to integrate environment variables with config.
//!
//! Run with: `cargo run --example 0302_env_integration`

#![allow(dead_code)]

use std::env;

// =========================================================================
// CLAP ENV INTEGRATION
// =========================================================================

/// Example of clap with env var support
mod cli_example {
    use clap::Parser;

    #[derive(Parser, Debug)]
    pub struct Args {
        /// API key (can also use DX_API_KEY env var)
        #[arg(long, env = "DX_API_KEY")]
        pub api_key: Option<String>,

        /// Debug mode (DX_DEBUG env var)
        #[arg(long, env = "DX_DEBUG")]
        pub debug: bool,

        /// Port number (DX_PORT env var)
        #[arg(short, long, env = "DX_PORT", default_value = "8080")]
        pub port: u16,

        /// Log level (DX_LOG_LEVEL env var)
        #[arg(long, env = "DX_LOG_LEVEL", default_value = "info")]
        pub log_level: String,
    }
}

// =========================================================================
// CONFIG + ENV MERGE
// =========================================================================

/// Configuration with env override support
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub debug: bool,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    /// Load config with environment variable overrides
    pub fn load() -> Self {
        // Start with defaults
        let mut config = Self {
            api_key: None,
            debug: false,
            port: 8080,
            log_level: "info".to_string(),
        };

        // Override with env vars
        if let Ok(key) = env::var("DX_API_KEY") {
            config.api_key = Some(key);
        }
        if let Ok(val) = env::var("DX_DEBUG") {
            config.debug = val == "1" || val.eq_ignore_ascii_case("true");
        }
        if let Ok(val) = env::var("DX_PORT") {
            if let Ok(port) = val.parse() {
                config.port = port;
            }
        }
        if let Ok(val) = env::var("DX_LOG_LEVEL") {
            config.log_level = val;
        }

        config
    }
}

fn main() {
    println!("=== Environment Variable Integration ===\n");

    // =========================================================================
    // CLAP ENV SUPPORT
    // =========================================================================

    println!("--- Clap #[arg(env = ...)] ---");
    println!(
        r#"
Clap can read from environment variables:

#[derive(Parser)]
struct Args {{
    #[arg(long, env = "DX_API_KEY")]
    api_key: Option<String>,

    #[arg(long, env = "DX_DEBUG")]
    debug: bool,

    #[arg(short, long, env = "DX_PORT", default_value = "8080")]
    port: u16,
}}

Priority order:
1. Command line argument (highest)
2. Environment variable
3. Default value (lowest)
"#
    );

    println!();

    // =========================================================================
    // CURRENT ENV VARS
    // =========================================================================

    println!("--- Current DX_* Environment Variables ---");
    let dx_vars: Vec<_> = env::vars().filter(|(k, _)| k.starts_with("DX_")).collect();

    if dx_vars.is_empty() {
        println!("  (none set)");
    } else {
        for (key, value) in dx_vars {
            println!("  {}={}", key, value);
        }
    }

    println!();

    // =========================================================================
    // CONFIG LOADING
    // =========================================================================

    println!("--- Loading Config with Env Overrides ---");
    let config = Config::load();
    println!("  api_key: {:?}", config.api_key);
    println!("  debug: {}", config.debug);
    println!("  port: {}", config.port);
    println!("  log_level: {}", config.log_level);

    println!();

    // =========================================================================
    // BEST PRACTICES
    // =========================================================================

    println!("--- Best Practices ---");
    println!(
        r#"
1. PREFIX all env vars (DX_, MYAPP_, etc.)
2. DOCUMENT env vars in --help
3. PRIORITY: CLI > env > config file > default
4. BOOLEAN vars: accept "1", "true", "yes"
5. SENSITIVE data: prefer env vars over config files
6. Use NO_COLOR standard for color disable

Common patterns:
  DX_DEBUG=1
  DX_LOG_LEVEL=debug
  NO_COLOR=1          # Standard for disabling color
  DX_CONFIG=/path     # Custom config file
"#
    );

    println!();

    // =========================================================================
    // TESTING EXAMPLE
    // =========================================================================

    println!("--- Test with Environment ---");
    println!(
        r#"
# Set env and run:
DX_DEBUG=1 DX_PORT=3000 cargo run -- --help

# Or export:
export DX_API_KEY="secret123"
dx hash -s "test"
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Environment variable integration:");
    println!("  1. Use clap's #[arg(env = ...)] for CLI args");
    println!("  2. Prefix all vars with app name");
    println!("  3. CLI args override env vars");
    println!("  4. Document all env vars in --help");
    println!("  5. Use env vars for secrets");
}
