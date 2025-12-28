//! Template command - Jinja2-style templating.

use crate::cli::commands::template::{TemplateArgs, TemplateCommand};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use tera::Tera;

/// Run the template command
pub fn run(args: TemplateArgs) -> Result<()> {
    match args.command {
        TemplateCommand::Render {
            template,
            data,
            json,
        } => {
            let template_content = fs::read_to_string(&template)
                .with_context(|| format!("Failed to read template: {}", template.display()))?;

            // Get context data
            let context_data: serde_json::Value = if let Some(json_str) = json {
                serde_json::from_str(&json_str).context("Failed to parse JSON data")?
            } else if let Some(data_file) = data {
                let data_content = fs::read_to_string(&data_file).with_context(|| {
                    format!("Failed to read data file: {}", data_file.display())
                })?;
                serde_json::from_str(&data_content).context("Failed to parse data file as JSON")?
            } else {
                serde_json::Value::Object(serde_json::Map::new())
            };

            // Create Tera instance and add template
            let mut tera = Tera::default();
            tera.add_raw_template("template", &template_content)
                .context("Failed to parse template")?;

            // Convert JSON to Tera context
            let context = tera::Context::from_value(context_data)
                .map_err(|e| anyhow::anyhow!("Failed to create context: {}", e))?;

            // Render
            let result = tera
                .render("template", &context)
                .context("Failed to render template")?;

            print!("{}", result);
            Ok(())
        }
        TemplateCommand::Validate { template } => {
            let template_content = fs::read_to_string(&template)
                .with_context(|| format!("Failed to read template: {}", template.display()))?;

            let mut tera = Tera::default();
            match tera.add_raw_template("template", &template_content) {
                Ok(_) => {
                    println!("{}", "Template is valid".green());
                    Ok(())
                }
                Err(e) => {
                    eprintln!("{}: {}", "Template error".red(), e);
                    anyhow::bail!("Invalid template")
                }
            }
        }
    }
}
