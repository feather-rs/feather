use crate::entity::EntityMoveEvent;
use crate::network::PacketQueue;
use feather_core::network::packet::implementation::{
    PlayerLook, PlayerPosition, PlayerPositionAndLookServerbound,
};
use feather_core::Position;
use legion::entity::Entity;
use legion::query::Write;
use tonks::{PreparedWorld, Query, Trigger};

#[derive(Default, Resource)]
struct Buf(Vec<(Entity, Position)>);

/// Handles player movement packets.
#[system]
fn movement(
    queue: &PacketQueue,
    _query: &mut Query<Write<Position>>,
    world: &mut PreparedWorld,
    buf: &mut Buf,
    trigger: &mut Trigger<EntityMoveEvent>,
) {
    let positions = queue.received::<PlayerPosition>().map(|(player, packet)| {
        let old = *world.get_component::<Position>(player).unwrap();
        (
            player,
            position!(
                packet.x,
                packet.feet_y,
                packet.z,
                old.pitch,
                old.yaw,
                packet.on_ground
            ),
        )
    });

    let looks = queue.received::<PlayerLook>().map(|(player, packet)| {
        let mut old = *world.get_component::<Position>(player).unwrap();
        old.pitch = packet.pitch;
        old.yaw = packet.yaw;
        old.on_ground = packet.on_ground;
        (player, old)
    });

    let pos_looks = queue
        .received::<PlayerPositionAndLookServerbound>()
        .map(|(player, packet)| {
            (
                player,
                position!(
                    packet.x,
                    packet.feet_y,
                    packet.z,
                    packet.pitch,
                    packet.yaw,
                    packet.on_ground
                ),
            )
        });

    buf.0.extend(positions.chain(looks).chain(pos_looks));

    buf.0.drain(..).for_each(|(player, new_pos)| {
        *world.get_component_mut::<Position>(player).unwrap() = new_pos;
        trigger.trigger(EntityMoveEvent { entity: player });
    });
}
