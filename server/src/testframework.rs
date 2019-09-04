//! Helper framework for writing unit tests.

use std::net::TcpListener;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use glm::DVec3;
use mio_extras::channel::{channel, Receiver, Sender};
use rand::Rng;
use shrev::EventChannel;
use specs::{Builder, Dispatcher, DispatcherBuilder, Entity, ReaderId, System, World, WorldExt};
use uuid::Uuid;

use feather_core::level::LevelData;
use feather_core::network::packet::{Packet, PacketType};
use feather_core::world::block::Block;
use feather_core::world::chunk::Chunk;
use feather_core::world::{BlockPosition, ChunkMap, ChunkPosition, Position};
use feather_core::Gamemode;

use crate::chunk_logic::ChunkHolders;
use crate::config::Config;
use crate::entity::metadata::{self, Metadata};
use crate::entity::{
    ChunkEntities, EntityDestroyEvent, EntitySpawnEvent, EntityType, ItemComponent, NamedComponent,
    PlayerComponent, PositionComponent, VelocityComponent,
};
use crate::io::ServerToWorkerMessage;
use crate::network::{NetworkComponent, PacketQueue};
use crate::player::{InventoryComponent, PlayerDisconnectEvent};
use crate::util::BroadcasterSystem;
use crate::PlayerCount;

/// Initializes a Specs world and dispatcher
/// using default configuration options and an
/// available server port.
pub fn init_world<'a, 'b>() -> (World, Dispatcher<'a, 'b>) {
    let mut config = Config::default();
    config.server.port = find_open_port().unwrap();

    let config = Arc::new(config);

    let player_count = Arc::new(PlayerCount(AtomicUsize::new(0)));
    let server_icon = Arc::new(None);
    let ioman = super::init_io_manager(
        Arc::clone(&config),
        Arc::clone(&player_count),
        Arc::clone(&server_icon),
    );
    let level = LevelData::default();

    super::init_world(config, player_count, ioman, level)
}

pub struct Player {
    pub entity: Entity,
    pub network_sender: Sender<ServerToWorkerMessage>,
    pub network_receiver: Receiver<ServerToWorkerMessage>,
}

/// Adds a player to the world, inserting
/// all the necessary components. Returns
/// a number of useful channels.
///
/// # Notes
/// * A `ChunkHolders` and `ChunkEntities` entry
/// is created for the player. If this behavior is not
/// desired, use `add_player_without_holder`.
pub fn add_player(world: &mut World) -> Player {
    let player = add_player_without_holder(world);

    let mut chunk_holders = world.fetch_mut::<ChunkHolders>();

    let view_distance = i32::from(world.fetch::<Arc<Config>>().server.view_distance);

    for x in -view_distance..=view_distance {
        for z in -view_distance..=view_distance {
            chunk_holders.insert_holder(ChunkPosition::new(x, z), player.entity);
        }
    }

    let mut chunk_entities = world.fetch_mut::<ChunkEntities>();
    chunk_entities.add_to_chunk(ChunkPosition::new(0, 0), player.entity);

    player
}

/// Adds a player to the world without adding the `ChunkHolders`
/// and `ChunkEntities` entries.
pub fn add_player_without_holder(world: &mut World) -> Player {
    let (ns1, nr1) = channel();
    let (ns2, nr2) = channel();
    let entity = world
        .create_entity()
        .with(NetworkComponent::new(ns1, nr2))
        .with(PlayerComponent {
            gamemode: Gamemode::Creative,
            profile_properties: vec![],
        })
        .with(PositionComponent {
            current: position!(0.0, 0.0, 0.0),
            previous: position!(0.0, 0.0, 0.0),
        })
        .with(NamedComponent {
            display_name: "".to_string(),
            uuid: Uuid::new_v4(),
        })
        .with(InventoryComponent::default())
        .with(Metadata::Player(metadata::Player::default()))
        .with(EntityType::Player)
        .build();

    Player {
        entity,
        network_sender: ns2,
        network_receiver: nr1,
    }
}

/// Asserts that the given player has received
/// a packet of the given type, returning the packet.
pub fn assert_packet_received(player: &Player, ty: PacketType) -> Box<dyn Packet> {
    while let Ok(msg) = player.network_receiver.try_recv() {
        if let ServerToWorkerMessage::SendPacket(packet) = msg {
            if packet.ty() == ty {
                return packet;
            }
        }
    }

    panic!();
}

