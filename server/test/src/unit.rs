//! Unit testing framework.

use feather_core::anvil::entity::BaseEntityData;
use feather_core::anvil::player::PlayerData;
use feather_core::network::{cast_packet, Packet};
use feather_core::util::{vec3, Position};
use feather_server_chunk::{
    chunk_worker, hold_chunk_request, release_chunk_request, ChunkWorkerHandle,
};
use feather_server_network::NewClientInfo;
use feather_server_player::on_chunk_cross_update_chunks;
use feather_server_types::{
    ChunkCrossEvent, ChunkHolder, Game, Name, NetworkId, ServerToWorkerMessage, Shared, Uuid,
    WorkerToServerMessage,
};
use feather_server_util::on_chunk_cross_update_chunk_entities;
use fecs::{
    Entity, EntityBuilder, Event, EventHandlers, Executor, OwnedResources, RawEventHandler,
    RawSystem, RefResources, ResourcesEnum, ResourcesProvider, World,
};
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

struct TrackedPlayer {
    /// IO worker-side receiver
    worker_rx: Mutex<flume::Receiver<ServerToWorkerMessage>>,
    _worker_tx: flume::Sender<WorkerToServerMessage>,

    buffered_sent_packets: Vec<Box<dyn Packet>>,
    disconnected: bool,
}

pub struct Test {
    pub game: Game,
    pub world: World,
    pub cworker_tester: ChunkWorkerTester,
    players: HashMap<Entity, TrackedPlayer>,
}

impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}

impl Test {
    /// Starts a new `Test`.
    pub fn new() -> Self {
        let (cworker_tester, cworker_handle) = ChunkWorkerTester::new();
        let mut world = World::new();
        let game = Self::create_game(cworker_handle, &mut world);

        Self {
            game,
            world,
            cworker_tester,
            players: HashMap::new(),
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
            level: Default::default(),
            chunk_entities: Default::default(),
            block_entities: Default::default(),
            time: Default::default(),
            event_handlers: Arc::new(event_handlers),
            resources: Arc::new(Default::default()),
            bump: Default::default(),
            shared: Arc::new(Shared {
                config: Arc::new(Default::default()),
                rng: Default::default(),
                player_count: Arc::new(Default::default()),
            }),
        };
        resources.insert(cworker_handle);

        let resources = Arc::new(resources);
        game.resources = resources;
        game
    }

    /// Adds a resource into the resource set.
    pub fn with_resource(mut self, resource: impl Any + Send + Sync) -> Self {
        let resources = Arc::get_mut(&mut self.game.resources).expect("resources already borrowed");
        resources.insert(resource);

        self
    }

    /// Runs a system for this `Test`.
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

    /// Runs a broadcast test routine. This:
    /// * Creates three players, two of whom are able to see each other
    /// * Calls `event` to trigger an event for the first player
    /// * Asserts that no packet was sent to the third player, who is too far away
    /// * Asserts that packet of type `P` was sent to the second player, returning it
    pub fn broadcast_routine<P, E, F, H>(
        &mut self,
        event: F,
        handler: H,
        send_to_self: bool,
    ) -> (P, Entity)
    where
        E: Event,
        P: Packet,
        F: FnOnce(&mut Self, Entity, Entity) -> E,
        H: RawEventHandler<Event = E>,
    {
        use feather_core::position;
        let player1 = self.player("", position!(0.0, 64.0, 0.0));
        let player2 = self.player("", position!(45.0, 1000.0, -37.9));
        let player3 = self.player("", position!(1000.0, -450.0, 100.0));

        let event = event(self, player1, player2);
        self.handle(event, handler);

        if !send_to_self {
            assert!(self.sent::<P>(player1).is_none());
        }
        assert!(self.sent::<P>(player3).is_none());

        (self.sent::<P>(player2).unwrap(), player1)
    }

    /// Creates a dummy player with the given name.
    pub fn player(&mut self, name: impl Into<Cow<'static, str>>, position: Position) -> Entity {
        let mut name = name.into();

        let (server_tx, worker_rx) = flume::unbounded();
        let (worker_tx, server_rx) = flume::unbounded();

        let entity = EntityBuilder::new().build().spawn_in(&mut self.world);

        let info = NewClientInfo {
            ip: SocketAddr::new(IpAddr::from([0, 0, 0, 1]), 25565),
            username: name.to_mut().to_owned(),
            profile: vec![],
            uuid: Uuid::new_v4(),
            data: PlayerData {
                entity: BaseEntityData::new(position, vec3(0.0, 0.0, 0.0), 20.0),
                gamemode: 1,
                inventory: vec![],
            },
            position,
            sender: server_tx,
            receiver: server_rx,
            entity,
        };
        feather_server_player::create(&mut self.game, &mut self.world, info);

        self.players.insert(
            entity,
            TrackedPlayer {
                worker_rx: Mutex::new(worker_rx),
                _worker_tx: worker_tx,
                buffered_sent_packets: vec![],
                disconnected: false,
            },
        );
        self.update_structures(entity, None, position);
        entity
    }

    /// Adds an entity with the given name and components.
    pub fn entity(&mut self, builder: EntityBuilder) -> Entity {
        let entity = builder.build().spawn_in(&mut self.world);

        if let Some(pos) = self.world.try_get::<Position>(entity).map(|r| *r) {
            self.update_structures(entity, None, pos);
        }

        entity
    }

    /// Sets the position of an entity.
    pub fn position(&mut self, entity: Entity, pos: Position) -> &mut Self {
        let old = *self.world.get::<Position>(entity);
        *self.world.get_mut::<Position>(entity) = pos;

        self.update_structures(entity, Some(old), pos);

        self
    }

    /// Returns the network ID of an entity.
    pub fn id(&self, entity: Entity) -> i32 {
        self.world.get::<NetworkId>(entity).0
    }

    /// Returns the UUID of an entity.
    pub fn uuid(&self, entity: Entity) -> Uuid {
        *self.world.get::<Uuid>(entity)
    }

    /// Returns the packet of type `P` sent to `player`.
    pub fn sent<P>(&mut self, player: Entity) -> Option<P>
    where
        P: Packet,
    {
        let tracked = self.tracked_player(player);

        Self::update_player(tracked);

        Self::remove_player_buffered_packet(tracked)
    }

    fn remove_player_buffered_packet<P>(player: &mut TrackedPlayer) -> Option<P>
    where
        P: Packet,
    {
        let index = player
            .buffered_sent_packets
            .iter()
            .position(|p| Box::deref(p).as_any().downcast_ref::<P>().is_some());

        index.map(|index| cast_packet(player.buffered_sent_packets.remove(index)))
    }

    /// Verifies that the player with the given name was disconnected.
    pub fn assert_disconnected(&mut self, player: Entity) -> &mut Self {
        let tracked = self.tracked_player(player);
        Self::update_player(tracked);

        assert!(
            tracked.disconnected,
            "player `{}` not disconnected",
            self.world.get::<Name>(player).0
        );

        self
    }

    fn tracked_player(&mut self, player: Entity) -> &mut TrackedPlayer {
        self.players
            .get_mut(&player)
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
