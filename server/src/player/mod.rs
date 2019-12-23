//! Systems and components specific to player entities.

use crate::broadcasters::movement::LastKnownPositions;
use crate::chunk_logic::ChunkHolder;
use crate::entity;
use crate::entity::{CreationPacketCreator, EntityId, Name, SpawnPacketCreator};
use crate::io::NewClientInfo;
use crate::join::Joined;
use crate::network::Network;
use crate::state::State;
use crate::util::degrees_to_stops;
use feather_core::network::packet::implementation::{PlayerInfo, PlayerInfoAction, SpawnPlayer};
use feather_core::{Gamemode, Packet, Position};
use legion::entity::Entity;
use mojang_api::ProfileProperty;
use tonks::{EntityAccessor, PreparedWorld};
use uuid::Uuid;

/// Profile properties of a player.
#[derive(Debug, Clone)]
pub struct ProfileProperties(pub Vec<ProfileProperty>);

/// Event triggered when a player joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerJoinEvent {
    pub player: Entity,
}

/// Tag used to mark a player.
///
/// Note that this is a _tag_, not a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// Creates a new player from the given `NewClientInfo`.
///
/// This function also triggers the `PlayerJoinEvent` for this player.
pub fn create(state: &State, info: NewClientInfo) {
    entity::base(state, info.position)
        .with_tag(Player)
        .with_component(info.uuid)
        .with_component(Network {
            sender: info.sender,
            receiver: info.receiver,
        })
        .with_component(info.ip)
        .with_component(ProfileProperties(info.profile))
        .with_component(Name(info.username))
        .with_component(ChunkHolder::default())
        .with_component(Joined(false))
        .with_component(LastKnownPositions::default())
        .with_component(SpawnPacketCreator(&create_spawn_packet))
        .with_component(CreationPacketCreator(&create_initialization_packet))
        .with_exec(|_, scheduler, player| {
            scheduler.trigger(PlayerJoinEvent { player });
        })
        .build();
}

/// Function to create a `SpawnPlayer` packet to spawn the player.
fn create_spawn_packet(accessor: &EntityAccessor, world: &PreparedWorld) -> Box<dyn Packet> {
    let entity_id = accessor.get_component::<EntityId>(world).unwrap().0;
    let player_uuid = *accessor.get_component::<Uuid>(world).unwrap();
    let pos = *accessor.get_component::<Position>(world).unwrap();

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
fn create_initialization_packet(
    accessor: &EntityAccessor,
    world: &PreparedWorld,
) -> Box<dyn Packet> {
    let name = accessor.get_component::<Name>(world).unwrap();
    let props = accessor.get_component::<ProfileProperties>(world).unwrap();
    let uuid = *accessor.get_component::<Uuid>(world).unwrap();

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
