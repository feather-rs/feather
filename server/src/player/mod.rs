//! Systems and components specific to player entities.

use crate::broadcasters::LastKnownPositions;
use crate::chunk_logic::ChunkHolder;
use crate::entity;
use crate::entity::{CreationPacketCreator, EntityId, Name, PreviousPosition, SpawnPacketCreator};
use crate::io::NewClientInfo;
use crate::network::Network;
use crate::p_inventory::EntityInventory;
use crate::util::degrees_to_stops;
use feather_core::network::packet::implementation::{PlayerInfo, PlayerInfoAction, SpawnPlayer};
use feather_core::{Gamemode, Packet, Position};
use fecs::{Entity, EntityRef, World};
use mojang_api::ProfileProperty;
use uuid::Uuid;

// pub mod chat;

pub const PLAYER_EYE_HEIGHT: f64 = 1.62;

/// Profile properties of a player.
#[derive(Debug, Clone)]
pub struct ProfileProperties(pub Vec<ProfileProperty>);

/// Zero-sized component used to mark players.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// Creates a new player from the given `NewClientInfo`.
///
/// This function also triggers the `PlayerJoinEvent` for this player.
pub fn create(world: &mut World, info: NewClientInfo) -> Entity {
    // TODO: blocked on https://github.com/TomGillen/legion/issues/36
    let entity = info.entity;
    world.add(entity, EntityId(entity::new_id())).unwrap();
    world.add(entity, info.position).unwrap();
    world.add(entity, PreviousPosition(info.position)).unwrap();
    world.add(entity, info.uuid).unwrap();
    world.add(entity, info.uuid).unwrap();
    world
        .add(
            entity,
            Network {
                tx: info.sender,
                rx: info.receiver,
            },
        )
        .unwrap();
    world.add(entity, info.ip).unwrap();
    world.add(entity, ProfileProperties(info.profile)).unwrap();
    world.add(entity, Name(info.username)).unwrap();
    world.add(entity, ChunkHolder::default()).unwrap();
    world.add(entity, LastKnownPositions::default()).unwrap();
    world
        .add(entity, SpawnPacketCreator(&create_spawn_packet))
        .unwrap();
    world
        .add(entity, CreationPacketCreator(&create_initialization_packet))
        .unwrap();
    world.add(entity, Gamemode::Creative).unwrap(); // TODO: proper gamemode handling
    world.add(entity, EntityInventory::default()).unwrap();
    world.add(entity, Player).unwrap();
    entity
}

/// Function to create a `SpawnPlayer` packet to spawn the player.
fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let entity_id = accessor.get::<EntityId>().0;
    let player_uuid = *accessor.get::<Uuid>();
    let pos = *accessor.get::<Position>();

    // TODO: metadata

    let packet = SpawnPlayer {
        entity_id,
        player_uuid,
        x: pos.x,
        y: pos.y,
        z: pos.z,
        yaw: degrees_to_stops(pos.yaw),
        pitch: degrees_to_stops(pos.pitch),
        metadata: Default::default(),
    };
    Box::new(packet)
}

/// Function to create a `PlayerInfo` packet to broadcast when the player joins.
fn create_initialization_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let name = accessor.get::<Name>();
    let props = accessor.get::<ProfileProperties>();
    let uuid = *accessor.get::<Uuid>();

    let props = props
        .0
        .iter()
        .map(|prop| {
            (
                prop.name.clone(),
                prop.value.clone(),
                prop.signature.clone(),
            )
        })
        .collect::<Vec<_>>();

    let display_name = json!({
        "text": name.0
    })
    .to_string();

    let action =
        PlayerInfoAction::AddPlayer(name.0.clone(), props, Gamemode::Creative, 50, display_name);

    let packet = PlayerInfo { action, uuid };
    Box::new(packet)
}
