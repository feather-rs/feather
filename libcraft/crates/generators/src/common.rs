use std::fs::{read_to_string, write};
use std::io::Write;

use libcraft_blocks::data::BlockReport;
use libcraft_blocks::BlockKind;

use anyhow::{anyhow, Context, Result};
use flate2::Compression;
use serde::Serialize;

pub fn load_block_report(path: &str) -> Result<BlockReport> {
    println!("Reading BlockReport from blocks.json");
    let block_report = read_to_string(path).context("blocks report `blocks.json` not found")?;
    serde_json::from_str::<BlockReport>(&block_report).map_err(|err| err.into())
}

/// Writes data to file provided in compressed binary format (.bc.gz)
pub fn compress_and_write<T: Serialize>(data: Vec<T>, path: &str) -> Result<()> {
    println!("Writing {} entries to {}", data.len(), path);
    let encoded = bincode::serialize(&data)?;

    let mut writer = flate2::write::GzEncoder::new(Vec::new(), Compression::best());
    writer.write_all(&encoded)?;
    write(
        [env!("CARGO_MANIFEST_DIR"), "/../../", path].concat(),
        &writer.finish()?,
    )?;

    Ok(())
}

pub fn state_name_to_block_kind(name: &str) -> Result<BlockKind> {
    name.split(':')
        .last()
        .map(BlockKind::from_name)
        .flatten()
        .ok_or_else(|| anyhow!("Could not convert state name to BlockKind"))
}
