use std::{cell::RefCell, cell::RefMut, mem, rc::Rc, sync::Arc};

use base::World;
use ecs::{Ecs, Entity, EntityBuilder, EventBus, HasResources, Resources};

use crate::{entity::player::Player, events::PlayerJoinEvent};

/// Stores the entire state of a Minecraft game.
///
/// This contains:
/// * A [`World`](base::World) containing chunks and blocks.
/// * An [`Ecs`](ecs::Ecs) containing entities.
/// * A [`Resources`](ecs::Resources) containing additional, user-defined data.
/// * An event bus for event handling.
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

    /// Event bus for event handling.
    event_bus: Rc<RefCell<EventBus<Game>>>,

    entity_builder: EntityBuilder,
}

impl Game {
    /// Creates a new, empty `Game`.
    pub fn new() -> Self {
        Self {
            world: World::new(),
            ecs: Ecs::new(),
            resources: Arc::new(Resources::new()),
            event_bus: Rc::new(RefCell::new(EventBus::new())),
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

    fn trigger_entity_spawn_events(&mut self, entity: Entity) {
        if self.ecs.get::<Player>(entity).is_ok() {
            self.trigger_event(PlayerJoinEvent { player: entity });
        }
    }

    /// Gets the `EventBus` to register event handlers.
    pub fn event_bus(&self) -> RefMut<EventBus<Game>> {
        self.event_bus.borrow_mut()
    }

    /// Triggers an event, invoking all event handlers for
    /// this event type.
    ///
    /// Event handlers may make arbitrary mutations to the `Game`.
    /// After calling this method, you should not assume anything
    /// about the game state, e.g. that an entity still exists.
    pub fn trigger_event<E: 'static>(&mut self, event: E) {
        let event_bus = Rc::clone(&self.event_bus);
        event_bus.borrow().handle(self, &event);
    }
}

impl HasResources for Game {
    fn resources(&self) -> Arc<Resources> {
        Arc::clone(&self.resources)
    }
}
