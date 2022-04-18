use crate::common::{compress_and_write, load_block_report, state_name_to_block_kind};
use libcraft_blocks::data::BlockReport;

pub fn generate() -> anyhow::Result<()> {
    let block_report = load_block_report("generated/reports/blocks.json")?;
    println!("Generating raw block states");
    generate_block_states(&block_report, "libcraft/assets/raw_block_states.bc.gz")?;
    println!("Generating raw block properties");
    generate_block_properties(&block_report, "libcraft/assets/raw_block_properties.bc.gz")?;
    Ok(())
}

pub fn generate_block_states(block_report: &BlockReport, path: &str) -> anyhow::Result<()> {
    let mut raw_block_states = Vec::new();

    for (name, entry) in &block_report.blocks {
        let kind = state_name_to_block_kind(name)?;
        for state in &entry.states {
            raw_block_states.push(state.to_raw_state(kind));
        }
    }

    raw_block_states.sort_unstable_by_key(|state| state.id);

    compress_and_write(raw_block_states, path)
}

pub fn generate_block_properties(block_report: &BlockReport, path: &str) -> anyhow::Result<()> {
    let mut raw_block_properties = Vec::new();

    for (name, entry) in &block_report.blocks {
        let kind = state_name_to_block_kind(name)?;
        raw_block_properties.push(entry.to_raw_properties(kind))
    }

    raw_block_properties.sort_unstable_by_key(|properties| properties.kind);

    compress_and_write(raw_block_properties, path)
}
