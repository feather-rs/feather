use std::cell::{Ref, RefMut};
use std::{cell::RefCell, mem, rc::Rc, sync::Arc};

use ahash::AHashMap;
use libcraft::EntityKind;
use libcraft::{Position, Text, Title};
use quill::chat::{ChatKind, ChatMessage};
use quill::components::EntityPosition;
use quill::entities::Player;
use quill::events::{EntityCreateEvent, EntityRemoveEvent, PlayerJoinEvent};
use quill::game::{WorldGeneratorFactoryNotFound, WorldNotFound, WorldSourceFactoryNotFound};
use quill::saveload::worldgen::WorldGeneratorFactory;
use quill::saveload::WorldSourceFactory;
use quill::threadpool::ThreadPool;
use quill::world::WorldDescriptor;
use quill::{ChatBox, World as _, WorldId};
use tokio::runtime::{self, Runtime};
use vane::{
    Entities, Entity, EntityBuilder, EntityDead, HasEntities, HasResources, Resources, SysResult,
    SystemExecutor,
};

use crate::chunk::entities::ChunkEntities;
use crate::events::PlayerRespawnEvent;
use crate::world::World;

type EntitySpawnCallback = Box<dyn FnMut(&mut EntityBuilder, EntityKind)>;

/// Stores the entire state of a Minecraft game.
///
/// This contains:
/// * A [`World`](crate::World) containing chunks and blocks.
/// * An [`Ecs`](vane::Ecs) containing entities.
/// * A [`Resources`](vane::Resources) containing additional, user-defined data.
/// * A [`SystemExecutor`] to run systems.
///
/// `feather-common` provides `Game` methods for actions such
/// as "drop item" or "kill entity." These high-level methods
/// should be preferred over raw interaction with the ECS.
pub struct Game {
    /// Contains entities, including players.
    pub ecs: Entities,
    /// Contains systems.
    pub system_executor: Rc<RefCell<SystemExecutor<Game>>>,

    /// User-defined resources.
    ///
    /// Stored in an `Arc` for borrow-checker purposes.
    pub resources: Arc<Resources>,

    /// A spatial index to efficiently find which entities are in a given chunk.
    pub chunk_entities: ChunkEntities,

    /// Total ticks elapsed since the server started.
    pub tick_count: u64,

    world_source_factories: AHashMap<String, Box<dyn WorldSourceFactory>>,
    world_generator_factories: AHashMap<String, Box<dyn WorldGeneratorFactory>>,

    worlds: AHashMap<WorldId, RefCell<World>>,
    default_world: WorldId,

    entity_spawn_callbacks: Vec<EntitySpawnCallback>,

    entity_builder: EntityBuilder,

    /// The Tokio runtime shared by the server and all plugins.
    runtime: Runtime,

    compute_pool: ThreadPool,
}

impl Default for Game {
    fn default() -> Self {
        Self::new(
            runtime::Builder::new_current_thread()
                .build()
                .expect("failed to initialize default Tokio runtime"),
        )
    }
}

impl Game {
    /// Creates a new, empty `Game`.
    pub fn new(runtime: Runtime) -> Self {
        Self {
            ecs: Entities::new(),
            system_executor: Rc::new(RefCell::new(SystemExecutor::new())),
            resources: Arc::new(Resources::new()),
            chunk_entities: ChunkEntities::default(),
            tick_count: 0,
            world_source_factories: AHashMap::new(),
            world_generator_factories: AHashMap::new(),
            worlds: AHashMap::new(),
            default_world: WorldId::new_random(), // needs to be set
            entity_spawn_callbacks: Vec::new(),
            entity_builder: EntityBuilder::new(),
            runtime,
            compute_pool: ThreadPool::new("compute", num_cpus::get()),
        }
    }

    /// Inserts a new resource.
    ///
    /// An existing resource with type `T` is overridden.
    ///
    /// # Panics
    /// Panics if any resources are currently borrowed.
    pub fn insert_resource<T>(&mut self, resource: T)
    where
        T: 'static,
    {
        Arc::get_mut(&mut self.resources)
            .expect("attempted to insert into resources while resources are borrowed")
            .insert(resource);
    }

