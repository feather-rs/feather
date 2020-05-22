use super::InteractionHandler;

use feather_core::blocks::BlockKind;
use feather_core::network::packets::PlayerBlockPlacement;
use feather_server_types::Game;
use fecs::{Entity, World};

pub struct CraftingTableInteraction;

impl InteractionHandler for CraftingTableInteraction {
    fn handle_interaction(
        &self,
        game: &mut Game,
        world: &mut World,
        player: Entity,
        target_block_kind: BlockKind,
        packet: PlayerBlockPlacement,
    ) {
    }
    fn handle_button_click(&self) {}
}
