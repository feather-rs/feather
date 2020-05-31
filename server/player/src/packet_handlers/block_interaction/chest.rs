use super::InteractionHandler;
use feather_core::inventory::Window;
use feather_core::network::packets::OpenWindow;
use feather_core::text::TextRoot;
use feather_core::util::BlockPosition;
use feather_server_types::{Game, Network};
use fecs::{Entity, World};

pub struct ChestInteraction;

impl InteractionHandler for ChestInteraction {
    fn handle_interaction(
        &self,
        game: &mut Game,
        world: &mut World,
        pos: BlockPosition,
        player: Entity,
        window_id: u8,
    ) {
        // Open chest window and set the player's window
        if let Some(chest_entity) = game.block_entities.get(&pos).copied() {
            let packet = OpenWindow {
                window_id,
                window_type: String::from("minecraft:chest"),
                window_title: TextRoot::from("Chest").into(),
                number_of_slots: 27,
                entity_id: None,
            };
            world.get::<Network>(player).send(packet);

            world
                .add(player, Window::chest(player, chest_entity))
                .unwrap();
        }
    }
}
