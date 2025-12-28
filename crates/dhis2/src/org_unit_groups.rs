//! DHIS2 organisation unit groups command.

use crate::Dhis2Client;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OrgUnitGroupResponse {
    #[serde(rename = "organisationUnitGroups")]
    organisation_unit_groups: Vec<OrgUnitGroup>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct OrgUnitGroup {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "organisationUnitGroupSet")]
    pub group_set: Option<GroupSetRef>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct GroupSetRef {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

/// Fetch organisation unit groups from DHIS2.
pub fn fetch(client: &Dhis2Client, limit: usize) -> Result<Vec<OrgUnitGroup>> {
    let url = format!("organisationUnitGroups.json?fields=*&pageSize={}", limit);

    let response: OrgUnitGroupResponse = client.get(&url)?;
    Ok(response.organisation_unit_groups)
}

/// Run the org-unit-groups subcommand.
pub fn run(client: &Dhis2Client, limit: usize, json: bool) -> Result<()> {
    let groups = fetch(client, limit)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&groups)?);
        return Ok(());
    }

    if groups.is_empty() {
        println!("No organisation unit groups found.");
        return Ok(());
    }

    println!(
        "{}",
        format!("Organisation Unit Groups (showing {})", groups.len())
            .cyan()
            .bold()
    );
    println!();

    // Print header
    println!(
        "  {:<12} {:<35} {:<12} {}",
        "ID".yellow(),
        "Name".yellow(),
        "Code".yellow(),
        "Group Set".yellow()
    );
    println!("  {}", "-".repeat(80));

    for group in &groups {
        let code = group.code.as_deref().unwrap_or("-");
        let group_set = group
            .group_set
            .as_ref()
            .and_then(|gs| gs.display_name.as_deref())
            .unwrap_or("-");

        // Truncate name if too long
        let name = if group.display_name.len() > 33 {
            format!("{}...", &group.display_name[..30])
        } else {
            group.display_name.clone()
        };

        println!(
            "  {:<12} {:<35} {:<12} {}",
            group.id.cyan(),
            name,
            code,
            group_set.dimmed()
        );
    }

    Ok(())
}
