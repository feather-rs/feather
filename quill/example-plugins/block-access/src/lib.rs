//! A plugin to demonstrate getting and setting blocks in the world.

use quill::components::{EntityDimension, EntityWorld};
use quill::{entities::Player, Game, Plugin, Position};

quill::plugin!(BlockAccess);

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
    for (_entity, (_, _pos, _world, _dimension)) in
        game.query::<(&Player, &Position, &EntityWorld, &EntityDimension)>()
    {
        todo!()
    }
}
