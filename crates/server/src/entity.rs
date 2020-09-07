//! Network interface for working with entities.

use base::{Position, Setup, State};
use common::{entity::player, Name};
use ecs::{Entity, EntityBuilder, EntityRef, SysResult};
use protocol::{packets::server::SpawnPlayer, ServerPlayPacket};
use uuid::Uuid;

use crate::network::{Network, NewPlayer};

/// The network ID of an entity. This is the "entity_id" field
/// for many packets.
///
/// Unique across all entities.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NetworkId(pub i32);

/// Allocates `NetworkId`s.
#[derive(Default)]
struct NetworkIdAllocator {
    free: Vec<i32>,
    head: i32,
}

impl NetworkIdAllocator {
    fn new() -> Self {
        Self::default()
    }

    fn alloc(&mut self) -> NetworkId {
        NetworkId(self.free.pop().unwrap_or_else(|| {
            self.head = self.head.wrapping_add(1);
            self.head.wrapping_sub(1)
        }))
    }

    fn dealloc(&mut self, id: &NetworkId) {
        self.free.push(id.0);
    }
}

pub fn setup(setup: &mut Setup) {
    setup
        .resource(NetworkIdAllocator::default())
        .despawn_callback(free_network_id);
}

/// Callback to free an entity's network ID when
/// it is despawned.
fn free_network_id(s: &mut State, entity: Entity) -> SysResult {
    if let Ok(network_id) = s.ecs.get::<NetworkId>(entity) {
        s.resource_mut::<NetworkIdAllocator>()?.dealloc(&network_id);
    }
    Ok(())
}

/// Adds base components shared between all entities.\
/// Currently, this includes `NetworkId`.
fn build(s: &State, builder: &mut EntityBuilder) -> SysResult {
    let mut allocator = s.resource_mut::<NetworkIdAllocator>()?;
    builder.add(allocator.alloc());
    Ok(())
}

type SpawnPacketFn = fn(entity: EntityRef) -> SysResult<ServerPlayPacket>;
/// Component which stores the packet used to send
/// an entity to the client.
///
/// This is a "lazily-evaluated" component; that is, it stores
/// a function which computes the packet given the
/// entity's other components.
pub struct SpawnPacket(SpawnPacketFn);

impl SpawnPacket {
    pub fn new(f: SpawnPacketFn) -> Self {
        SpawnPacket(f)
    }

    pub fn create(&self, entity: EntityRef) -> SysResult<ServerPlayPacket> {
        (self.0)(entity)
    }
}

/// Adds components for a player entity.
///
/// This will add all components added by `common::entity::player::build()`
/// in addition to those needed for the server.
pub fn build_player(s: &State, builder: &mut EntityBuilder, player: NewPlayer) -> SysResult {
    player::build(builder);
    build(s, builder)?;
    builder
        .add(SpawnPacket::new(player_spawn_packet)) // TODO
        .add(player.uuid)
        .add(Network::new(player.worker))
        .add(player.addr)
        .add(Name(player.username));
    Ok(())
}

fn player_spawn_packet(entity: EntityRef) -> SysResult<ServerPlayPacket> {
    let pos = entity.get::<Position>()?;
    Ok(ServerPlayPacket::SpawnPlayer(SpawnPlayer {
        entity_id: entity.get::<NetworkId>()?.0,
        player_uuid: *entity.get::<Uuid>()?,
        x: pos.x,
        y: pos.y,
        z: pos.z,
        yaw: pos.yaw,
        pitch: pos.pitch,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_id_contiguous() {
        let mut a = NetworkIdAllocator::new();
        for x in 0..1000 {
            assert_eq!(a.alloc().0, x);
        }

        a.dealloc(&NetworkId(995));
        assert_eq!(a.alloc().0, 995);
        assert_eq!(a.alloc().0, 1000);
    }

    #[test]
    fn network_id_free_callback() {
        let mut a = NetworkIdAllocator::new();
        let id = a.alloc();
        assert_eq!(id.0, 0);
        let (mut state, _) = Setup::new()
            .resource(a)
            .despawn_callback(free_network_id)
            .build();

        let e1 = state.ecs.spawn((id, "entity1"));
        let e2 = state.ecs.spawn(("entity2",));

        state.despawn(e1).unwrap();
        assert_eq!(
            state
                .resource_mut::<NetworkIdAllocator>()
                .unwrap()
                .alloc()
                .0,
            0
        );
        state.despawn(e2).unwrap();
        assert_eq!(
            state
                .resource_mut::<NetworkIdAllocator>()
                .unwrap()
                .alloc()
                .0,
            1
        );
    }
}
