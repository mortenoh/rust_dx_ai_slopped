//! # Optional Telemetry
//!
//! This example shows how to implement opt-in usage analytics.
//!
//! Run with: `cargo run --example 1003_telemetry_opt_in`

#![allow(dead_code)]

fn main() {
    println!("=== Optional Telemetry ===\n");

    // =========================================================================
    // OPT-IN DESIGN
    // =========================================================================

    println!("--- Opt-In Design ---");
    println!(
        r#"
Key principles for ethical telemetry:

1. OPT-IN by default (never opt-out)
2. Clearly explain what's collected
3. Allow users to disable at any time
4. Don't collect sensitive data
5. Be transparent about data usage

First run prompt:

  dx v1.0.0

  Help improve dx by sharing anonymous usage data?
  This includes:
    - Commands used (not arguments)
    - OS and architecture
    - Errors encountered (not file paths)

  [Y]es  [N]o  [L]ater

  Learn more: https://dx.example.com/telemetry
"#
    );

    println!();

    // =========================================================================
    // CONFIGURATION
    // =========================================================================

    println!("--- Configuration ---");
    println!(
        r#"
Telemetry configuration:

// src/config/telemetry.rs
use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {{
    /// Whether telemetry is enabled
    pub enabled: bool,

    /// Unique installation ID (random, not user-identifiable)
    pub installation_id: String,

    /// Whether user has made a choice
    pub prompted: bool,
}}

impl Default for TelemetryConfig {{
    fn default() -> Self {{
        Self {{
            enabled: false,  // Default OFF
            installation_id: uuid::Uuid::new_v4().to_string(),
            prompted: false,
        }}
    }}
}}

// CLI commands
#[derive(Subcommand)]
enum TelemetryCommands {{
    /// Enable telemetry
    Enable,
    /// Disable telemetry
    Disable,
    /// Show telemetry status
    Status,
}}
"#
    );

    println!();

    // =========================================================================
    // WHAT TO COLLECT
    // =========================================================================

    println!("--- What to Collect ---");
    println!(
        r#"
Collect only non-sensitive, aggregate data:

#[derive(Serialize)]
struct TelemetryEvent {{
    // Anonymous installation ID
    installation_id: String,

    // Command used (NOT arguments/files)
    command: String,  // e.g., "hash", "encode"

    // Platform info
    os: String,       // "linux", "macos", "windows"
    arch: String,     // "x86_64", "aarch64"

    // Outcome
    success: bool,
    duration_ms: u64,

    // Error type (NOT message/details)
    error_type: Option<String>,  // e.g., "FileNotFound"

    // Version
    version: String,
}}

What NOT to collect:
  ✗ File paths
  ✗ File contents
  ✗ Command arguments
  ✗ Environment variables
  ✗ IP addresses
  ✗ Usernames
  ✗ Error messages with paths
"#
    );

    println!();

    // =========================================================================
    // SENDING DATA
    // =========================================================================

    println!("--- Sending Data ---");
    println!(
        r#"
Send telemetry asynchronously:

pub struct Telemetry {{
    config: TelemetryConfig,
    client: reqwest::Client,
}}

impl Telemetry {{
    pub fn new(config: TelemetryConfig) -> Self {{
        Self {{
            config,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
        }}
    }}

    pub fn is_enabled(&self) -> bool {{
        self.config.enabled && !self.is_disabled_by_env()
    }}

    fn is_disabled_by_env(&self) -> bool {{
        std::env::var("DX_NO_TELEMETRY").is_ok() ||
        std::env::var("DO_NOT_TRACK").is_ok()
    }}

    pub async fn send(&self, event: TelemetryEvent) {{
        if !self.is_enabled() {{
            return;
        }}

        // Fire and forget - don't block CLI
        let client = self.client.clone();
        tokio::spawn(async move {{
            let _ = client
                .post("https://telemetry.example.com/events")
                .json(&event)
                .send()
                .await;
            // Silently ignore errors
        }});
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // PRIVACY CONTROLS
    // =========================================================================

    println!("--- Privacy Controls ---");
    println!(
        r#"
Multiple ways to disable:

1. CLI COMMAND
   dx telemetry disable

2. CONFIG FILE
   # ~/.config/dx/config.toml
   [telemetry]
   enabled = false

3. ENVIRONMENT VARIABLE
   export DX_NO_TELEMETRY=1
   # or
   export DO_NOT_TRACK=1  # Universal standard

4. COMPILE-TIME (for enterprise)
   cargo build --no-default-features
   # Removes telemetry code entirely

Check status:
   $ dx telemetry status
   Telemetry: disabled
   Installation ID: abc123...

   To enable:  dx telemetry enable
   To disable: dx telemetry disable

   Learn more: https://dx.example.com/telemetry
"#
    );

    println!();

    // =========================================================================
    // TRANSPARENCY
    // =========================================================================

    println!("--- Transparency ---");
    println!(
        r#"
Be transparent about telemetry:

1. DOCUMENTATION
   Create a dedicated telemetry page:
   https://dx.example.com/telemetry

   Explain:
   - Exactly what is collected
   - How data is used
   - How long it's retained
   - How to opt out

2. IN-APP DISCLOSURE
   $ dx --version
   dx 1.0.0
   Telemetry: enabled (dx telemetry disable to opt out)

3. OPEN SOURCE THE COLLECTOR
   Publish the telemetry server code
   Let users verify what's collected

4. DATA ACCESS
   Allow users to request their data:
   dx telemetry export > my-data.json

5. AGGREGATE ONLY
   Only publish aggregate statistics
   Never identify individual users
"#
    );

    println!();

    // =========================================================================
    // ALTERNATIVES
    // =========================================================================

    println!("--- Alternatives to Telemetry ---");
    println!(
        r#"
Consider alternatives:

1. CRASH REPORTS (opt-in)
   Only collect when errors occur
   Include stack trace (sanitized)

2. GITHUB ISSUES
   Encourage users to report issues
   More context, direct feedback

3. SURVEYS
   Occasional in-app survey prompts
   Direct user feedback

4. DOWNLOAD COUNTS
   GitHub/crates.io provide this
   No user tracking needed

5. FEATURE FLAGS
   Use feature flags to test
   A/B testing without telemetry

Example crash report:

use sentry::{{init, capture_error}};

fn main() {{
    if config.crash_reports_enabled {{
        let _guard = init("https://key@sentry.io/123");
    }}

    if let Err(e) = run() {{
        capture_error(&e);
        // ...
    }}
}}
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Optional telemetry:");
    println!("  1. Always opt-in, never opt-out");
    println!("  2. Collect only anonymous, aggregate data");
    println!("  3. Provide multiple disable methods");
    println!("  4. Be transparent about collection");
    println!("  5. Respect DO_NOT_TRACK standard");
}
