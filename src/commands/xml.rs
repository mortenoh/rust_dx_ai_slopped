//! XML command - XML utilities.

use crate::cli::commands::xml::{XmlArgs, XmlCommand};
use anyhow::{Context, Result};
use colored::Colorize;
use quick_xml::events::{BytesText, Event};
use quick_xml::{Reader, Writer};
use serde_json::{Map, Value};
use std::fs;
use std::io::{self, Cursor, Read};
use std::path::PathBuf;

/// Run the xml command
pub fn run(args: XmlArgs) -> Result<()> {
    match args.command {
        XmlCommand::Format { input, indent } => cmd_format(input, indent),
        XmlCommand::Validate { input, quiet } => cmd_validate(input, quiet),
        XmlCommand::ToJson { input, pretty } => cmd_to_json(input, pretty),
    }
}

fn read_input(input: Option<PathBuf>) -> Result<String> {
    match input {
        Some(path) if path.to_string_lossy() == "-" => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
        Some(path) => fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.display())),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn cmd_format(input: Option<PathBuf>, indent_size: usize) -> Result<()> {
    let content = read_input(input)?;

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', indent_size);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let elem = e;
                writer.write_event(Event::Start(elem))?;
            }
            Ok(Event::End(e)) => {
                let elem = e;
                writer.write_event(Event::End(elem))?;
            }
            Ok(Event::Empty(e)) => {
                let elem = e;
                writer.write_event(Event::Empty(elem))?;
            }
            Ok(Event::Text(e)) => {
                let text = e.decode()?;
                if !text.trim().is_empty() {
                    writer.write_event(Event::Text(BytesText::new(&text)))?;
                }
            }
            Ok(Event::CData(e)) => {
                writer.write_event(Event::CData(e))?;
            }
            Ok(Event::Comment(e)) => {
                writer.write_event(Event::Comment(e))?;
            }
            Ok(Event::Decl(e)) => {
                writer.write_event(Event::Decl(e))?;
            }
            Ok(Event::PI(e)) => {
                writer.write_event(Event::PI(e))?;
            }
            Ok(Event::DocType(e)) => {
                writer.write_event(Event::DocType(e))?;
            }
            Ok(Event::GeneralRef(e)) => {
                writer.write_event(Event::GeneralRef(e))?;
            }
            Err(e) => anyhow::bail!("Error parsing XML: {}", e),
        }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();
    let formatted = String::from_utf8(result)?;
    println!("{}", formatted);
    Ok(())
}

fn cmd_validate(input: Option<PathBuf>, quiet: bool) -> Result<()> {
    let content = read_input(input)?;

    let mut reader = Reader::from_str(&content);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => {
                if !quiet {
                    eprintln!("{}: {}", "Invalid XML".red(), e);
                }
                anyhow::bail!("Invalid XML syntax");
            }
        }
        buf.clear();
    }

    if !quiet {
        println!("{}", "Valid XML".green());
    }
    Ok(())
}

fn cmd_to_json(input: Option<PathBuf>, pretty: bool) -> Result<()> {
    let content = read_input(input)?;

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    let json = xml_to_json(&mut reader)?;

    let output = if pretty {
        serde_json::to_string_pretty(&json)?
    } else {
        serde_json::to_string(&json)?
    };

    println!("{}", output);
    Ok(())
}

fn xml_to_json(reader: &mut Reader<&[u8]>) -> Result<Value> {
    let mut buf = Vec::new();
    let mut stack: Vec<(String, Map<String, Value>, Vec<Value>)> = Vec::new();
    let mut root: Option<Value> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = Map::new();

                // Process attributes
                for attr in e.attributes().flatten() {
                    let key = format!("@{}", String::from_utf8_lossy(attr.key.as_ref()));
                    let value = attr.unescape_value()?.to_string();
                    attrs.insert(key, Value::String(value));
                }

                stack.push((name, attrs, Vec::new()));
            }
            Ok(Event::End(_)) => {
                if let Some((name, attrs, children)) = stack.pop() {
                    let mut obj = attrs;

                    // Group children by name
                    let mut child_map: Map<String, Value> = Map::new();
                    for child in children {
                        if let Value::Object(child_obj) = child {
                            for (k, v) in child_obj {
                                if let Some(existing) = child_map.get_mut(&k) {
                                    // Convert to array if multiple children with same name
                                    if let Value::Array(arr) = existing {
                                        arr.push(v);
                                    } else {
                                        let prev = existing.clone();
                                        *existing = Value::Array(vec![prev, v]);
                                    }
                                } else {
                                    child_map.insert(k, v);
                                }
                            }
                        } else {
                            // Text content
                            obj.insert("#text".to_string(), child);
                        }
                    }

                    // Merge child elements into obj
                    for (k, v) in child_map {
                        obj.insert(k, v);
                    }

                    let element = Value::Object({
                        let mut wrapper = Map::new();
                        wrapper.insert(name, Value::Object(obj));
                        wrapper
                    });

                    if let Some((_, _, parent_children)) = stack.last_mut() {
                        parent_children.push(element);
                    } else {
                        root = Some(element);
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = Map::new();

                for attr in e.attributes().flatten() {
                    let key = format!("@{}", String::from_utf8_lossy(attr.key.as_ref()));
                    let value = attr.unescape_value()?.to_string();
                    attrs.insert(key, Value::String(value));
                }

                let element = Value::Object({
                    let mut wrapper = Map::new();
                    wrapper.insert(name, Value::Object(attrs));
                    wrapper
                });

                if let Some((_, _, parent_children)) = stack.last_mut() {
                    parent_children.push(element);
                } else {
                    root = Some(element);
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.decode()?.to_string();
                if !text.trim().is_empty() {
                    if let Some((_, _, children)) = stack.last_mut() {
                        children.push(Value::String(text));
                    }
                }
            }
            Ok(_) => {} // Ignore other events
            Err(e) => anyhow::bail!("Error parsing XML: {}", e),
        }
        buf.clear();
    }

    root.ok_or_else(|| anyhow::anyhow!("Empty XML document"))
}
