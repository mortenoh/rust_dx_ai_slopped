//! DHIS2 system info command.

use super::client::Dhis2Client;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SystemInfo {
    #[serde(rename = "contextPath")]
    context_path: Option<String>,
    version: Option<String>,
    revision: Option<String>,
    #[serde(rename = "buildTime")]
    build_time: Option<String>,
    #[serde(rename = "serverDate")]
    server_date: Option<String>,
    #[serde(rename = "serverTimeZoneId")]
    server_timezone: Option<String>,
    #[serde(rename = "systemId")]
    system_id: Option<String>,
    #[serde(rename = "systemName")]
    system_name: Option<String>,
    #[serde(rename = "instanceBaseUrl")]
    instance_base_url: Option<String>,
    #[serde(rename = "databaseInfo")]
    database_info: Option<DatabaseInfo>,
}

#[derive(Debug, Deserialize)]
struct DatabaseInfo {
    name: Option<String>,
    user: Option<String>,
    #[serde(rename = "spatialSupport")]
    spatial_support: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Pager {
    total: i64,
}

#[derive(Debug, Deserialize)]
struct PagedResponse {
    pager: Option<Pager>,
}

/// Run the info subcommand.
pub fn run(client: &Dhis2Client) -> Result<()> {
    println!("{}", "=== DHIS2 System Information ===".cyan().bold());
    println!();

    // Fetch system info
    let info: SystemInfo = client.get("system/info.json")?;

    println!("{}", "Server".yellow());
    println!(
        "  System Name:   {}",
        info.system_name.as_deref().unwrap_or("-")
    );
    println!(
        "  Version:       {}",
        info.version.as_deref().unwrap_or("-").green()
    );
    println!(
        "  Revision:      {}",
        info.revision.as_deref().unwrap_or("-")
    );
    println!(
        "  Build Time:    {}",
        info.build_time.as_deref().unwrap_or("-")
    );
    println!(
        "  Server Date:   {}",
        info.server_date.as_deref().unwrap_or("-")
    );
    println!(
        "  Timezone:      {}",
        info.server_timezone.as_deref().unwrap_or("-")
    );
    println!(
        "  Instance URL:  {}",
        info.instance_base_url.as_deref().unwrap_or("-")
    );
    println!("  System ID:     {}", info.system_id.as_deref().unwrap_or("-"));

    if let Some(db) = &info.database_info {
        println!();
        println!("{}", "Database".yellow());
        println!("  Name:          {}", db.name.as_deref().unwrap_or("-"));
        println!("  User:          {}", db.user.as_deref().unwrap_or("-"));
        println!(
            "  Spatial:       {}",
            db.spatial_support
                .map(|s| if s { "Yes".green() } else { "No".red() })
                .unwrap_or_else(|| "-".normal())
        );
    }

    // Fetch object counts
    println!();
    println!("{}", "Metadata Statistics".yellow());

    let endpoints = [
        ("organisationUnits", "Organisation Units"),
        ("dataElements", "Data Elements"),
        ("indicators", "Indicators"),
        ("dataSets", "Data Sets"),
        ("programs", "Programs"),
        ("users", "Users"),
        ("dashboards", "Dashboards"),
    ];

    for (endpoint, label) in endpoints {
        match client.get::<PagedResponse>(&format!("{}.json?paging=true&pageSize=1", endpoint)) {
            Ok(resp) => {
                if let Some(pager) = resp.pager {
                    println!("  {:22} {:>8}", label, pager.total.to_string().cyan());
                }
            }
            Err(_) => {
                // Skip endpoints that fail (might not have permission)
            }
        }
    }

    println!();
    println!("{}", "Connection".yellow());
    println!("  Base URL:      {}", client.base_url());
    println!("  API Path:      /api/");
    println!("  Auth Method:   Basic Auth");

    Ok(())
}