    /// Adds a new entity spawn callback, invoked
    /// before an entity is created.
    ///
    /// This allows you to add components to entities
    /// before they are built.
    pub fn add_entity_spawn_callback(
        &mut self,
        callback: impl FnMut(&mut EntityBuilder, EntityKind) + 'static,
    ) {
        self.entity_spawn_callbacks.push(Box::new(callback));
    }

    /// Creates an empty entity builder to create entities in
    /// the ecs world.
    pub fn create_empty_entity_builder(&mut self) -> EntityBuilder {
        mem::take(&mut self.entity_builder)
    }

    /// Creates an entity builder with the default components
    /// for an entity of type `init`.
    pub fn create_entity_builder(&mut self, position: Position, kind: EntityKind) -> EntityBuilder {
        let mut builder = mem::take(&mut self.entity_builder);
        builder.add(EntityPosition(position));
        self.invoke_entity_spawn_callbacks(&mut builder, kind);
        builder
    }

    /// Spawns an entity and returns its [`Entity`](vane::Entity) handle.
    ///
    /// Also triggers necessary events, like `EntitySpawnEvent` and `PlayerJoinEvent`.
    pub fn spawn_entity(&mut self, mut builder: EntityBuilder) -> Entity {
        let entity = self.ecs.spawn_builder(&mut builder);
        self.entity_builder = builder;

        self.trigger_entity_spawn_events(entity);

        entity
    }

    fn invoke_entity_spawn_callbacks(&mut self, builder: &mut EntityBuilder, kind: EntityKind) {
        let mut callbacks = mem::take(&mut self.entity_spawn_callbacks);
        for callback in &mut callbacks {
            callback(builder, kind);
        }
        self.entity_spawn_callbacks = callbacks;
    }

    fn trigger_entity_spawn_events(&mut self, entity: Entity) {
        self.ecs
            .insert_entity_event(entity, EntityCreateEvent)
            .unwrap();
        if self.ecs.has::<Player>(entity) {
            self.ecs
                .insert_entity_event(entity, PlayerJoinEvent)
                .unwrap();
            self.ecs
                .insert_entity_event(entity, PlayerRespawnEvent)
                .unwrap();
        }
    }

    /// Causes the given entity to be removed on the next tick.
    /// In the meantime, triggers `EntityRemoveEvent`.
    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), EntityDead> {
        self.ecs.defer_despawn(entity);
        self.ecs.insert_entity_event(entity, EntityRemoveEvent)
    }

    /// Broadcasts a chat message to all entities with
    /// a `ChatBox` component (usually just players).
    pub fn broadcast_chat(&self, kind: ChatKind, message: impl Into<Text>) {
        let message = message.into();
        for (_, mut mailbox) in self.ecs.query::<&mut ChatBox>().iter() {
            mailbox.send(ChatMessage::new(kind, message.clone()));
        }
    }

    /// Utility method to send a message to an entity.
    pub fn send_message(&mut self, entity: Entity, message: ChatMessage) -> SysResult {
        let mut mailbox = self.ecs.get_mut::<ChatBox>(entity)?;
        mailbox.send(message);
        Ok(())
    }

    /// Utility method to send a title to an entity.
    pub fn send_title(&mut self, entity: Entity, title: Title) -> SysResult {
        let mut mailbox = self.ecs.get_mut::<ChatBox>(entity)?;
        mailbox.send_title(title);
        Ok(())
    }

    pub fn default_world(&self) -> Ref<dyn quill::World> {
        <Self as quill::Game>::default_world(self)
    }

    pub fn default_world_mut(&self) -> RefMut<dyn quill::World> {
        <Self as quill::Game>::default_world_mut(self)
    }

    pub fn default_world_id(&self) -> WorldId {
        <Self as quill::Game>::default_world_id(self)
    }

    pub fn world(&self, id: WorldId) -> Result<Ref<dyn quill::World>, WorldNotFound> {
        <Self as quill::Game>::world(self, id)
    }

    pub fn world_mut(&self, id: WorldId) -> Result<RefMut<dyn quill::World>, WorldNotFound> {
        <Self as quill::Game>::world_mut(self, id)
    }

    pub fn worlds_mut(&self) -> impl Iterator<Item = RefMut<World>> + '_ {
        self.worlds.values().map(RefCell::borrow_mut)
    }
}

