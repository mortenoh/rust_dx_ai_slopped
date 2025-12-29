//! DHIS2 data value sets command.
//!
//! Fetches data values from the dataValueSets endpoint.
//! See: https://docs.dhis2.org/en/develop/using-the-api/dhis-core-version-241/data.html

use crate::Dhis2Client;
use anyhow::{bail, Result};
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct DataValueSet {
    #[serde(rename = "dataSet")]
    pub data_set: Option<String>,
    #[serde(rename = "completeDate")]
    pub complete_date: Option<String>,
    pub period: Option<String>,
    #[serde(rename = "orgUnit")]
    pub org_unit: Option<String>,
    #[serde(rename = "dataValues", default)]
    pub data_values: Vec<DataValue>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct DataValue {
    #[serde(rename = "dataElement")]
    pub data_element: String,
    pub period: Option<String>,
    #[serde(rename = "orgUnit")]
    pub org_unit: Option<String>,
    #[serde(rename = "categoryOptionCombo")]
    pub category_option_combo: Option<String>,
    #[serde(rename = "attributeOptionCombo")]
    pub attribute_option_combo: Option<String>,
    pub value: String,
    #[serde(rename = "storedBy")]
    pub stored_by: Option<String>,
    pub created: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "followUp")]
    pub follow_up: Option<bool>,
}

/// Fetch data values from DHIS2.
pub fn fetch(
    client: &Dhis2Client,
    data_set: &str,
    org_unit: &str,
    period: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: usize,
) -> Result<DataValueSet> {
    let mut url = format!(
        "dataValueSets.json?dataSet={}&orgUnit={}&limit={}",
        data_set, org_unit, limit
    );

    if let Some(p) = period {
        url.push_str(&format!("&period={}", p));
    } else if let (Some(start), Some(end)) = (start_date, end_date) {
        url.push_str(&format!("&startDate={}&endDate={}", start, end));
    } else {
        bail!("Either --period or both --start-date and --end-date are required");
    }

    let response: DataValueSet = client.get(&url)?;
    Ok(response)
}

/// Run the data-values subcommand.
#[allow(clippy::too_many_arguments)]
pub fn run(
    client: &Dhis2Client,
    data_set: String,
    org_unit: String,
    period: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: usize,
    json: bool,
) -> Result<()> {
    let data_value_set = fetch(
        client,
        &data_set,
        &org_unit,
        period.as_deref(),
        start_date.as_deref(),
        end_date.as_deref(),
        limit,
    )?;

    if json {
        println!("{}", serde_json::to_string_pretty(&data_value_set)?);
        return Ok(());
    }

    let values = &data_value_set.data_values;

    if values.is_empty() {
        println!("No data values found.");
        return Ok(());
    }

    println!(
        "{}",
        format!("Data Values (showing {})", values.len())
            .cyan()
            .bold()
    );
    if let Some(ds) = &data_value_set.data_set {
        println!("  Data Set: {}", ds.cyan());
    }
    if let Some(ou) = &data_value_set.org_unit {
        println!("  Org Unit: {}", ou);
    }
    if let Some(p) = &data_value_set.period {
        println!("  Period:   {}", p);
    }
    println!();

    // Print header
    println!(
        "  {:<12} {:<12} {:<15} {:<12} {}",
        "DataElement".yellow(),
        "Period".yellow(),
        "COC".yellow(),
        "Value".yellow(),
        "LastUpdated".yellow()
    );
    println!("  {}", "-".repeat(75));

    for dv in values {
        let period = dv.period.as_deref().unwrap_or("-");
        let coc = dv
            .category_option_combo
            .as_deref()
            .map(|s| if s.len() > 13 { &s[..13] } else { s })
            .unwrap_or("-");
        let last_updated = dv
            .last_updated
            .as_deref()
            .map(|s| if s.len() > 10 { &s[..10] } else { s })
            .unwrap_or("-");

        // Truncate data element if too long
        let de = if dv.data_element.len() > 11 {
            &dv.data_element[..11]
        } else {
            &dv.data_element
        };

        // Color value based on type
        let value_display = if dv.value.parse::<f64>().is_ok() {
            dv.value.green().to_string()
        } else {
            dv.value.clone()
        };

        println!(
            "  {:<12} {:<12} {:<15} {:<12} {}",
            de.cyan(),
            period,
            coc,
            value_display,
            last_updated.dimmed()
        );
    }

    Ok(())
}
