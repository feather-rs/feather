mod biomes;
mod block_kinds;
mod block_states;
mod dimensions;
mod entities;
mod inventory;
mod items;
mod simplified_block;

pub fn generate_all() -> anyhow::Result<()> {
    items::generate();
    block_kinds::generate();
    block_states::generate()?;
    simplified_block::generate();
    biomes::generate()?;
    dimensions::generate()?;
    inventory::generate();
    entities::generate();
    Ok(())
}
