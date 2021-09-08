use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::events::TimeUpdateEvent;

use crate::Server;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(time_change)
        .add_system(time_tick);
}

fn time_change(game: &mut Game, server: &mut Server) -> SysResult {
    for _ in game.ecs.query::<(&TimeUpdateEvent,)>().iter() {
        server.broadcast_with(|client| {
            client.send_time(&game.world.time);
        });
    }
    Ok(())
}

fn time_tick(game: &mut Game, _server: &mut Server) -> SysResult {
    game.world.time.increment();
    Ok(())
}
