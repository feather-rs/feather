//! A lightweight ECS wrapper tailored to Feather's needs.
//!
//! This is implemented as a wrapper around the Bevy Engine's fork of the
//!  `hecs` crate, but we've made some interface changes:
//! * A system framework has been implemented, with systems written as plain functions and
//! executed sequentialy.
//! * `World` is renamed to `Ecs` so as to avoid conflict with Minecraft's concept of worlds.
//! * We add support for events based on components.
//!
//! This wrapper library exists in case we need additional features in the ECS. If necessary,
//! we can change the backend crate or fork it as needed, without refactoring the rest of the codebase.

use event::EventTracker;
use hecs::{Component, DynamicBundle, Query, World};

#[doc(inline)]
pub use hecs::{
    BuiltEntity, ComponentError, Entity, EntityBuilder, MissingComponent, NoSuchEntity,
    QueryBorrow, Ref, RefMut,
};

mod system;
pub use system::{GroupBuilder, HasEcs, HasResources, SysResult, SystemExecutor};

mod resources;
pub use resources::{ResourceError, Resources};

mod event;

/// Stores entities and their components. This is a wrapper
/// around `hecs::World` with a slightly changed interface
/// and support for events.
///
/// # Events
/// This struct supports _events_ by adding components to entities.
/// For example, the `EntityDamageEvent` is triggered whenever an
/// entity takes damage. What happens next:
/// 1. The system that damaged the entity adds `EntityDamageEvent` as a component
/// to the entity.
/// 2. All systems get a chance to observe that event by calling [`Ecs::query`]
/// using the `EntityDamageEvent` type.
/// 3. When the system that triggered the event runs again, the component
/// is automatically removed.
///
/// This ensures that each event is observed exactly once by each system.
///
/// Events can either be associated with an entity—in which case they
/// are added as a component to the entity—or they can be standalone.
/// For example, `BlockChangeEvent` is not related to any specific
/// entity. These standalone events are entities with only one component—the event.
#[derive(Default)]
pub struct Ecs {
    world: World,
    event_tracker: EventTracker,
}

impl Ecs {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `hecs::World`. Should be used with caution.
    pub fn inner(&self) -> &World {
        &self.world
    }

    pub fn inner_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Spawns an entity with the provided components.
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.world.spawn(components)
    }

    /// Returns an `EntityRef` for an entity.
    pub fn entity(&self, entity: Entity) -> Result<EntityRef, NoSuchEntity> {
        self.world.entity(entity).map(EntityRef)
    }

    /// Gets a component of an entity.
    pub fn get<T: Component>(&self, entity: Entity) -> Result<Ref<T>, ComponentError> {
        self.world.get(entity)
    }

    /// Mutably gets a component of an entity.
    pub fn get_mut<T: Component>(&self, entity: Entity) -> Result<RefMut<T>, ComponentError> {
        self.world.get_mut(entity)
    }

    /// Adds a component to an entity.
    ///
    /// Do not use this function to add events. Use [`insert_event`]
    /// instead.
    pub fn insert(
        &mut self,
        entity: Entity,
        component: impl Component,
    ) -> Result<(), NoSuchEntity> {
        self.world.insert_one(entity, component)
    }

    /// Creates an event not related to any entity. Use
    /// `insert_entity_event` for events regarding specific
    /// entities (`PlayerJoinEvent`, `EntityDamageEvent`, etc...)
    pub fn insert_event<T: Component>(&mut self, event: T) {
        let entity = self.world.spawn((event,));
        self.event_tracker.insert_event(entity);
    }

    /// Adds an event component to an entity and schedules
    /// it to be removed immeditately before the current system
    /// runs again. Thus, all systems have exactly one chance
    /// to observe the event before it is dropped.
    pub fn insert_entity_event<T: Component>(
        &mut self,
        entity: Entity,
        event: T,
    ) -> Result<(), NoSuchEntity> {
        self.insert(entity, event)?;
        self.event_tracker.insert_entity_event::<T>(entity);
        Ok(())
    }

    /// Removes a component from an entit and returns it.
    pub fn remove<T: Component>(&mut self, entity: Entity) -> Result<T, ComponentError> {
        self.world.remove_one(entity)
    }

    /// Returns an iterator over all entities that match a query parameter.
    pub fn query<Q: Query>(&self) -> QueryBorrow<Q> {
        self.world.query()
    }

    /// Sets the index of the currently executing system,
    /// used for event tracking.
    pub fn set_current_system_index(&mut self, index: usize) {
        self.event_tracker.set_current_system_index(index);
    }

    /// Should be called before each system runs.
    pub fn remove_old_events(&mut self) {
        self.event_tracker.remove_old_events(&mut self.world);
    }
}

/// Allows access to all components of a single entity.
pub struct EntityRef<'a>(hecs::EntityRef<'a>);

impl<'a> EntityRef<'a> {
    /// Borrows the component of type `T` from this entity.
    pub fn get<T: Component>(&self) -> Result<Ref<'a, T>, ComponentError> {
        self.0
            .get()
            .ok_or_else(|| ComponentError::MissingComponent(MissingComponent::new::<T>()))
    }

    /// Uniquely borrows the component of type `T` from this entity.
    pub fn get_mut<T: Component>(&self) -> Result<RefMut<'a, T>, ComponentError> {
        self.0
            .get_mut()
            .ok_or_else(|| ComponentError::MissingComponent(MissingComponent::new::<T>()))
    }
}
