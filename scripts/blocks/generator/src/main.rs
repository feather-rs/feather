//! This program is used to generate block state ID mappings
//! and corresponding Rust code. It reads from vanilla block.json
//! files. See `format.md` for more information.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::App;
use std::collections::HashMap;
use std::fs::File;

use failure::Error;
use std::io::{BufReader, BufWriter};
use std::process::exit;

/// The block state ID to use when a block
/// in the native file was not found
/// in the input file. This would happen
/// when the input file is an older version
/// than the native version.
const DEFAULT_STATE_ID: u32 = 1; // Stone

/// Deserializable struct representing a block
/// data report from Vanilla.
#[derive(Clone, Debug, Deserialize, Deref, DerefMut)]
struct BlockReport {
    #[serde(flatten)]
    blocks: HashMap<String, Block>,
}

/// A block entry in the data report.
#[derive(Clone, Debug, Deserialize)]
struct Block {
    states: Vec<State>,
    properties: Option<BlockProperties>,
}

/// List of block properties.
#[derive(Clone, Debug, Deserialize, Deref, DerefMut)]
struct BlockProperties {
    #[serde(flatten)]
    props: HashMap<String, Vec<String>>,
}

/// A block state from the data report.
#[derive(Clone, Debug, Deserialize)]
struct State {
    id: u32,
    properties: Option<StateProperties>,
}

/// Properties of a block state from the data report.
#[derive(Clone, Debug, Deserialize, Deref, DerefMut)]
struct StateProperties {
    #[serde(flatten)]
    props: HashMap<String, String>,
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    if let Err(e) = run() {
        error!("An error occurred: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("mappings") => {
            let args = matches.subcommand_matches("mappings").unwrap();
            generate_mappings_file(
                args.value_of("input").unwrap(),
                args.value_of("output").unwrap(),
                args.value_of("native").unwrap(),
            )?;
        }
        Some(s) => {
            error!("Invalid subcommand {}", s);
            return Ok(());
        }
        None => {
            error!("No subcommand specified");
            return Ok(());
        }
    }

    Ok(())
}

fn generate_mappings_file(input: &str, output: &str, native_input: &str) -> Result<(), Error> {
    info!(
        "Generating mappings file {} using input report {} and native report {}",
        output, input, native_input
    );

    let in_file = File::open(input)?;
    let out_file = File::create(output)?;
    let native_file = File::open(native_input)?;

    info!("Parsing data files");

    let report: BlockReport = serde_json::from_reader(BufReader::new(&in_file))?;
    let native_report: BlockReport = serde_json::from_reader(BufReader::new(&native_file))?;

    info!("Parsing successful");

    let out = BufWriter::new(&out_file);

    // Write header to output file
    // See format.md

    // Go through block types in native
    // file and attempt to find corresponding
    // entry in input file.
    for (string_id, block) in &native_report.blocks {}

    Ok(())
}
