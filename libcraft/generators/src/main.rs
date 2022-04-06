mod common;
mod generators;

use common::load_block_report;
use generators::{generate_block_properties, generate_block_states};

fn main() -> anyhow::Result<()> {
    let block_report = load_block_report("generated/reports/blocks.json")?;
    println!("Generating raw block states");
    generate_block_states(&block_report, "libcraft/blocks/assets/raw_block_states.bc.gz")?;
    println!("Generating raw block properties");
    generate_block_properties(
        &block_report,
        "libcraft/blocks/assets/raw_block_properties.bc.gz",
    )?;

    Ok(())
}
