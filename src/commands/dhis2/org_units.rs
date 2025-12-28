//! DHIS2 organisation units command.

use super::client::Dhis2Client;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct OrgUnitResponse {
    #[serde(rename = "organisationUnits")]
    organisation_units: Vec<OrgUnit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrgUnit {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub code: Option<String>,
    pub level: i32,
    pub path: String,
    #[serde(default)]
    pub parent: Option<ParentRef>,
    #[serde(default)]
    pub geometry: Option<Value>,
    #[serde(rename = "featureType")]
    pub feature_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParentRef {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

impl OrgUnit {
    /// Get geometry type from GeoJSON.
    pub fn geometry_type(&self) -> Option<&str> {
        self.geometry
            .as_ref()
            .and_then(|g| g.get("type"))
            .and_then(|t| t.as_str())
    }
}

/// Fetch organisation units from DHIS2.
pub fn fetch(
    client: &Dhis2Client,
    level: Option<i32>,
    limit: usize,
    include_geometry: bool,
) -> Result<Vec<OrgUnit>> {
    let mut fields =
        "id,displayName,name,shortName,code,level,path,parent[id,displayName],featureType";

    let fields_with_geom;
    if include_geometry {
        fields_with_geom = format!("{},geometry", fields);
        fields = &fields_with_geom;
    }

    let mut url = format!(
        "organisationUnits.json?fields={}&pageSize={}",
        fields, limit
    );

    if let Some(lvl) = level {
        url.push_str(&format!("&filter=level:eq:{}", lvl));
    }

    let response: OrgUnitResponse = client.get(&url)?;
    Ok(response.organisation_units)
}

/// Run the org-units subcommand.
pub fn run(
    client: &Dhis2Client,
    level: Option<i32>,
    limit: usize,
    geometry: bool,
    json: bool,
) -> Result<()> {
    let org_units = fetch(client, level, limit, geometry)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&org_units)?);
        return Ok(());
    }

    if org_units.is_empty() {
        println!("No organisation units found.");
        return Ok(());
    }

    println!(
        "{}",
        format!("Organisation Units (showing {})", org_units.len())
            .cyan()
            .bold()
    );
    println!();

    // Print header
    println!(
        "  {:<12} {:<4} {:<30} {:<12} {}",
        "ID".yellow(),
        "Lvl".yellow(),
        "Name".yellow(),
        "Code".yellow(),
        "Parent".yellow()
    );
    println!("  {}", "-".repeat(80));

    for ou in &org_units {
        let parent = ou
            .parent
            .as_ref()
            .and_then(|p| p.display_name.as_deref())
            .unwrap_or("-");

        let code = ou.code.as_deref().unwrap_or("-");

        // Truncate name if too long
        let name = if ou.display_name.len() > 28 {
            format!("{}...", &ou.display_name[..25])
        } else {
            ou.display_name.clone()
        };

        println!(
            "  {:<12} {:>4} {:<30} {:<12} {}",
            ou.id.cyan(),
            ou.level,
            name,
            code,
            parent.dimmed()
        );

        // Show geometry info if requested
        if geometry {
            if let Some(geom_type) = ou.geometry_type() {
                println!(
                    "             {} {}",
                    "geometry:".dimmed(),
                    geom_type.green()
                );
            }
        }
    }

    if let Some(lvl) = level {
        println!();
        println!("  Filtered by level: {}", lvl);
    }

    Ok(())
}
