//! Unit testing framework.

use feather_core::anvil::entity::BaseEntityData;
use feather_core::anvil::player::PlayerData;
use feather_core::network::Packet;
use feather_core::util::{vec3, Position};
use feather_server_chunk::{
    chunk_worker, hold_chunk_request, release_chunk_request, ChunkWorkerHandle,
};
use feather_server_network::NewClientInfo;
use feather_server_player::on_chunk_cross_update_chunks;
use feather_server_types::{
    ChunkCrossEvent, ChunkHolder, Game, ServerToWorkerMessage, Uuid, WorkerToServerMessage,
};
use feather_server_util::on_chunk_cross_update_chunk_entities;
use fecs::{
    Entity, EntityBuilder, Event, EventHandlers, Executor, OwnedResources, RawEventHandler,
    RawSystem, RefResources, ResourcesEnum, ResourcesProvider, World,
};
use std::any::{type_name, Any};
use std::borrow::Cow;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

struct TrackedPlayer {
    /// IO worker-side receiver
    worker_rx: Mutex<flume::Receiver<ServerToWorkerMessage>>,
    _worker_tx: flume::Sender<WorkerToServerMessage>,

    entity: Entity,

    buffered_sent_packets: Vec<Box<dyn Packet>>,
    disconnected: bool,
}

pub struct TestRun {
    pub game: Game,
    pub world: World,
    pub cworker_tester: ChunkWorkerTester,
    players: HashMap<Cow<'static, str>, TrackedPlayer>,
    entities: HashMap<Cow<'static, str>, Entity>,
}

impl Default for TestRun {
    fn default() -> Self {
        Self::new()
    }
}

