//! Handles item ID mapping generation.

use failure::Error;
use indexmap::IndexMap;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

mod mappings;
mod rust;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemReport {
    #[serde(flatten)]
    pub mappings: IndexMap<String, Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub protocol_id: i32,
}

pub fn load_report(path: &str) -> Result<ItemReport, Error> {
    let mut file = File::open(path)?;

    let mut string = String::new();
    file.read_to_string(&mut string)?;

    let report = serde_json::from_str(&string)?;

    Ok(report)
}

pub fn generate_mappings_file(input: &str, output: &str) -> Result<(), Error> {
    info!("Parsing data file");
    let report = load_report(input)?;
    info!("Data file parsed successfully");

    info!("Generating mappings file {}", output);

    let buf = mappings::generate_mappings_file(report, true)?;
    let mut file = File::create(output)?;
    file.write_all(&buf)?;

    info!("Success");

    Ok(())
}

pub fn generate_rust(input: &str, output: &str) -> Result<(), Error> {
    info!("Parsing data file");
    let report = load_report(input)?;
    info!("Data file parsed successfully");

    info!("Generating Rust code");
    let buf = rust::generate_rust(report)?;
    let mut file = File::create(output)?;
    file.write_all(buf.as_bytes())?;
    info!("Generated code");

    info!("Formatting code with rustfmt");
    Command::new("rustfmt").arg(output).output()?;
    info!("Success");

    Ok(())
}
