//! Network interface for working with entities.

use base::{Position, Setup, State};
use common::entity::player;
use ecs::{EntityBuilder, EntityRef, SysResult};
use protocol::{packets::server::SpawnPlayer, ServerPlayPacket};
use uuid::Uuid;

/// The network ID of an entity. This is the "entity_id" field
/// for many packets.
///
/// Unique across all entities.
#[derive(Debug)]
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
            self.head
        }))
    }

    fn dealloc(&mut self, id: &NetworkId) {
        self.free.push(id.0);
    }
}

pub fn setup(setup: &mut Setup) {
    setup.resource(NetworkIdAllocator::default());
}

/// Adds base components shared between all entities.\
/// Currently, this includes `NetworkId`.
fn build(s: &mut State, builder: &mut EntityBuilder) -> SysResult {
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
pub fn build_player(s: &mut State, builder: &mut EntityBuilder) -> SysResult {
    player::build(builder);
    build(s, builder)?;
    builder.add(SpawnPacket::new(player_spawn_packet));
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
