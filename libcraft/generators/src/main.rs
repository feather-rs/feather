mod common;
mod generators;

use common::load_block_report;
use generators::{generate_block_properties, generate_block_states};

fn main() -> anyhow::Result<()> {
    let block_report = load_block_report("blocks.json")?;
    println!("Generating raw block states");
    generate_block_states(&block_report, "crates/blocks/assets/raw_block_states.bc.gz")?;
    println!("Generating raw block properties");
    generate_block_properties(
        &block_report,
        "crates/blocks/assets/raw_block_properties.bc.gz",
    )?;

    Ok(())
}