impl TestRun {
    /// Starts a new `TestRun`.
    pub fn new() -> Self {
        let (cworker_tester, cworker_handle) = ChunkWorkerTester::new();
        let mut world = World::new();
        let game = Self::create_game(cworker_handle, &mut world);

        Self {
            game,
            world,
            cworker_tester,
            players: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    fn create_game(cworker_handle: ChunkWorkerHandle, world: &mut World) -> Game {
        let mut resources = OwnedResources::new();

        let mut event_handlers = EventHandlers::new()
            .with(hold_chunk_request)
            .with(release_chunk_request);
        event_handlers.set_up(&mut resources, world);

        let mut game = Game {
            chunk_map: Default::default(),
            tick_count: 0,
            chunk_holders: Default::default(),
            config: Arc::new(Default::default()),
            level: Default::default(),
            chunk_entities: Default::default(),
            time: Default::default(),
            running_tasks: Default::default(),
            event_handlers: Arc::new(event_handlers),
            resources: Arc::new(Default::default()),
            rng: Default::default(),
            bump: Default::default(),
            player_count: Arc::new(Default::default()),
        };
        resources.insert(cworker_handle);

        let resources = Arc::new(resources);
        game.resources = resources;
        game
    }

    /// Adds a resource into the resource set.
    pub fn with_resource(&mut self, resource: impl Any + Send + Sync) -> &mut Self {
        let resources = Arc::get_mut(&mut self.game.resources).expect("resources already borrowed");
        resources.insert(resource);

        self
    }

    /// Alias for `with_resource`
    pub fn add_resource(&mut self, resource: impl Any + Send + Sync) {
        self.with_resource(resource);
    }

    /// Runs a system for this `TestRun`.
    pub fn run(&mut self, mut system: impl RawSystem) -> &mut Self {
        system.set_up(
            Arc::get_mut(&mut self.game.resources).expect("resources already borrowed"),
            &mut self.world,
        );
        self.exec_with_resources(move |world, resources| {
            system.run(resources, world, &Executor::new())
        });
        self
    }

    /// Handles an event with the given handler.
    pub fn handle<E>(&mut self, event: E, mut handler: impl RawEventHandler<Event = E>) -> &mut Self
    where
        E: Event,
    {
        handler.set_up(
            Arc::get_mut(&mut self.game.resources).expect("resources already borrowed"),
            &mut self.world,
        );
        self.handle_with(move |_| event, handler)
    }

    /// Handles a lazily-created event with the given handler.
    ///
    /// The lazy closure has access to the `TestRun` and may
    /// as such query for player entity handles, etc.
    pub fn handle_with<E>(
        &mut self,
        event: impl FnOnce(&mut Self) -> E,
        handler: impl RawEventHandler<Event = E>,
    ) -> &mut Self
    where
        E: Event,
    {
        let event = event(self);
        self.exec_with_resources(move |world, resources| handler.handle(resources, world, &event));
        self
    }

    fn exec_with_resources(&mut self, f: impl FnOnce(&mut World, &ResourcesEnum)) {
        f(
            &mut self.world,
            &RefResources::new(Arc::clone(&self.game.resources).deref(), (&mut self.game,))
                .as_resources_ref(),
        );
    }

    /// Executes a closure with access to `self`.
    pub fn exec(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self {
        f(self);
        self
    }

    /// Creates a dummy player with the given name. Future calls
    /// which require players will allow use of this named player.
    pub fn with_player(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Self {
        let mut name = name.into();
        use feather_core::position;
        let pos = position!(0.0, 64.0, 0.0);

        let (server_tx, worker_rx) = flume::unbounded();
        let (worker_tx, server_rx) = flume::unbounded();

        let entity = EntityBuilder::new().build().spawn_in(&mut self.world);

        let info = NewClientInfo {
            ip: SocketAddr::new(IpAddr::from([0, 0, 0, 1]), 25565),
            username: name.to_mut().to_owned(),
            profile: vec![],
            uuid: Uuid::new_v4(),
            data: PlayerData {
                entity: BaseEntityData::new(pos, vec3(0.0, 0.0, 0.0)),
                gamemode: 1,
                inventory: vec![],
            },
            position: pos,
            sender: server_tx,
            receiver: server_rx,
            entity,
        };
        feather_server_player::create(&mut self.game, &mut self.world, info);

        self.players.insert(
            name.clone(),
            TrackedPlayer {
                worker_rx: Mutex::new(worker_rx),
                _worker_tx: worker_tx,
                entity,
                buffered_sent_packets: vec![],
                disconnected: false,
            },
        );
        self.entities.insert(name, entity);
        self.update_structures(entity, None, pos);
        self
    }

    /// Returns the given player's `Entity`. Panics
    /// if the player with the name does not exist.
    pub fn player(&self, name: impl Into<Cow<'static, str>>) -> Entity {
        self.players[&name.into()].entity
    }

    /// Sets the position of a player or entity. Panics
    /// if the player or entity with the name does not exist.
    pub fn with_position(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        pos: Position,
    ) -> &mut Self {
        let entity = self.entity(name);

        let old = *self.world.get::<Position>(entity);
        *self.world.get_mut::<Position>(entity) = pos;

        self.update_structures(entity, Some(old), pos);

        self
    }

    /// Adds an entity with the given name and components.
    ///
    /// _Does not trigger events_.
    pub fn with_entity(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        builder: EntityBuilder,
    ) -> &mut Self {
        let entity = builder.build().spawn_in(&mut self.world);

        self.entities.insert(name.into(), entity);
        self
    }

    /// Retrieves an entity with the given name. Panics
    /// if the entity does not exist.
    pub fn entity(&self, name: impl Into<Cow<'static, str>>) -> Entity {
        self.entities[&name.into()]
    }

    /// Verifies that the player with the given name received the given
    /// packet.
    ///
    /// # Panics
    /// Panics if the player does not exist or the packet was not received.
    pub fn assert_packet_sent<P, N>(&mut self, player: N) -> &mut Self
    where
        P: Packet,
        N: Into<Cow<'static, str>>,
    {
        let name = player.into();
        let player = self.tracked_player(&name);

        Self::update_player(player);

        if !Self::remove_player_buffered_packet::<P>(player) {
            panic!(
                "packet {} not received for player `{}`",
                type_name::<P>(),
                name
            );
        }

        self
    }

    /// Verifies that the player with the given name did not receive the given
    /// packet.
    ///
    /// # Panics
    /// Panics if the player does not exist or the packet was received.
    pub fn assert_not_packet_sent<P, N>(&mut self, player: N) -> &mut Self
    where
        P: Packet,
        N: Into<Cow<'static, str>>,
    {
        let name = player.into();
        let player = self.tracked_player(&name);

        Self::update_player(player);

        if Self::remove_player_buffered_packet::<P>(player) {
            panic!(
                "did not expect packet {} received for player `{}`",
                type_name::<P>(),
                name
            );
        }

        self
    }

    fn remove_player_buffered_packet<P>(player: &mut TrackedPlayer) -> bool
    where
        P: Packet,
    {
        let index = player
            .buffered_sent_packets
            .iter()
            .position(|p| Box::deref(p).as_any().downcast_ref::<P>().is_some());

        if let Some(index) = index {
            player.buffered_sent_packets.remove(index);
            true
        } else {
            false
        }
    }

    /// Verifies that the player with the given name was disconnected.
    pub fn assert_disconnected(&mut self, player: impl Into<Cow<'static, str>>) -> &mut Self {
        let name = player.into();
        let player = self.tracked_player(&name);
        Self::update_player(player);

        assert!(player.disconnected, "player `{}` not disconnected", name);

        self
    }

    fn tracked_player(&mut self, player: &str) -> &mut TrackedPlayer {
        self.players
            .get_mut(player)
            .unwrap_or_else(|| panic!("player `{}` does not exist", player))
    }

    fn update_player(player: &mut TrackedPlayer) {
        for msg in player.worker_rx.lock().unwrap().try_iter() {
            match msg {
                ServerToWorkerMessage::SendPacket(packet) => {
                    player.buffered_sent_packets.push(packet)
                }
                ServerToWorkerMessage::Disconnect => player.disconnected = true,
            }
        }
    }

    /// Updates acceleration structures required for tests to pass.
    fn update_structures(&mut self, entity: Entity, old: Option<Position>, new: Position) {
        let cross = ChunkCrossEvent {
            old: old.map(Position::chunk),
            new: new.chunk(),
            entity,
        };
        self.handle(cross, on_chunk_cross_update_chunk_entities);

        if self.world.has::<ChunkHolder>(entity) {
            self.handle(cross, on_chunk_cross_update_chunks);
        }
    }

    /// Verifies that an entity is alive.
    pub fn assert_alive(&mut self, entity: Entity) -> &mut Self {
        assert!(
            self.world.is_alive(entity),
            "expected entity {:?} to be alive, but it was not",
            entity
        );
        self
    }

    /// Verifies that en entity is dead.
    pub fn assert_dead(&mut self, entity: Entity) -> &mut Self {
        assert!(
            !self.world.is_alive(entity),
            "expected entity {:?} to be dead, but it was not",
            entity
        );
        self
    }
}

pub struct ChunkWorkerTester {
    pub cworker_tx: crossbeam::Sender<chunk_worker::Reply>,
    pub cworker_rx: crossbeam::Receiver<chunk_worker::Request>,
}

impl ChunkWorkerTester {
    fn new() -> (Self, ChunkWorkerHandle) {
        let (cworker_tx, rx) = crossbeam::unbounded();
        let (tx, cworker_rx) = crossbeam::unbounded();

        (
            Self {
                cworker_rx,
                cworker_tx,
            },
            ChunkWorkerHandle {
                receiver: rx,
                sender: tx,
            },
        )
    }
}
