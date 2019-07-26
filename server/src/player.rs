//! This module provides systems and components
//! relating to players, including player movement
//! and inventory handling.

use std::ops::{Deref, DerefMut};

use hashbrown::HashSet;
use rayon::prelude::*;
use shrev::EventChannel;
use specs::storage::BTreeStorage;
use specs::{
    Component, Entities, Entity, Join, LazyUpdate, ParJoin, Read, ReadStorage, ReaderId, System,
    World, WorldExt, WriteStorage,
};

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    ChunkData, PlayerInfo, PlayerInfoAction, PlayerLook, PlayerPosition,
    PlayerPositionAndLookServerbound, SpawnPlayer,
};
use feather_core::network::packet::{Packet, PacketType};
use feather_core::world::chunk::Chunk;
use feather_core::world::{ChunkMap, ChunkPosition, Position};
use feather_core::Gamemode;

use crate::chunkclient::{load_chunk, ChunkLoadEvent, ChunkWorkerHandle};
use crate::entity::{broadcast_entity_movement, EntityComponent, PlayerComponent};
use crate::joinhandler::{PlayerJoinEvent, SPAWN_POSITION};
use crate::network::{send_packet_to_player, NetworkComponent, PacketQueue, PlayerPreJoinEvent};
use feather_core::entitymeta::{EntityMetadata, MetaEntry};

/// System for handling player movement
/// packets.
pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, EntityComponent>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, PacketQueue>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ecomps, pcomps, packet_queue, netcomps, entities, _) = data;

        // Take movement packets
        let mut packets = vec![];
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPosition));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPositionAndLookServerbound));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerLook));

        // Handle movement packets
        for (player, packet) in packets {
            let ecomp = ecomps.get(player).unwrap();

            // Get position using packet and old position
            let (new_pos, has_moved, has_looked) = new_pos_from_packet(ecomp.position, packet);

            // Broadcast position update
            broadcast_entity_movement(
                player,
                ecomp.position,
                new_pos,
                has_moved,
                has_looked,
                &netcomps,
                &pcomps,
                &entities,
            );

            // Set new position
            ecomps.get_mut(player).unwrap().position = new_pos;
        }
    }
}

fn new_pos_from_packet(old_pos: Position, packet: Box<Packet>) -> (Position, bool, bool) {
    let mut has_looked = false;
    let mut has_moved = false;

    let pos = match packet.ty() {
        PacketType::PlayerPosition => {
            has_moved = true;
            let packet = cast_packet::<PlayerPosition>(&packet);

            Position::new(
                packet.x,
                packet.feet_y,
                packet.z,
                old_pos.pitch,
                old_pos.yaw,
            )
        }
        PacketType::PlayerLook => {
            has_looked = true;
            let packet = cast_packet::<PlayerLook>(&packet);

            Position::new(old_pos.x, old_pos.y, old_pos.z, packet.pitch, packet.yaw)
        }
        PacketType::PlayerPositionAndLookServerbound => {
            has_moved = true;
            has_looked = true;
            let packet = cast_packet::<PlayerPositionAndLookServerbound>(&packet);

            Position::new(packet.x, packet.feet_y, packet.z, packet.pitch, packet.yaw)
        }
        _ => panic!(),
    };

    (pos, has_moved, has_looked)
}

/// Component storing what chunks are pending
/// to send to a player.
#[derive(Clone, Debug)]
pub struct ChunkPendingComponent {
    pub pending: HashSet<ChunkPosition>,
}

impl Deref for ChunkPendingComponent {
    type Target = HashSet<ChunkPosition>;

    fn deref(&self) -> &Self::Target {
        &self.pending
    }
}

impl DerefMut for ChunkPendingComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pending
    }
}

impl Component for ChunkPendingComponent {
    type Storage = BTreeStorage<Self>;
}

/// System for initializing the necessary components
/// when a player joins.
pub struct PlayerInitSystem {
    join_event_reader: Option<ReaderId<PlayerPreJoinEvent>>,
}

impl PlayerInitSystem {
    pub fn new() -> Self {
        Self {
            join_event_reader: None,
        }
    }
}

impl<'a> System<'a> for PlayerInitSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerPreJoinEvent>>,
        WriteStorage<'a, PlayerComponent>,
        WriteStorage<'a, EntityComponent>,
        WriteStorage<'a, ChunkPendingComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, mut player_comps, mut entity_comps, mut chunk_pending_comps) = data;

        // Run through events
        for event in events.read(&mut self.join_event_reader.as_mut().unwrap()) {
            let player_comp = PlayerComponent {
                profile_properties: event.profile_properties.clone(),
                gamemode: Gamemode::Creative,
            };
            player_comps.insert(event.player, player_comp);

            let entity_comp = EntityComponent {
                uuid: event.uuid.clone(),
                display_name: event.username.clone(),
                position: SPAWN_POSITION,
                on_ground: true,
            };
            entity_comps.insert(event.player, entity_comp);

            let chunk_pending_comp = ChunkPendingComponent {
                pending: HashSet::new(),
            };
            chunk_pending_comps.insert(event.player, chunk_pending_comp);
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::SystemData;
        Self::SystemData::setup(world);

        self.join_event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerPreJoinEvent>>()
                .register_reader(),
        );
    }
}

/// System for sending chunks to players once they're loaded.
///
/// This system listens to `ChunkLoadEvent`s.
pub struct ChunkSendSystem {
    load_event_reader: Option<ReaderId<ChunkLoadEvent>>,
}

impl ChunkSendSystem {
    pub fn new() -> Self {
        Self {
            load_event_reader: None,
        }
    }
}

