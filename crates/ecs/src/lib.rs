//! A lightweight ECS wrapper tailored to Feather's needs.
//!
//! This is implemented as a wrapper around the Bevy Engine's fork of the
//!  `hecs` crate, but we've made some interface changes:
//! * A system framework has been implemented, with systems written as plain functions and
//! executed sequentialy.
//! * `World` is renamed to `Ecs` so as to avoid conflict with Minecraft's concept of worlds.
//!
//! This wrapper library exists in case we need additional features in the ECS. If necessary,
//! we can change the backend crate or fork it as needed, without refactoring the rest of the codebase.

use hecs::{Component, DynamicBundle, Query, World};

#[doc(inline)]
pub use hecs::{
    BuiltEntity, ComponentError, Entity, EntityBuilder, MissingComponent, NoSuchEntity,
    QueryBorrow, Ref, RefMut,
};

mod system;
pub use system::{GroupBuilder, HasResources, SysResult, SystemExecutor};

mod event;
pub use event::{EventBus, HandlerGroupBuilder};

pub use resources::Resources;

/// Stores entities and their components. This is a wrapper
/// around `hecs::World` with a slightly changed interface.
#[derive(Default)]
pub struct Ecs(World);

impl Ecs {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `hecs::World`. Should be used with caution.
    pub fn inner(&self) -> &World {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut World {
        &mut self.0
    }

    /// Spawns an entity with the provided components.
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.0.spawn(components)
    }

    /// Returns an `EntityRef` for an entity.
    pub fn entity(&self, entity: Entity) -> Result<EntityRef, NoSuchEntity> {
        self.0.entity(entity).map(EntityRef)
    }

    /// Gets a component of an entity.
    pub fn get<T: Component>(&self, entity: Entity) -> Result<Ref<T>, ComponentError> {
        self.0.get(entity)
    }

    /// Mutably gets a component of an entity.
    pub fn get_mut<T: Component>(&self, entity: Entity) -> Result<RefMut<T>, ComponentError> {
        self.0.get_mut(entity)
    }

    /// Returns an iterator over all entities that match a query parameter.
    pub fn query<Q: Query>(&self) -> QueryBorrow<Q> {
        self.0.query()
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
