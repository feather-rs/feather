mod crafting_table;

pub use crafting_table::CraftingTableInteraction;

use feather_server_types::Game;
use fecs::{Entity, World};

pub trait InteractionHandler: Send + Sync {
    /// Called whenever a player right clicks on the block
    fn handle_interaction(&self, game: &mut Game, world: &mut World, player: Entity, window_id: u8);
    /// Called whenever a player clicks on a button within the GUI of the block
    fn handle_button_click(&self);
}