impl quill::Game for Game {
    fn ecs(&self) -> &Entities {
        &self.ecs
    }

    fn ecs_mut(&mut self) -> &mut Entities {
        &mut self.ecs
    }

    fn resources(&self) -> &Resources {
        &self.resources
    }

    fn resources_mut(&mut self) -> &mut Resources {
        Arc::get_mut(&mut self.resources)
            .expect("attempted to mutate Resources while a resource is borrowed")
    }

    fn create_world(&mut self, desc: WorldDescriptor) {
        log::info!(
            "Created world '{}'",
            desc.name.clone().unwrap_or_else(|| desc.id.to_string())
        );
        let world = World::new(desc, self);
        self.worlds.insert(world.id(), RefCell::new(world));
    }

    fn register_world_source_factory(&mut self, name: &str, factory: Box<dyn WorldSourceFactory>) {
        self.world_source_factories.insert(name.to_owned(), factory);
    }

    fn register_world_generator_factory(
        &mut self,
        name: &str,
        factory: Box<dyn WorldGeneratorFactory>,
    ) {
        self.world_generator_factories
            .insert(name.to_owned(), factory);
    }

    fn world_source_factory(
        &self,
        name: &str,
    ) -> Result<&dyn WorldSourceFactory, WorldSourceFactoryNotFound> {
        self.world_source_factories
            .get(name)
            .ok_or_else(|| WorldSourceFactoryNotFound(name.to_owned()))
            .map(|b| &**b)
    }

    fn world_generator_factory(
        &self,
        name: &str,
    ) -> Result<&dyn WorldGeneratorFactory, WorldGeneratorFactoryNotFound> {
        self.world_generator_factories
            .get(name)
            .ok_or_else(|| WorldGeneratorFactoryNotFound(name.to_owned()))
            .map(|b| &**b)
    }

    fn remove_world(&mut self, id: WorldId) -> Result<(), WorldNotFound> {
        self.worlds
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| WorldNotFound(id))
    }

    fn world(&self, id: WorldId) -> Result<Ref<dyn quill::World>, WorldNotFound> {
        Ok(Ref::map(
            self.worlds
                .get(&id)
                .ok_or_else(|| WorldNotFound(id))?
                .borrow(),
            |world| world,
        ))
    }

    fn world_mut(&self, id: WorldId) -> Result<RefMut<dyn quill::World>, WorldNotFound> {
        Ok(RefMut::map(
            self.worlds
                .get(&id)
                .ok_or_else(|| WorldNotFound(id))?
                .borrow_mut(),
            |world| world,
        ))
    }

    fn set_default_world(&mut self, id: WorldId) {
        assert!(
            self.worlds.contains_key(&id),
            "tried to set default world to world ID {}, which does not exist",
            id
        );
        self.default_world = id;
    }

    fn default_world_id(&self) -> WorldId {
        self.default_world
    }

    fn spawn_entity(&mut self, builder: EntityBuilder) -> Entity {
        Game::spawn_entity(self, builder)
    }

    fn queue_remove_entity(&mut self, entity: Entity) {
        self.remove_entity(entity).ok();
    }

    fn tokio_runtime(&self) -> runtime::Handle {
        self.runtime.handle().clone()
    }

    fn compute_pool(&self) -> &quill::threadpool::ThreadPool {
        &self.compute_pool
    }
}

impl HasResources for Game {
    fn resources(&self) -> Arc<Resources> {
        Arc::clone(&self.resources)
    }
}

impl HasEntities for Game {
    fn entities(&self) -> &Entities {
        &self.ecs
    }

    fn entities_mut(&mut self) -> &mut Entities {
        &mut self.ecs
    }
}
