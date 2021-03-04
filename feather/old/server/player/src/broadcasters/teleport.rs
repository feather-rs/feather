use feather_core::network::packets::PlayerPositionAndLookClientbound;
use feather_core::util::Position;
use feather_server_types::{BumpVec, Game, Network, Teleported};
use fecs::{component, IntoQuery, Read, World};
use std::sync::atomic::{AtomicI32, Ordering};

/// TODO: how are we supposed to handle this?
static TELEPORT_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

/// System which polls for players with the `Teleported` component
/// and notifies them of their new position.
#[fecs::system]
pub fn send_teleported(world: &mut World, game: &mut Game) {
    let mut to_delete = BumpVec::new_in(game.bump());
    for (entity, (network, pos)) in <(Read<Network>, Read<Position>)>::query()
        .filter(component::<Teleported>())
        .iter_entities(world.inner())
    {
        let teleport_id = TELEPORT_ID_COUNTER.fetch_add(1, Ordering::AcqRel);
        let packet = PlayerPositionAndLookClientbound {
            x: pos.x,
            y: pos.y,
            z: pos.z,
            yaw: pos.yaw,
            pitch: pos.pitch,
            flags: 0,
            teleport_id, // todo: properly handle the teleport ID
        };
        network.send(packet);
        to_delete.push(entity);
    }

    for entity in to_delete {
        let _ = world.remove::<Teleported>(entity);
    }
}
