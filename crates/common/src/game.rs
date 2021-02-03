use std::{mem, sync::Arc};

use ecs::{Ecs, Entity, EntityBuilder, HasEcs, HasResources, NoSuchEntity, Resources};

use crate::{
    entity::player::Player,
    events::{EntityRemoveEvent, PlayerJoinEvent},
    World,
};

/// Stores the entire state of a Minecraft game.
///
/// This contains:
/// * A [`World`](base::World) containing chunks and blocks.
/// * An [`Ecs`](ecs::Ecs) containing entities.
/// * A [`Resources`](ecs::Resources) containing additional, user-defined data.
///
/// `feather-common` provides `Game` methods for actions such
/// as "drop item" or "kill entity." These high-level methods
/// should be preferred over raw interaction with the ECS.
pub struct Game {
    /// Contains chunks and blocks.
    pub world: World,
    /// Contains entities, including players.
    pub ecs: Ecs,

    /// User-defined resources.
    ///
    /// Stored in an `Arc` for borrow-checker purposes.
    pub resources: Arc<Resources>,

    entity_builder: EntityBuilder,
}

impl Game {
    /// Creates a new, empty `Game`.
    pub fn new() -> Self {
        Self {
            world: World::new(),
            ecs: Ecs::new(),
            resources: Arc::new(Resources::new()),
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

    /// Creates a new `EntityBuilder`.
    pub fn create_entity_builder(&mut self) -> EntityBuilder {
        mem::take(&mut self.entity_builder)
    }

    /// Spawns an entity and returns its [`Entity`](ecs::Entity) handle.
    ///
    /// Also triggers necessary events, like `EntitySpawnEvent` and `PlayerJoinEvent`.
    pub fn spawn_entity(&mut self, mut builder: EntityBuilder) -> Entity {
        let entity = self.ecs.spawn(builder.build());
        self.entity_builder = builder;
        self.trigger_entity_spawn_events(entity);
        entity
    }

    /// Causes the given entity to be removed on the next tick.
    /// In the meantime, triggers `EntityDespawnEvent` and `PlayerLeaveEvent`.
    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.ecs.defer_despawn(entity);
        self.ecs.insert_entity_event(entity, EntityRemoveEvent)
    }

    fn trigger_entity_spawn_events(&mut self, entity: Entity) {
        if self.ecs.get::<Player>(entity).is_ok() {
            self.ecs
                .insert_entity_event(entity, PlayerJoinEvent)
                .unwrap();
        }
    }
}

impl HasResources for Game {
    fn resources(&self) -> Arc<Resources> {
        Arc::clone(&self.resources)
    }
}

impl HasEcs for Game {
    fn ecs(&self) -> &Ecs {
        &self.ecs
    }

    fn ecs_mut(&mut self) -> &mut Ecs {
        &mut self.ecs
    }
}
