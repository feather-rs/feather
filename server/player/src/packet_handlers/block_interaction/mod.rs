mod chest;
mod crafting_table;

pub use chest::ChestInteraction;
pub use crafting_table::CraftingTableInteraction;

use feather_core::util::BlockPosition;
use feather_server_types::Game;
use fecs::{Entity, World};

pub trait InteractionHandler: Send + Sync {
    /// Called whenever a player right clicks on the block
    fn handle_interaction(
        &self,
        game: &mut Game,
        world: &mut World,
        pos: BlockPosition,
        player: Entity,
        window_id: u8,
    );
}
