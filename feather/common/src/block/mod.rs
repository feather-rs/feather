use ecs::SystemExecutor;

use crate::Game;

pub mod placement;
pub mod util;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(placement::block_placement);
}