/// Asserts that a player did not receive
/// any packets of the given type.
/// Panics if not.
pub fn assert_packet_not_received(player: &Player, ty: PacketType) {
    while let Ok(msg) = player.network_receiver.try_recv() {
        if let ServerToWorkerMessage::SendPacket(packet) = msg {
            assert_ne!(packet.ty(), ty);
        }
    }
}

/// Retrieves up to `cap` packets sent to a player, if any.
/// If `cap` is set to `None`, all packets will be read.
///
/// Note that this function consumes messages in
/// the network channel until enough packets have been read.
pub fn received_packets(player: &Player, cap: Option<usize>) -> Vec<Box<dyn Packet>> {
    let mut result = vec![];

    while let Ok(msg) = player.network_receiver.try_recv() {
        if let ServerToWorkerMessage::SendPacket(pack) = msg {
            result.push(pack);
        }
        if let Some(cap) = cap.as_ref() {
            if result.len() >= *cap {
                break;
            }
        }
    }

    result
}

/// Adds a received packet to the packet queue
/// for a given player.
pub fn receive_packet<P: Packet + 'static>(player: &Player, world: &World, packet: P) {
    let queue = world.fetch_mut::<PacketQueue>();
    queue.add_for_packet(player.entity, Box::new(packet));
}

/// Attempts to find an available port.
fn find_open_port() -> Option<u16> {
    let start = rand::thread_rng().gen_range(10000, 30000);
    (start..60000).find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
}

/// Asserts that a player was disconnected, panicking if not.
pub fn assert_disconnected(player: &Player) {
    let mut disconnected = false;
    for packet in received_packets(player, None) {
        if packet.ty() == PacketType::DisconnectPlay {
            disconnected = true;
        }
    }

    assert!(disconnected);
}

/// Asserts that a player was not disconnected, panicking
/// if they were.
pub fn assert_not_disconnected(player: &Player) {
    let mut disconnected = false;
    for packet in received_packets(player, None) {
        if packet.ty() == PacketType::DisconnectPlay {
            disconnected = true;
        }
    }

    assert!(!disconnected);
}

/// Sends a packet to the player.
pub fn send_packet<P: Packet + 'static>(player: &Player, packet: P) {
    player
        .network_sender
        .send(ServerToWorkerMessage::NotifyPacketReceived(Box::new(
            packet,
        )))
        .unwrap();
}

/// Registers a reader for events of the given type.
pub fn reader<E: Send + Sync>(w: &World) -> ReaderId<E> {
    let mut channel = w.fetch_mut::<EventChannel<E>>();
    channel.register_reader()
}

/// Triggers the given event, writing it to
/// the corresponding `EventChannel`.
pub fn trigger_event<E: Send + Sync + 'static>(world: &World, event: E) {
    let mut channel = world.fetch_mut::<EventChannel<E>>();
    channel.single_write(event);
}

/// Returns all triggered events of a given type.
pub fn triggered_events<E: Send + Sync + Clone + 'static>(
    world: &World,
    reader: &mut ReaderId<E>,
) -> Vec<E> {
    let channel = world.fetch::<EventChannel<E>>();
    channel.read(reader).cloned().collect()
}

/// Creates an entity at the origin with zero
/// velocity.
///
///
/// # Notes
/// * A `ChunkHolders` and `ChunkEntities` entry
/// is created for the entity. If this behavior is not
/// desired, use `add_entity_without_holder`.
pub fn add_entity(world: &mut World, ty: EntityType, trigger_spawn_event: bool) -> Entity {
    add_entity_with_pos(world, ty, Position::default(), trigger_spawn_event)
}

/// Creates an entity at the origin with zero velocity, without
/// adding a chunk holder or chunk entities entry for it.
pub fn add_entity_without_holder(
    world: &mut World,
    ty: EntityType,
    trigger_spawn_event: bool,
) -> Entity {
    add_entity_without_holder_with_pos(world, ty, Position::default(), trigger_spawn_event)
}

/// Creates an entity with the given position
/// and zero velocity.
pub fn add_entity_with_pos(
    world: &mut World,
    ty: EntityType,
    pos: Position,
    trigger_spawn_event: bool,
) -> Entity {
    add_entity_with_pos_and_vel(
        world,
        ty,
        pos,
        glm::vec3(0.0, 0.0, 0.0),
        trigger_spawn_event,
    )
}

