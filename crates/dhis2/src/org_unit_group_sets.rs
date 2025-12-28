//! DHIS2 organisation unit group sets command.

use crate::Dhis2Client;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OrgUnitGroupSetResponse {
    #[serde(rename = "organisationUnitGroupSets")]
    organisation_unit_group_sets: Vec<OrgUnitGroupSet>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct OrgUnitGroupSet {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "dataDimension")]
    pub data_dimension: Option<bool>,
    #[serde(rename = "compulsory")]
    pub compulsory: Option<bool>,
    #[serde(rename = "includeSubhierarchyInAnalytics")]
    pub include_subhierarchy: Option<bool>,
}

/// Fetch organisation unit group sets from DHIS2.
pub fn fetch(client: &Dhis2Client, limit: usize) -> Result<Vec<OrgUnitGroupSet>> {
    let url = format!("organisationUnitGroupSets.json?fields=*&pageSize={}", limit);

    let response: OrgUnitGroupSetResponse = client.get(&url)?;
    Ok(response.organisation_unit_group_sets)
}

/// Run the org-unit-group-sets subcommand.
pub fn run(client: &Dhis2Client, limit: usize, json: bool) -> Result<()> {
    let group_sets = fetch(client, limit)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&group_sets)?);
        return Ok(());
    }

    if group_sets.is_empty() {
        println!("No organisation unit group sets found.");
        return Ok(());
    }

    println!(
        "{}",
        format!(
            "Organisation Unit Group Sets (showing {})",
            group_sets.len()
        )
        .cyan()
        .bold()
    );
    println!();

    // Print header
    println!(
        "  {:<12} {:<35} {:<12} {:<8} {}",
        "ID".yellow(),
        "Name".yellow(),
        "Code".yellow(),
        "Dim".yellow(),
        "Compulsory".yellow()
    );
    println!("  {}", "-".repeat(80));

    for gs in &group_sets {
        let code = gs.code.as_deref().unwrap_or("-");
        let dim = gs
            .data_dimension
            .map(|d| if d { "Yes" } else { "No" })
            .unwrap_or("-");
        let compulsory = gs
            .compulsory
            .map(|c| if c { "Yes" } else { "No" })
            .unwrap_or("-");

        // Truncate name if too long
        let name = if gs.display_name.len() > 33 {
            format!("{}...", &gs.display_name[..30])
        } else {
            gs.display_name.clone()
        };

        println!(
            "  {:<12} {:<35} {:<12} {:<8} {}",
            gs.id.cyan(),
            name,
            code,
            dim,
            compulsory.dimmed()
        );
    }

    Ok(())
}
