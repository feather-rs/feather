use super::InteractionHandler;

use feather_core::blocks::BlockKind;
use feather_core::network::packets::{OpenWindow, PlayerBlockPlacement};
use feather_core::text::TextRoot;
use feather_server_types::{Game, Network};
use fecs::{Entity, World};

pub struct CraftingTableInteraction;

impl InteractionHandler for CraftingTableInteraction {
    fn handle_interaction(
        &self,
        _game: &mut Game,
        world: &mut World,
        player: Entity,
        window_id: u8,
    ) {
        // Show the crafting table GUI for the player
        let open_window_packet = Box::new(OpenWindow {
            window_id: window_id,
            window_type: String::from("minecraft:crafting_table"),
            window_title: TextRoot::from("Crafting Table").into(),
            number_of_slots: 0,
            entity_id: None,
        });

        log::info!("Window ID: {}", window_id);

        let network = world.get::<Network>(player);

        network.send_boxed(open_window_packet);
    }

    fn handle_button_click(&self) {}
}