pub fn add_entity_without_holder_with_pos(
    world: &mut World,
    ty: EntityType,
    pos: Position,
    trigger_spawn_event: bool,
) -> Entity {
    add_entity_without_holder_with_pos_and_vel(
        world,
        ty,
        pos,
        glm::vec3(0.0, 0.0, 0.0),
        trigger_spawn_event,
    )
}

/// Creates an entity with the given position and velocity.
pub fn add_entity_with_pos_and_vel(
    world: &mut World,
    ty: EntityType,
    pos: Position,
    vel: DVec3,
    trigger_spawn_event: bool,
) -> Entity {
    let entity =
        add_entity_without_holder_with_pos_and_vel(world, ty, pos, vel, trigger_spawn_event);

    let mut chunk_entities = world.fetch_mut::<ChunkEntities>();
    chunk_entities.add_to_chunk(pos.chunk_pos(), entity);

    entity
}

pub fn add_entity_without_holder_with_pos_and_vel(
    world: &mut World,
    ty: EntityType,
    pos: Position,
    vel: DVec3,
    trigger_spawn_event: bool,
) -> Entity {
    let entity = world
        .create_entity()
        .with(PositionComponent {
            current: pos,
            previous: pos,
        })
        .with(VelocityComponent(vel))
        .with(NamedComponent {
            uuid: Uuid::new_v4(),
            display_name: "bla".to_string(),
        })
        .with(ty)
        .with(Metadata::Entity(metadata::Entity::default()))
        .build();

    if ty == EntityType::Item {
        world
            .write_component::<ItemComponent>()
            .insert(entity, ItemComponent { collectable_at: 20 })
            .unwrap();
    }

    if trigger_spawn_event {
        let event = EntitySpawnEvent { entity, ty };
        trigger_event(&world, event);
    }
    entity
}

/// Populates a 15x15 area of chunks around the origin
/// with air.
pub fn populate_with_air(world: &mut World) {
    for x in -15..=15 {
        for z in -15..=15 {
            let chunk = Chunk::new(ChunkPosition::new(x, z));
            world
                .fetch_mut::<ChunkMap>()
                .set_chunk_at(chunk.position(), chunk);
        }
    }
}

/// Asserts that an entity was not removed.
pub fn assert_not_removed(world: &World, entity: Entity) {
    assert!(world.entities().is_alive(entity));
}

/// Asserts that an entity was removed.
pub fn assert_removed(world: &World, entity: Entity) {
    assert!(!world.entities().is_alive(entity));
}

/// Retrieves the position of an entity.
pub fn entity_pos(world: &World, entity: Entity) -> Position {
    world
        .read_component::<PositionComponent>()
        .get(entity)
        .unwrap()
        .current
}

/// Retrieves the velocity of an entity.
pub fn entity_vel(world: &World, entity: Entity) -> Option<DVec3> {
    if let Some(comp) = world.read_component::<VelocityComponent>().get(entity) {
        Some(comp.0)
    } else {
        None
    }
}

/// Sets an entity's position.
pub fn set_entity_pos(world: &World, entity: Entity, pos: Position) {
    let mut storage = world.write_component::<PositionComponent>();
    storage.get_mut(entity).unwrap().current = pos;
}

/// Sets an entity's velocity.
pub fn set_entity_velocity(world: &World, entity: Entity, vel: DVec3) {
    let mut storage = world.write_component::<VelocityComponent>();
    storage.get_mut(entity).unwrap().0 = vel;
}

/// Sets the block at the given position in the world.
pub fn set_block(x: i32, y: i32, z: i32, block: Block, world: &World) {
    let mut chunk_map = world.fetch_mut::<ChunkMap>();
    chunk_map
        .set_block_at(BlockPosition::new(x, y, z), block)
        .unwrap();
}