impl<'a> System<'a> for ChunkSendSystem {
    type SystemData = (
        WriteStorage<'a, ChunkPendingComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, ChunkMap>,
        Read<'a, EventChannel<ChunkLoadEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut pendings, netcomps, chunk_map, load_events) = data;

        for event in load_events.read(&mut self.load_event_reader.as_mut().unwrap()) {
            // TODO perhaps this is slightly inefficient?
            (&netcomps, &mut pendings)
                .par_join()
                .for_each(|(net, pending)| {
                    if pending.contains(&event.pos) {
                        // It's safe to unwrap the chunk value now,
                        // because we know it's been loaded.
                        let chunk = chunk_map.chunk_at(event.pos).unwrap();
                        send_chunk_data(chunk, net);

                        pending.remove(&event.pos);
                    }
                });
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::SystemData;
        Self::SystemData::setup(world);
        self.load_event_reader = Some(
            world
                .fetch_mut::<EventChannel<ChunkLoadEvent>>()
                .register_reader(),
        );
    }
}

/// System for broadcasting when a player joins
/// the game. Also spawns other players to
/// the player's client.
pub struct JoinBroadcastSystem {
    reader: Option<ReaderId<PlayerJoinEvent>>,
}

impl JoinBroadcastSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for JoinBroadcastSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerJoinEvent>>,
        ReadStorage<'a, EntityComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (join_events, entity_comps, player_comps, net_comps, entities) = data;

        for event in join_events.read(&mut self.reader.as_mut().unwrap()) {
            // Broadcast join
            let entity_comp = entity_comps.get(event.player).unwrap();
            let player_comp = player_comps.get(event.player).unwrap();

            let (player_info, spawn_player) =
                get_player_initialization_packets(entity_comp, player_comp, event.player);

            for (player, net) in (&entities, &net_comps).join() {
                // Send player info to the player who joined
                // so they can see themselves in the tablist,
                // but don't send spawn player.
                send_packet_to_player(net, player_info.clone());
                if player != event.player {
                    send_packet_to_player(net, spawn_player.clone());
                }
            }

            let net_comp = net_comps.get(event.player).unwrap();

            // Send existing players to new player
            for (entity_comp, player_comp, entity) in
                (&entity_comps, &player_comps, &entities).join()
            {
                if entity != event.player {
                    let (player_info, spawn_player) =
                        get_player_initialization_packets(entity_comp, player_comp, entity);
                    send_packet_to_player(net_comp, player_info);
                    send_packet_to_player(net_comp, spawn_player);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::SystemData;
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerJoinEvent>>()
                .register_reader(),
        );
    }
}

/// Returns the player info and spawn player packets
/// for the given player.
fn get_player_initialization_packets(
    ecomp: &EntityComponent,
    pcomp: &PlayerComponent,
    player: Entity,
) -> (PlayerInfo, SpawnPlayer) {
    let display_name = json!({
        "text": ecomp.display_name
    })
    .to_string();

    let mut props = vec![];
    for prop in pcomp.profile_properties.iter() {
        props.push((
            prop.name.clone(),
            prop.value.clone(),
            prop.signature.clone(),
        ));
    }

    let action = PlayerInfoAction::AddPlayer(
        ecomp.display_name.clone(),
        props,
        Gamemode::Creative,
        50,
        display_name,
    );
    let player_info = PlayerInfo::new(action, ecomp.uuid.clone());

    let metadata = EntityMetadata::new().with(&[
        (0, MetaEntry::Byte(0)),
        (1, MetaEntry::VarInt(300)),
        (2, MetaEntry::OptChat(None)),
        (3, MetaEntry::Boolean(false)),
        (4, MetaEntry::Boolean(false)),
        (5, MetaEntry::Boolean(false)),
        (6, MetaEntry::Byte(0)),
        (7, MetaEntry::Float(1.0)),
        (8, MetaEntry::VarInt(0)),
        (9, MetaEntry::Boolean(false)),
        (10, MetaEntry::VarInt(0)),
        (11, MetaEntry::Float(0.0)),
        (12, MetaEntry::VarInt(0)),
        (13, MetaEntry::Byte(0)),
        (14, MetaEntry::Byte(1)),
        // TODO NBT
    ]);

    let spawn_player = SpawnPlayer::new(
        player.id() as i32,
        ecomp.uuid.clone(),
        ecomp.position.x,
        ecomp.position.y,
        ecomp.position.z,
        degrees_to_stops(ecomp.position.pitch),
        degrees_to_stops(ecomp.position.yaw),
        metadata,
    );

    (player_info, spawn_player)
}

fn send_chunk_data(chunk: &Chunk, net: &NetworkComponent) {
    let packet = ChunkData::new(chunk.clone());
    send_packet_to_player(net, packet);
}

/// Attempts to send the chunk at the given position to
/// the given player. If the chunk is not loaded, it will
/// be loaded and sent at a later time as soon as it is
/// loaded.
pub fn send_chunk_to_player(
    chunk_pos: ChunkPosition,
    net: &NetworkComponent,
    player: Entity,
    chunk_map: &ChunkMap,
    chunk_handle: &ChunkWorkerHandle,
    lazy: &LazyUpdate,
) {
    if let Some(chunk) = chunk_map.chunk_at(chunk_pos) {
        send_chunk_data(chunk, net);
    } else {
        // Queue for loading
        load_chunk(chunk_handle, chunk_pos);
        lazy.exec_mut(move |world| {
            world
                .write_component::<ChunkPendingComponent>()
                .get_mut(player)
                .unwrap()
                .pending
                .insert(chunk_pos);
        });
    }
}

fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.) as u8
}
