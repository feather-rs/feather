use std::{cell::RefCell, mem, rc::Rc, sync::Arc};

use base::{BlockId, BlockPosition, ChunkPosition, Position, Text};
use ecs::{
    Ecs, Entity, EntityBuilder, HasEcs, HasResources, NoSuchEntity, Resources, SysResult,
    SystemExecutor,
};
use quill_common::{entities::Player, entity_init::EntityInit};

use crate::{
    chat::{ChatKind, ChatMessage},
    chunk_entities::ChunkEntities,
    events::{BlockChangeEvent, EntityCreateEvent, EntityRemoveEvent, PlayerJoinEvent},
    ChatBox, Level,
};

type EntitySpawnCallback = Box<dyn FnMut(&mut EntityBuilder, &EntityInit)>;

/// Stores the entire state of a Minecraft game.
///
/// This contains:
/// * A [`World`](vane::Ecs) containing entities.
/// * A [`Level`] containing blocks.
/// * A [`Resources`](ecs::Resources) containing additional, user-defined data.
/// * A [`SystemExecutor`] to run systems.
///
/// `feather-common` provides `Game` methods for actions such
/// as "drop item" or "kill entity." These high-level methods
/// should be preferred over raw interaction with the ECS.
pub struct Game {
    /// Contains chunks and blocks.
    pub level: Level,
    /// Contains entities, including players.
    pub world: Ecs,
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

    entity_spawn_callbacks: Vec<EntitySpawnCallback>,

    entity_builder: EntityBuilder,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Creates a new, empty `Game`.
    pub fn new() -> Self {
        Self {
            level: Level::new(),
            world: Ecs::new(),
            system_executor: Rc::new(RefCell::new(SystemExecutor::new())),
            resources: Arc::new(Resources::new()),
            chunk_entities: ChunkEntities::default(),
            tick_count: 0,
            entity_spawn_callbacks: Vec::new(),
            entity_builder: EntityBuilder::new(),
        }
    }

    /// Inserts a new resource.
    ///
    /// An existing resource with type `T` is overriden.
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
        callback: impl FnMut(&mut EntityBuilder, &EntityInit) + 'static,
    ) {
        self.entity_spawn_callbacks.push(Box::new(callback));
    }

    /// Creates an emtpy entity builder to create entities in
    /// the ecs world.
    pub fn create_empty_entity_builder(&mut self) -> EntityBuilder {
        mem::take(&mut self.entity_builder)
    }

    /// Creates an entity builder with the default components
    /// for an entity of type `init`.
    pub fn create_entity_builder(&mut self, position: Position, init: EntityInit) -> EntityBuilder {
        let mut builder = mem::take(&mut self.entity_builder);
        builder.add(position);
        self.invoke_entity_spawn_callbacks(&mut builder, init);
        builder
    }

    /// Spawns an entity and returns its [`Entity`](ecs::Entity) handle.
    ///
    /// Also triggers necessary events, like `EntitySpawnEvent` and `PlayerJoinEvent`.
    pub fn spawn_entity(&mut self, mut builder: EntityBuilder) -> Entity {
        let entity = self.world.spawn(builder.build());
        self.entity_builder = builder;

        self.trigger_entity_spawn_events(entity);

        entity
    }

    fn invoke_entity_spawn_callbacks(&mut self, builder: &mut EntityBuilder, init: EntityInit) {
        let mut callbacks = mem::take(&mut self.entity_spawn_callbacks);
        for callback in &mut callbacks {
            callback(builder, &init);
        }
        self.entity_spawn_callbacks = callbacks;
    }

    fn trigger_entity_spawn_events(&mut self, entity: Entity) {
        self.world
            .insert_entity_event(entity, EntityCreateEvent)
            .unwrap();
        if self.world.get::<Player>(entity).is_ok() {
            self.world
                .insert_entity_event(entity, PlayerJoinEvent)
                .unwrap();
        }
    }

    /// Causes the given entity to be removed on the next tick.
    /// In the meantime, triggers `EntityRemoveEvent`.
    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.world.defer_despawn(entity);
        self.world.insert_entity_event(entity, EntityRemoveEvent)
    }

    /// Broadcasts a chat message to all entities with
    /// a `ChatBox` component (usually just players).
    pub fn broadcast_chat(&self, kind: ChatKind, message: impl Into<Text>) {
        let message = message.into();
        for (_, mailbox) in self.world.query::<&mut ChatBox>().iter() {
            mailbox.send(ChatMessage::new(kind, message.clone()));
        }
    }

    /// Utility method to send a message to an entity.
    pub fn send_message(&mut self, entity: Entity, message: ChatMessage) -> SysResult {
        let mut mailbox = self.world.get_mut::<ChatBox>(entity)?;
        mailbox.send(message);
        Ok(())
    }

    /// Gets the block at the given position.
    pub fn block(&self, pos: BlockPosition) -> Option<BlockId> {
        self.level.block_at(pos)
    }

    /// Sets the block at the given position.
    ///
    /// Triggers necessary `BlockChangeEvent`s.
    pub fn set_block(&mut self, pos: BlockPosition, block: BlockId) -> bool {
        let was_successful = self.level.set_block_at(pos, block);
        if was_successful {
            self.world.insert_event(BlockChangeEvent::single(pos));
        }
        was_successful
    }

    /// Fills the given chunk section (16x16x16 blocks).
    ///
    /// All blocks in the chunk section are overwritten with `block`.
    pub fn fill_chunk_section(
        &mut self,
        chunk_pos: ChunkPosition,
        section_y: usize,
        block: BlockId,
    ) -> bool {
        let mut chunk = match self.level.chunk_map().chunk_at_mut(chunk_pos) {
            Some(chunk) => chunk,
            None => return false,
        };

        let was_successful = chunk.fill_section(section_y + 1, block);

        if !was_successful {
            return false;
        }

        self.world
            .insert_event(BlockChangeEvent::fill_chunk_section(
                chunk_pos,
                section_y as u32,
            ));

        true
    }

    /// Breaks the block at the given position, propagating any
    /// necessary block updates.
    pub fn break_block(&mut self, pos: BlockPosition) -> bool {
        self.set_block(pos, BlockId::air())
    }
}

impl HasResources for Game {
    fn resources(&self) -> Arc<Resources> {
        Arc::clone(&self.resources)
    }
}

impl HasEcs for Game {
    fn ecs(&self) -> &Ecs {
        &self.world
    }

    fn ecs_mut(&mut self) -> &mut Ecs {
        &mut self.world
    }
}
