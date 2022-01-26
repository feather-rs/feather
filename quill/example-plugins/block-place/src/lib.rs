//! Allows the user to place blocks.

use quill::{events::BlockPlacementEvent, Game, Plugin};

#[quill::plugin]
pub struct BlockPlace;

impl Plugin for BlockPlace {
    fn enable(_game: &mut quill::Game, setup: &mut quill::Setup<Self>) -> Self {
        setup.add_system(system);
        Self
    }

    fn disable(self, _game: &mut quill::Game) {}
}

fn system(_plugin: &mut BlockPlace, game: &mut Game) {
    for (_entity, _event) in game.query::<&BlockPlacementEvent>() {
        println!("A client has placed a block!");
    }
}
