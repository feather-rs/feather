use base::{anvil::level::LevelData, Gamemode, Setup, State};
use common::{entity::player::Player, Name};
use ecs::{Added, SysResult};

use crate::{entity::NetworkId, session::Session, Server};

pub fn setup(setup: &mut Setup) {
    setup.system(join_players);
}

/// Joins a player after they join.
fn join_players(state: &mut State) -> SysResult {
    for (_, name, session, network_id, gamemode) in
        &mut state
            .ecs
            .query::<(Added<Player>, &Name, &Session, &NetworkId, &Gamemode)>()
    {
        let server = state.resource::<Server>()?;
        let view_distance = server.config.server.view_distance;
        let max_players = server.config.server.max_players;
        let level = state.resource::<LevelData>()?;

        session.join(
            network_id,
            *gamemode,
            u64::from_ne_bytes(level.seed.to_ne_bytes()),
            max_players as u32,
            view_distance,
            level.generator_type(),
        );

        log::trace!("Joined player {}", name.0);
    }
    Ok(())
}
