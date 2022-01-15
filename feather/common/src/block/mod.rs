use ecs::SystemExecutor;

use crate::Game;

pub mod placement;
pub mod update;
pub mod util;
pub mod wall;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(placement::block_placement);
    systems.add_system(update::block_update);
}
