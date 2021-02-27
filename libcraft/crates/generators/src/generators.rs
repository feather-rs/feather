use libcraft_blocks_data::{BlockReport, RawBlockState};

pub fn generate_block_states(block_report: &BlockReport) -> anyhow::Result<Vec<RawBlockState>> {
    let mut raw_block_states = Vec::new();

    println!(
        "Processing {} block report entries",
        block_report.blocks.len()
    );
    for (_, entry) in &block_report.blocks {
        for state in &entry.states {
            raw_block_states.push(state.to_raw_state());
        }
    }

    raw_block_states.sort_unstable_by_key(|state| state.id);

    Ok(raw_block_states)
}
