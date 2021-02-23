use std::{fs, io::Write};

use anyhow::Context;
use flate2::Compression;
use libcraft_blocks_data::BlockReport;

pub fn generate_block_states() -> anyhow::Result<()> {
    let blocks_report =
        fs::read_to_string("blocks.json").context("blocks report `blocks.json` not found")?;
    let blocks_report: BlockReport = serde_json::from_str(&blocks_report)?;

    let mut raw_block_states = Vec::new();

    for (name, entry) in blocks_report.blocks {
        println!("Processing {}", name);
        for state in &entry.states {
            raw_block_states.push(state.to_raw_state());
        }
    }

    raw_block_states.sort_unstable_by_key(|state| state.id);

    let encoded = bincode::serialize(&raw_block_states)?;

    let mut writer = flate2::write::GzEncoder::new(Vec::new(), Compression::best());
    writer.write_all(&encoded)?;
    fs::write(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../blocks/assets/raw_block_states.bc.gz"
        ),
        &writer.finish()?,
    )?;

    Ok(())
}
