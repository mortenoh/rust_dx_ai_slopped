//! # Update Checking
//!
//! This example shows how to implement self-update functionality.
//!
//! Run with: `cargo run --example 1002_update_checking`

#![allow(dead_code)]

fn main() {
    println!("=== Update Checking ===\n");

    // =========================================================================
    // CHECK FOR UPDATES
    // =========================================================================

    println!("--- Check for Updates ---");
    println!(
        r#"
Check GitHub releases for new versions:

use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {{
    tag_name: String,
    html_url: String,
    prerelease: bool,
}}

pub async fn check_for_updates() -> anyhow::Result<Option<Release>> {{
    let current = Version::parse(env!("CARGO_PKG_VERSION"))?;

    let client = reqwest::Client::new();
    let response: Release = client
        .get("https://api.github.com/repos/user/dx/releases/latest")
        .header("User-Agent", "dx")
        .send()
        .await?
        .json()
        .await?;

    // Parse version (strip 'v' prefix)
    let latest_str = response.tag_name.trim_start_matches('v');
    let latest = Version::parse(latest_str)?;

    if latest > current && !response.prerelease {{
        Ok(Some(response))
    }} else {{
        Ok(None)
    }}
}}

// Usage
if let Some(release) = check_for_updates().await? {{
    println!("New version available: {{}}", release.tag_name);
    println!("Download: {{}}", release.html_url);
}}
"#
    );

    println!();

    // =========================================================================
    // SELF-UPDATE
    // =========================================================================

    println!("--- Self-Update Command ---");
    println!(
        r#"
Implement a self-update command:

// Using self_update crate
// Cargo.toml: self_update = "0.40"

use self_update::cargo_crate_version;

pub fn run_update() -> anyhow::Result<()> {{
    let status = self_update::backends::github::Update::configure()
        .repo_owner("user")
        .repo_name("dx")
        .bin_name("dx")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    match status {{
        self_update::Status::Updated(v) => {{
            println!("Updated to version {{}}", v);
        }}
        self_update::Status::UpToDate(v) => {{
            println!("Already at latest version {{}}", v);
        }}
    }}

    Ok(())
}}

// CLI integration
#[derive(Subcommand)]
enum Commands {{
    /// Check for and install updates
    Update {{
        /// Check only, don't install
        #[arg(long)]
        check: bool,
    }},
}}
"#
    );

    println!();

    // =========================================================================
    // BACKGROUND UPDATE CHECK
    // =========================================================================

    println!("--- Background Update Check ---");
    println!(
        r#"
Check for updates in the background:

use std::time::{{Duration, SystemTime}};
use std::path::PathBuf;

struct UpdateChecker {{
    cache_file: PathBuf,
    check_interval: Duration,
}}

impl UpdateChecker {{
    pub fn new() -> Self {{
        let cache_dir = directories::ProjectDirs::from("com", "example", "dx")
            .map(|d| d.cache_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        Self {{
            cache_file: cache_dir.join("update_check.json"),
            check_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
        }}
    }}

    pub async fn check_if_needed(&self) -> Option<String> {{
        // Check if we should run
        if !self.should_check() {{
            return None;
        }}

        // Run check in background (don't block CLI)
        tokio::spawn(async {{
            if let Ok(Some(release)) = check_for_updates().await {{
                // Cache the result
                self.save_result(&release.tag_name);
            }}
        }});

        // Return cached result if any
        self.get_cached_update()
    }}

    fn should_check(&self) -> bool {{
        // Don't check if:
        // - DX_NO_UPDATE_CHECK is set
        // - Not a TTY (scripting)
        // - Checked recently

        if std::env::var("DX_NO_UPDATE_CHECK").is_ok() {{
            return false;
        }}

        if !std::io::stdout().is_terminal() {{
            return false;
        }}

        // Check last check time from cache
        self.last_check_expired()
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // UPDATE NOTIFICATION
    // =========================================================================

    println!("--- Update Notification ---");
    println!(
        r#"
Show update notification non-intrusively:

use colored::Colorize;

pub fn maybe_show_update_notice() {{
    let checker = UpdateChecker::new();

    if let Some(new_version) = checker.get_cached_update() {{
        eprintln!(
            "\n{{}} {{}} → {{}} ({{}})\n",
            "Update available:".yellow().bold(),
            env!("CARGO_PKG_VERSION"),
            new_version.green(),
            "dx update".cyan()
        );
    }}
}}

// Call at end of main commands
fn main() -> anyhow::Result<()> {{
    let result = run_cli();

    // Show update notice after command output
    maybe_show_update_notice();

    result
}}

Output:
  $ dx hash myfile.txt
  sha256: abc123...

  Update available: 1.0.0 → 1.1.0 (dx update)
"#
    );

    println!();

    // =========================================================================
    // UPDATE CHANNELS
    // =========================================================================

    println!("--- Update Channels ---");
    println!(
        r#"
Support different update channels:

pub enum Channel {{
    Stable,
    Beta,
    Nightly,
}}

impl Channel {{
    fn get_latest(&self) -> String {{
        match self {{
            Channel::Stable => "https://api.github.com/.../releases/latest",
            Channel::Beta => "https://api.github.com/.../releases?per_page=1",
            Channel::Nightly => "https://nightly.example.com/latest",
        }}.to_string()
    }}
}}

// CLI
#[derive(Args)]
pub struct UpdateArgs {{
    /// Update channel
    #[arg(long, default_value = "stable")]
    channel: Channel,

    /// Allow updating to pre-release versions
    #[arg(long)]
    allow_prerelease: bool,
}}

// Config
#[derive(Deserialize)]
struct Config {{
    update: UpdateConfig,
}}

#[derive(Deserialize)]
struct UpdateConfig {{
    channel: String,
    auto_check: bool,
    check_interval: u64, // seconds
}}
"#
    );

    println!();

    // =========================================================================
    // ROLLBACK
    // =========================================================================

    println!("--- Rollback Support ---");
    println!(
        r#"
Allow rollback to previous version:

pub fn update_with_backup() -> anyhow::Result<()> {{
    let current_exe = std::env::current_exe()?;
    let backup_path = current_exe.with_extension("bak");

    // Backup current binary
    std::fs::copy(&current_exe, &backup_path)?;

    match do_update() {{
        Ok(_) => {{
            // Success - remove backup
            let _ = std::fs::remove_file(&backup_path);
            Ok(())
        }}
        Err(e) => {{
            // Failed - restore backup
            eprintln!("Update failed, restoring previous version...");
            std::fs::copy(&backup_path, &current_exe)?;
            std::fs::remove_file(&backup_path)?;
            Err(e)
        }}
    }}
}}

// Explicit rollback command
#[derive(Subcommand)]
enum UpdateCommands {{
    /// Install latest version
    Install,
    /// Rollback to previous version
    Rollback,
    /// List available versions
    List,
}}
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Update checking:");
    println!("  1. Check GitHub releases for new versions");
    println!("  2. Use self_update crate for self-update");
    println!("  3. Background checks (non-blocking)");
    println!("  4. Non-intrusive notifications");
    println!("  5. Support rollback for safety");
}
