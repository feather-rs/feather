mod block_states;
mod blocks;
mod entities;
mod inventory;
mod items;
mod simplified_block;

pub fn generate_all() {
    items::generate();
    blocks::generate();
    simplified_block::generate();
    // block_states::generate();
    inventory::generate();
    entities::generate();
}
