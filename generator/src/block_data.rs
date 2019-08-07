use super::WriteExt;
use byteorder::{LittleEndian, WriteBytesExt};
use failure::Error;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

/// The block state ID to use when a block
/// in the native file was not found
/// in the input file. This would happen
/// when the input file is an older version
/// than the native version.
pub const DEFAULT_STATE_ID: u16 = 1; // Stone

/// Deserializable struct representing a block
/// data report from Vanilla.
#[derive(Clone, Debug, Deserialize, Deref, DerefMut)]
pub struct BlockReport {
    #[serde(flatten)]
    pub blocks: IndexMap<String, Block>,
}

/// A block entry in the data report.
#[derive(Clone, Debug, Deserialize)]
pub struct Block {
    pub states: Vec<State>,
    pub properties: Option<BlockProperties>,
}

/// List of block properties.
#[derive(Clone, Debug, Deserialize, Deref, DerefMut)]
pub struct BlockProperties {
    #[serde(flatten)]
    pub props: HashMap<String, Vec<String>>,
}

/// A block state from the data report.
#[derive(Clone, Debug, Deserialize)]
pub struct State {
    pub id: u16,
    pub properties: Option<StateProperties>,
}

/// Properties of a block state from the data report.
#[derive(Clone, Debug, Deserialize, Deref, DerefMut, Default)]
pub struct StateProperties {
    #[serde(flatten)]
    pub props: HashMap<String, String>,
}

pub fn generate_mappings_file(
    input: &str,
    output: &str,
    native_input: &str,
    proto: u32,
    version: &str,
) -> Result<(), Error> {
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

    let mut out = BufWriter::new(&out_file);

    // Write header to output file
    // See block_format.md
    write_header(&mut out, version, proto, false)?;

    // Go through native block types and attempt
    // to find corresponding state ID in report.
    // If it doesn't exist, just set to `DEFAULT_STATE_ID`.
    let mut state_bufs = vec![];
    for (string_id, block) in &native_report.blocks {
        for state in &block.states {
            let mut state_buf = vec![];

            let props = state.properties.clone().unwrap_or_default();
            let props = props.props;

            // Try to find corresponding state ID, defaulting to `DEFAULT_STATE_ID`
            let state_id = find_state_in_report(&report, string_id.as_str(), &props)
                .unwrap_or(DEFAULT_STATE_ID);

            state_buf.write_u16::<LittleEndian>(state.id)?; // Native ID
            state_buf.write_u16::<LittleEndian>(state_id)?;
            state_bufs.push(state_buf);
        }
    }

    out.write_u32::<LittleEndian>(state_bufs.len() as u32)?;
    for buf in state_bufs {
        out.write_all(&buf)?;
    }

    out.flush()?;

    info!("Mappings file generated successfully");
    Ok(())
}

pub fn generate_native_mappings_file(
    input: &str,
    output: &str,
    proto: u32,
    version: &str,
) -> Result<(), Error> {
    info!(
        "Generating native mappings file {} using input report {}",
        output, input
    );

    let in_file = File::open(input)?;
    let out_file = File::create(output)?;

    info!("Parsing data file");

    let report: BlockReport = serde_json::from_reader(BufReader::new(&in_file))?;

    info!("Parsing successful");

    let mut out = BufWriter::new(&out_file);

    write_header(&mut out, version, proto, true)?;

    let mut count = 0;
    let mut buf = vec![];
    // Go through blocks and write to mappings
    // file.
    for (block_name, block) in &report.blocks {
        for state in &block.states {
            // Write name
            buf.write_string(block_name.as_str())?;

            // Write properties
            let len = {
                if let Some(props) = state.properties.as_ref() {
                    props.props.len()
                } else {
                    0
                }
            };

            buf.write_u32::<LittleEndian>(len as u32)?;
            if let Some(props) = state.properties.as_ref() {
                for (name, value) in &props.props {
                    buf.write_string(name.as_str())?;
                    buf.write_string(value.as_str())?;
                }
            }

            // Write ID
            buf.write_u16::<LittleEndian>(state.id)?;

            count += 1;
        }
    }

    out.write_u32::<LittleEndian>(count)?;
    out.write_all(&buf)?;

    info!("Mappings file generated successfully");
    Ok(())
}

fn find_state_in_report(
    report: &BlockReport,
    name: &str,
    props: &HashMap<String, String>,
) -> Option<u16> {
    let block = report.blocks.get(name)?;

    let state = block.states.iter().find(|state| match &state.properties {
        None => props.is_empty(),
        Some(state_props) => props == &state_props.props,
    })?;

    Some(state.id)
}

fn write_header<W: Write>(
    out: &mut W,
    version: &str,
    proto: u32,
    native: bool,
) -> Result<(), Error> {
    out.write_all(b"FEATHER_BLOCK_DATA_FILE")?;
    out.write_string(version)?;
    out.write_u32::<LittleEndian>(proto)?;
    out.write_u8(native as u8)?;
    Ok(())
}
