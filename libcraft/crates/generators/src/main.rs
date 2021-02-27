mod common;
mod generators;

use common::{compress_and_write, load_block_report};
use generators::generate_block_states;

fn main() -> anyhow::Result<()> {
    println!("Generating raw block states");
    let block_report = load_block_report("blocks.json")?;
    let states = generate_block_states(&block_report)?;
    compress_and_write(states, "crates/blocks/assets/raw_block_states.bc.gz")?;

    Ok(())
}
