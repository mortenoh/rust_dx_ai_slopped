//! DHIS2 command implementation.

use crate::cli::commands::dhis2::{Dhis2Args, Dhis2Command};
use anyhow::Result;

pub fn run(args: Dhis2Args) -> Result<()> {
    let client = dx_dhis2::Dhis2Client::new(&args.server, &args.user, &args.password)?;

    match args.command {
        Dhis2Command::Uid {
            count,
            validate,
            json,
            plain,
        } => dx_dhis2::uid::run(count, validate, json, plain),

        Dhis2Command::Info => dx_dhis2::info::run(&client),

        Dhis2Command::OrgUnits {
            level,
            limit,
            geometry,
            json,
        } => dx_dhis2::org_units::run(&client, level, limit, geometry, json),

        Dhis2Command::DataElements {
            limit,
            value_type,
            json,
        } => dx_dhis2::elements::run(&client, limit, value_type, json),

        Dhis2Command::DataSets { limit, json } => dx_dhis2::data_sets::run(&client, limit, json),

        Dhis2Command::OrgUnitGroups { limit, json } => {
            dx_dhis2::org_unit_groups::run(&client, limit, json)
        }

        Dhis2Command::OrgUnitGroupSets { limit, json } => {
            dx_dhis2::org_unit_group_sets::run(&client, limit, json)
        }

        #[cfg(feature = "ui")]
        Dhis2Command::Tui => dx_dhis2::tui::run(client),
    }
}
