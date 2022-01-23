//! A plugin to demonstrate getting and setting blocks in the world.

use quill::{entities::Player, BlockState, Game, Plugin, Position};

#[quill::plugin]
pub struct BlockAccess;

impl Plugin for BlockAccess {
    fn enable(_game: &mut quill::Game, setup: &mut quill::Setup<Self>) -> Self {
        setup.add_system(system);
        Self
    }

    fn disable(self, _game: &mut quill::Game) {}
}

fn system(_plugin: &mut BlockAccess, game: &mut Game) {
    // Set the blocks each player is standing on
    // to bedrock.
    for (_entity, (_, pos)) in game.query::<(&Player, &Position)>() {
        let block_pos = pos.block();

        game.set_block(block_pos, BlockState::from_id(33).unwrap())
            .ok();
    }
}
