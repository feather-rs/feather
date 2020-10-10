#![forbid(unsafe_code)]

extern crate nalgebra_glm as glm;

mod broadcasters;
mod chat;
mod death;
mod join;
mod packet_handlers;
mod view;

use feather_core::inventory::{Area, Inventory, SlotIndex, Window};
use feather_core::network::packets::{PlayerInfo, PlayerInfoAction, SpawnPlayer};
use feather_core::network::Packet;
use feather_core::text::Text;
use feather_core::util::{Gamemode, Position};
use feather_server_network::NewClientInfo;
use feather_server_types::{
    BlocksFallen, CanBreak, CanInstaBreak, CanRespawn, CanTakeDamage, ChunkHolder,
    CreationPacketCreator, EntitySpawnEvent, Game, GamemodeUpdateEvent, Health, HealthUpdateEvent,
    HeldItem, InventoryUpdateEvent, LastKnownPositions, MaxHealth, MessageReceiver, Name, Network,
    NetworkId, OpenWindowCount, Player, PlayerJoinEvent, PlayerPreJoinEvent, PreviousPosition,
    PreviousVelocity, ProfileProperties, SpawnPacketCreator, Uuid, Velocity,
};
use feather_server_util::degrees_to_stops;
use fecs::{Entity, EntityRef, World};

pub use broadcasters::*;
pub use chat::*;
pub use death::*;
pub use join::*;
pub use packet_handlers::*;
use std::sync::atomic::Ordering;
pub use view::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemTimedUse {
    pub tick_start: u64,
}

/// Creates a new player from the given `NewClientInfo`.
///
/// This function also triggers events for the player join.
pub fn create(game: &mut Game, world: &mut World, info: NewClientInfo) -> Entity {
    // TODO: blocked on https://github.com/TomGillen/legion/issues/36
    let entity = info.entity;
    world.add(entity, NetworkId(entity::new_id())).unwrap();
    world.add(entity, info.position).unwrap();
    world.add(entity, PreviousPosition::default()).unwrap();
    world.add(entity, Velocity::default()).unwrap();
    world.add(entity, PreviousVelocity::default()).unwrap();
    world.add(entity, info.uuid).unwrap();
    world
        .add(
            entity,
            Network {
                tx: info.sender,
                rx: info.receiver.into(),
            },
        )
        .unwrap();
    world.add(entity, info.ip).unwrap();
    world.add(entity, ProfileProperties(info.profile)).unwrap();
    world.add(entity, Name(info.username)).unwrap();
    world.add(entity, OpenWindowCount::default()).unwrap();
    world.add(entity, ChunkHolder::default()).unwrap();
    world.add(entity, LastKnownPositions::default()).unwrap();
    world
        .add(entity, SpawnPacketCreator(&create_spawn_packet))
        .unwrap();
    world
        .add(entity, CreationPacketCreator(&create_initialization_packet))
        .unwrap();

    let gamemode = Gamemode::from_id(info.data.gamemode as u8);
    add_gamemode_comps(world, gamemode, entity);

    let items = info
        .data
        .inventory
        .iter()
        .map(|slot| (slot.convert_index().unwrap_or_default(), slot.into()));

    let (window, slots) = {
        let inventory = Inventory::player();
        let window = Window::player(entity);
        world.add(entity, inventory).unwrap();

        let accessor = window.accessor(world).unwrap();
        items.for_each(|(index, item)| {
            let _ = accessor.set_item_at(index, item);
        });

        let window2 = window.clone();
        let slots = info.data.inventory.iter().map(move |slot| {
            window2
                .convert_network(slot.convert_index().unwrap_or_default())
                .map(SlotIndex::from)
                .unwrap_or(SlotIndex {
                    area: Area::Hotbar,
                    slot: 0,
                })
        });

        drop(accessor);
        (window, slots)
    };
    world.add(entity, window).unwrap();
    world
        .add(entity, HeldItem(info.data.held_item as usize))
        .unwrap();

    world.add(entity, MessageReceiver::default()).unwrap();

    world.add(entity, Player).unwrap();

    world.add(entity, CanRespawn).unwrap();
    world.add(entity, MaxHealth(20)).unwrap();
    world
        .add(entity, Health(info.data.animal.health as u32))
        .unwrap();
    world.add(entity, BlocksFallen::default()).unwrap();

    game.player_count.fetch_add(1, Ordering::SeqCst);
    game.handle(world, EntitySpawnEvent { entity });
    game.handle(world, PlayerPreJoinEvent { player: entity });
    game.handle(world, PlayerJoinEvent { player: entity });
    game.handle(
        world,
        InventoryUpdateEvent {
            slots: slots.collect(),
            entity,
        },
    );
    game.handle(
        world,
        HealthUpdateEvent {
            old: 0,
            new: info.data.animal.health as u32,
            entity,
        },
    );

    entity
}

fn add_gamemode_comps(world: &mut World, gamemode: Gamemode, entity: Entity) {
    world.add(entity, gamemode).unwrap();

    // Remove old gamemode comps
    let _ = world.remove::<CanTakeDamage>(entity);
    let _ = world.remove::<CanInstaBreak>(entity);
    let _ = world.remove::<CanBreak>(entity);

    match gamemode {
        Gamemode::Survival | Gamemode::Adventure => world.add(entity, CanTakeDamage).unwrap(),
        Gamemode::Creative => world.add(entity, CanInstaBreak).unwrap(),
        _ => (),
    }

    if gamemode == Gamemode::Survival || gamemode == Gamemode::Creative {
        world.add(entity, CanBreak).unwrap();
    }
}

/// When a player's gamemode is updated, updates their capability
/// marker components (`CanBreak`, `CanTakeDamage`, etc)
#[fecs::event_handler]
pub fn on_gamemode_update_update_capabilities(event: &GamemodeUpdateEvent, world: &mut World) {
    if world.is_alive(event.player) {
        add_gamemode_comps(world, event.new, event.player);
    }
}

/// Function to create a `SpawnPlayer` packet to spawn the player.
fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let entity_id = accessor.get::<NetworkId>().0;
    let player_uuid = *accessor.get::<Uuid>();
    let pos = *accessor.get::<Position>();

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

    let display_name = Text::of(name.0.clone()).into();

    let action =
        PlayerInfoAction::AddPlayer(name.0.clone(), props, Gamemode::Creative, 50, display_name);

    let packet = PlayerInfo { action, uuid };
    Box::new(packet)
}
