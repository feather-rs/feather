use base::{EntityKind, Position};
use quill_common::components::OnGround;
use uuid::Uuid;
use vane::{EntityBuilder, EntityRef, SysResult};

use crate::{Client, NetworkId};

/// Component that sends the spawn packet for an entity
/// using its components.
pub struct SpawnPacketSender(fn(&EntityRef, &mut Client) -> SysResult);

impl SpawnPacketSender {
    pub fn send(&self, entity: &EntityRef, client: &mut Client) -> SysResult {
        (self.0)(entity, client)
    }
}

/// Stores the [`Position`] of an entity on
/// the previous tick. Used to determine
/// when to send movement updates.
#[derive(Copy, Clone, Debug)]
pub struct PreviousPosition(pub Position);
/// Stores the [`OnGround`] status of an entity on
/// the previous tick. Used to determine
/// what movement packet to send.
#[derive(Copy, Clone, Debug)]
pub struct PreviousOnGround(pub OnGround);

pub fn add_entity_components(builder: &mut EntityBuilder, kind: EntityKind) {
    if !builder.has::<NetworkId>() {
        builder.add(NetworkId::new());
    }

    // can't panic because this is only called after both position and onground is added to all entities.
    // Position is added in the caller of this function and on_ground is added in the
    // build default function. All entity builder functions call the build default function.
    let prev_position = builder.get::<Position>().unwrap();
    let on_ground = builder.get::<OnGround>().unwrap();

    builder
        .add(PreviousPosition(prev_position))
        .add(PreviousOnGround(on_ground));
    add_spawn_packet(builder, kind);
}

fn add_spawn_packet(builder: &mut EntityBuilder, kind: EntityKind) {
    // TODO: object entities spawned with Spawn Entity
    // (minecarts, items, ...)
    let spawn_packet = match kind {
        EntityKind::Player => spawn_player,
        _ => spawn_living_entity,
    };
    builder.add(SpawnPacketSender(spawn_packet));
}

fn spawn_player(entity: &EntityRef, client: &mut Client) -> SysResult {
    let network_id = *entity.get::<NetworkId>()?;
    let uuid = *entity.get::<Uuid>()?;
    let pos = *entity.get::<Position>()?;

    client.send_player(network_id, uuid, pos);
    Ok(())
}

fn spawn_living_entity(entity: &EntityRef, client: &mut Client) -> SysResult {
    let network_id = *entity.get::<NetworkId>()?;
    let uuid = *entity.get::<Uuid>()?;
    let pos = *entity.get::<Position>()?;
    let kind = *entity.get::<EntityKind>()?;

    client.send_living_entity(network_id, uuid, pos, kind);
    Ok(())
}
