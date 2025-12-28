//! DHIS2 data elements command.

use crate::Dhis2Client;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DataElementResponse {
    #[serde(rename = "dataElements")]
    data_elements: Vec<DataElement>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct DataElement {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "valueType")]
    pub value_type: Option<String>,
    #[serde(rename = "aggregationType")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "domainType")]
    pub domain_type: Option<String>,
    pub description: Option<String>,
}

/// Fetch data elements from DHIS2.
pub fn fetch(
    client: &Dhis2Client,
    limit: usize,
    value_type: Option<&str>,
) -> Result<Vec<DataElement>> {
    let fields =
        "id,displayName,name,shortName,code,valueType,aggregationType,domainType,description";

    let mut url = format!("dataElements.json?fields={}&pageSize={}", fields, limit);

    if let Some(vt) = value_type {
        url.push_str(&format!("&filter=valueType:eq:{}", vt));
    }

    let response: DataElementResponse = client.get(&url)?;
    Ok(response.data_elements)
}

/// Run the elements subcommand.
pub fn run(
    client: &Dhis2Client,
    limit: usize,
    value_type: Option<String>,
    json: bool,
) -> Result<()> {
    let elements = fetch(client, limit, value_type.as_deref())?;

    if json {
        println!("{}", serde_json::to_string_pretty(&elements)?);
        return Ok(());
    }

    if elements.is_empty() {
        println!("No data elements found.");
        return Ok(());
    }

    println!(
        "{}",
        format!("Data Elements (showing {})", elements.len())
            .cyan()
            .bold()
    );
    println!();

    // Print header
    println!(
        "  {:<12} {:<35} {:<12} {:<10} {}",
        "ID".yellow(),
        "Name".yellow(),
        "Type".yellow(),
        "Aggreg".yellow(),
        "Domain".yellow()
    );
    println!("  {}", "-".repeat(85));

    for elem in &elements {
        let value_type = elem.value_type.as_deref().unwrap_or("-");
        let aggregation = elem.aggregation_type.as_deref().unwrap_or("-");
        let domain = elem.domain_type.as_deref().unwrap_or("-");

        // Truncate name if too long
        let name = if elem.display_name.len() > 33 {
            format!("{}...", &elem.display_name[..30])
        } else {
            elem.display_name.clone()
        };

        // Color-code value types
        let value_type_colored = match value_type {
            "NUMBER" => value_type.green(),
            "INTEGER" | "INTEGER_POSITIVE" | "INTEGER_ZERO_OR_POSITIVE" => value_type.cyan(),
            "TEXT" | "LONG_TEXT" => value_type.yellow(),
            "BOOLEAN" | "TRUE_ONLY" => value_type.magenta(),
            "DATE" | "DATETIME" => value_type.blue(),
            _ => value_type.normal(),
        };

        println!(
            "  {:<12} {:<35} {:<12} {:<10} {}",
            elem.id.cyan(),
            name,
            value_type_colored,
            aggregation,
            domain.dimmed()
        );
    }

    if let Some(vt) = &value_type {
        println!();
        println!("  Filtered by value type: {}", vt);
    }

    Ok(())
}
