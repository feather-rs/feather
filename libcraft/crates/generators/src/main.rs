mod block_state;

use block_state::generate_block_states;

fn main() -> anyhow::Result<()> {
    println!("Generating raw block states");
    generate_block_states()
}
