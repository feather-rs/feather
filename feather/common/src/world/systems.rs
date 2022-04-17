use quill::SysResult;
use vane::SystemExecutor;

use crate::Game;

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.add_system(update_world_chunks);
}

fn update_world_chunks(game: &mut Game) -> SysResult {
    let mut events = Vec::new();
    for mut world in game.worlds_mut() {
        events.extend(world.update_chunks());
    }

    for event in events {
        game.ecs.insert_event(event);
    }

    Ok(())
}
