//! DHIS2 data sets command.

use crate::Dhis2Client;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DataSetResponse {
    #[serde(rename = "dataSets")]
    data_sets: Vec<DataSet>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct DataSet {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "periodType")]
    pub period_type: Option<String>,
    #[serde(rename = "timelyDays")]
    pub timely_days: Option<f64>,
    #[serde(rename = "expiryDays")]
    pub expiry_days: Option<f64>,
    pub description: Option<String>,
}

/// Fetch data sets from DHIS2.
pub fn fetch(client: &Dhis2Client, limit: usize) -> Result<Vec<DataSet>> {
    let fields = "id,displayName,name,shortName,code,periodType,timelyDays,expiryDays,description";
    let url = format!("dataSets.json?fields={}&pageSize={}", fields, limit);

    let response: DataSetResponse = client.get(&url)?;
    Ok(response.data_sets)
}

/// Run the data-sets subcommand.
pub fn run(client: &Dhis2Client, limit: usize, json: bool) -> Result<()> {
    let data_sets = fetch(client, limit)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&data_sets)?);
        return Ok(());
    }

    if data_sets.is_empty() {
        println!("No data sets found.");
        return Ok(());
    }

    println!(
        "{}",
        format!("Data Sets (showing {})", data_sets.len())
            .cyan()
            .bold()
    );
    println!();

    // Print header
    println!(
        "  {:<12} {:<40} {:<12} {}",
        "ID".yellow(),
        "Name".yellow(),
        "Period".yellow(),
        "Code".yellow()
    );
    println!("  {}", "-".repeat(80));

    for ds in &data_sets {
        let period = ds.period_type.as_deref().unwrap_or("-");
        let code = ds.code.as_deref().unwrap_or("-");

        // Truncate name if too long
        let name = if ds.display_name.len() > 38 {
            format!("{}...", &ds.display_name[..35])
        } else {
            ds.display_name.clone()
        };

        // Color-code period types
        let period_colored = match period {
            "Monthly" => period.green(),
            "Quarterly" => period.cyan(),
            "Yearly" => period.yellow(),
            "Weekly" => period.magenta(),
            "Daily" => period.blue(),
            _ => period.normal(),
        };

        println!(
            "  {:<12} {:<40} {:<12} {}",
            ds.id.cyan(),
            name,
            period_colored,
            code.dimmed()
        );
    }

    Ok(())
}