/// A dispatcher builder for isolating tests.
pub struct TestBuilder<'a, 'b> {
    world: World,
    dispatcher: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> TestBuilder<'a, 'b> {
    pub fn with<S>(mut self, system: S, name: &'static str) -> Self
    where
        S: for<'c> System<'c> + Send + 'a,
    {
        self.dispatcher.add(system, name, &[]);
        self
    }

    pub fn with_dep<S>(mut self, system: S, name: &'static str, deps: &[&'static str]) -> Self
    where
        S: for<'c> System<'c> + Send + 'a,
    {
        self.dispatcher.add(system, name, deps);
        self
    }

    pub fn build(mut self) -> (World, Dispatcher<'a, 'b>) {
        self.world
            .insert(Arc::new(PlayerCount(AtomicUsize::new(0))));
        self.world
            .insert(EventChannel::<PlayerDisconnectEvent>::new());
        self.world.insert(EventChannel::<EntityDestroyEvent>::new());
        self.world.insert(EventChannel::<EntitySpawnEvent>::new());
        self.world.insert(crate::time::Time(0));
        self.world.insert(ChunkHolders::default());
        self.world.insert(ChunkEntities::default());
        self.world.insert(Arc::new(Config::default()));

        // Insert the broadcaster system, since it is so commonly
        // used that it should be used for all tests.
        self.dispatcher.add_barrier();
        self.dispatcher.add(BroadcasterSystem, "", &[]);

        let mut dispatcher = self.dispatcher.build();
        dispatcher.setup(&mut self.world);

        self.world.register::<VelocityComponent>();
        self.world.register::<PositionComponent>();
        self.world.register::<NamedComponent>();
        self.world.register::<Metadata>();
        self.world.register::<EntityType>();
        self.world.register::<ItemComponent>();
        self.world.register::<PlayerComponent>();
        self.world.register::<InventoryComponent>();
        self.world.register::<NetworkComponent>();

        (self.world, dispatcher)
    }
}

pub fn builder<'a, 'b>() -> TestBuilder<'a, 'b> {
    TestBuilder {
        world: World::new(),
        dispatcher: DispatcherBuilder::new(),
    }
}

/// Heh... tests for the testing framework.
/// Not sure what the point of this is, since
/// all other tests would fail if the testing
/// framework didn't work.
mod tests {
    use feather_core::network::packet::implementation::{DisconnectPlay, LoginStart};

    use crate::entity::{PlayerComponent, PositionComponent};
    use crate::network::{send_packet_to_player, NetworkComponent};

    use super::*;

    #[test]
    fn test_find_open_port() {
        let port = find_open_port().unwrap();
        println!("Found open port: {}", port);
        assert!(TcpListener::bind(("127.0.0.1", port)).is_ok());
    }

    #[test]
    fn test_init_world() {
        // Check that initializing the world doesn't cause
        // a panic.
        let (w, mut d) = init_world();

        // Check that running the dispatcher works fine
        d.dispatch(&w);
    }

    #[test]
    fn test_add_player() {
        let (mut w, _) = init_world();

        let entity = add_player(&mut w).entity;

        assert!(w.read_component::<PlayerComponent>().get(entity).is_some());
        assert!(w
            .read_component::<PositionComponent>()
            .get(entity)
            .is_some());
        assert!(w.read_component::<NetworkComponent>().get(entity).is_some());
    }

    #[test]
    fn test_received_packets() {
        let (mut w, _) = init_world();

        let player = add_player(&mut w);

        let cap = 1;
        send_packet_to_player(
            w.read_component().get(player.entity).unwrap(),
            LoginStart::new("".to_string()),
        );
        send_packet_to_player(
            w.read_component().get(player.entity).unwrap(),
            LoginStart::new("".to_string()),
        );

        let packets = received_packets(&player, Some(cap));
        assert_eq!(packets.len(), 1);

        let packets = received_packets(&player, Some(cap));
        assert_eq!(packets.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_assert_packet_received() {
        let (mut w, _) = init_world();

        let player = add_player(&mut w);
        assert_packet_received(&player, PacketType::Handshake);
    }

    #[test]
    #[should_panic]
    fn test_assert_disconnected() {
        let (mut w, _) = init_world();

        let player = add_player(&mut w);
        assert_disconnected(&player);
    }

    #[test]
    #[should_panic]
    fn test_assert_not_disconnected() {
        let (mut w, _) = init_world();

        let disconnect = DisconnectPlay::new("bla".to_string());

        let player = add_player(&mut w);
        send_packet_to_player(w.read_component().get(player.entity).unwrap(), disconnect);
        assert_not_disconnected(&player);
    }
}
