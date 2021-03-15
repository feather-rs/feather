use std::any::type_name;

use ahash::AHashMap;

use crate::{
    component::{Component, ComponentMeta, ComponentTypeId},
    entity::{Entities, EntityId},
    entity_builder::EntityBuilder,
    storage::SparseSetStorage,
};

#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("entity does not have a component of type {0}")]
    MissingComponent(&'static str),
    #[error(transparent)]
    MissingEntity(#[from] EntityDead),
}

#[derive(Debug, thiserror::Error)]
#[error("entity is dead or was unloaded")]
pub struct EntityDead;

/// The entity-component data structure.
///
/// An `Ecs` stores _components_ for _entities_.
///
/// This struct is equivalent to `World` in most ECS
/// libraries, but it has been renamed to `Ecs` to avoid
/// conflict with Minecraft's definition of a "world." (In
/// Feather, the `World` stores blocks, not entities.)
#[derive(Default)]
pub struct Ecs {
    components: AHashMap<ComponentTypeId, SparseSetStorage>,
    entities: Entities,
}

impl Ecs {
    /// Creates a new, empty ECS.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a component for an entity.
    ///
    /// Time complexity: O(1)
    pub fn get<T: Component>(&self, entity: EntityId) -> Result<&T, ComponentError> {
        let storage = self.storage_for::<T>()?;
        self.check_entity(entity)?;
        storage
            .get::<T>(entity.index())
            .ok_or_else(|| ComponentError::MissingComponent(type_name::<T>()))
    }

    /// Inserts a component for an entity.
    ///
    /// If the entity already has this component, then it
    /// is overriden.
    ///
    /// Time complexity: O(1)
    pub fn insert<T: Component>(
        &mut self,
        entity: EntityId,
        component: T,
    ) -> Result<(), EntityDead> {
        self.check_entity(entity)?;
        let storage = self.storage_or_insert_for::<T>();
        storage.insert(entity.index(), component);
        Ok(())
    }

    /// Creates a new entity with no components.
    ///
    /// Time complexity: O(1)
    pub fn spawn_empty(&mut self) -> EntityId {
        self.entities.allocate()
    }

    /// Creates a new entity and adds all components
    /// from `builder` to the entity.
    ///
    /// `builder` is reset and can be reused after this call.
    pub fn spawn(&mut self, builder: &mut EntityBuilder) -> EntityId {
        let entity = self.spawn_empty();

        for (component_meta, component) in builder.drain() {
            let storage = self.storage_or_insert_for_untyped(component_meta);
            unsafe {
                storage.insert_raw(entity.index(), component.as_ptr());
            }
        }

        builder.reset();

        entity
    }

    fn check_entity(&self, entity: EntityId) -> Result<(), EntityDead> {
        self.entities
            .check_generation(entity)
            .map_err(|_| EntityDead)
    }

    fn storage_for<T: Component>(&self) -> Result<&SparseSetStorage, ComponentError> {
        self.components
            .get(&ComponentTypeId::of::<T>())
            .ok_or_else(|| ComponentError::MissingComponent(type_name::<T>()))
    }

    fn storage_or_insert_for<T: Component>(&mut self) -> &mut SparseSetStorage {
        self.components
            .entry(ComponentTypeId::of::<T>())
            .or_insert_with(|| SparseSetStorage::new(ComponentMeta::of::<T>()))
    }

    fn storage_or_insert_for_untyped(
        &mut self,
        component_meta: ComponentMeta,
    ) -> &mut SparseSetStorage {
        self.components
            .entry(component_meta.type_id)
            .or_insert_with(|| SparseSetStorage::new(component_meta))
    }
}
