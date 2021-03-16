use std::{any::type_name, borrow::Cow, iter, marker::PhantomData};

use ahash::AHashMap;
use itertools::Either;

use crate::{
    bundle::ComponentBundle,
    component::{Component, ComponentMeta},
    entity::{Entities, EntityId},
    entity_builder::EntityBuilder,
    query::{QueryDriverIter, QueryTuple},
    storage::SparseSetStorage,
    QueryDriver,
};

pub use self::components::Components;

mod components;

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
    components: Components,
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
        self.check_entity(entity)?;
        self.components.get(entity.index())
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
        self.components.insert(entity.index(), component);
        Ok(())
    }

    /// Removes a component from an entity.
    ///
    /// Returns `Err` if the entity does not exist
    /// or if it did not have the component.
    ///
    /// Time complexity: O(1)
    pub fn remove<T: Component>(&mut self, entity: EntityId) -> Result<(), ComponentError> {
        self.check_entity(entity)?;
        self.components.remove::<T>(entity.index())
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
    ///
    /// Time complexity: O(n) with respect to the number of components in `builder`.
    pub fn spawn_builder(&mut self, builder: &mut EntityBuilder) -> EntityId {
        let entity = self.spawn_empty();

        for (component_meta, component) in builder.drain() {
            unsafe {
                self.components
                    .insert_raw(entity.index(), component_meta, component.as_ptr());
            }
        }

        builder.reset();

        entity
    }

    /// Creates a new entity using a `ComponentBundle`, i.e.,
    /// a tuple of components.
    ///
    /// Time complexity: O(n) with respect to the number of components in `bundle`.
    pub fn spawn_bundle(&mut self, bundle: impl ComponentBundle) -> EntityId {
        let entity = self.spawn_empty();

        bundle.add_to_entity(self, entity);

        entity
    }

    /// Despawns an entity. Future access to the entity
    /// will result in `EntityDead`.
    ///
    /// Time complexity: O(n) with respect to the total number of components
    /// stored in this ECS.
    pub fn despawn(&mut self, entity: EntityId) -> Result<(), EntityDead> {
        self.entities.deallocate(entity).map_err(|_| EntityDead)?;

        // PERF: could we somehow optimize this linear search
        // by only checking storages containing the entity?
        for (_, storage) in self.components.storages_mut() {
            storage.remove(entity.index());
        }

        Ok(())
    }

    /// Queries for all entities that have the given set of components.
    ///
    /// Returns an iterator over tuples of `(entity, components)`.
    pub fn query<'w, 'q, Q: QueryTuple<'w>>(&'w self) -> Query<'w, 'q, Q> {
        let sparse_sets = Q::sparse_sets(&self.components).unwrap_or_else(|| todo!());
        let sparse_set_refs: Vec<_> = sparse_sets.iter().map(|set| set.to_ref()).collect();
        let dense_indices = Q::dense_indices();

        let driver = QueryDriver::new(Cow::Owned(sparse_set_refs), Cow::Owned(dense_indices));

        Query {
            driver,
            sparse_sets,
            entities: &self.entities,
            _marker: PhantomData,
        }
    }

    fn check_entity(&self, entity: EntityId) -> Result<(), EntityDead> {
        self.entities
            .check_generation(entity)
            .map_err(|_| EntityDead)
    }
}

/// An iterator over a statically-typed query.
///
/// Call [`iter`] to iterate over the items.
pub struct Query<'w, 'q, Q> {
    driver: QueryDriver<'w, 'q>,
    sparse_sets: Vec<&'w SparseSetStorage>,
    entities: &'w Entities,
    _marker: PhantomData<Q>,
}

impl<'w, 'q, Q> Query<'w, 'q, Q>
where
    Q: QueryTuple<'w>,
{
    pub fn iter(&'q mut self) -> QueryIter<'w, 'q, Q> {
        QueryIter {
            driver: self.driver.iter(),
            sparse_sets: &self.sparse_sets,
            entities: self.entities,
            _marker: self._marker,
        }
    }
}

pub struct QueryIter<'w, 'q, Q> {
    driver: QueryDriverIter<'w, 'q>,
    sparse_sets: &'q [&'w SparseSetStorage],
    entities: &'w Entities,
    _marker: PhantomData<Q>,
}

impl<'w, 'q, Q> Iterator for QueryIter<'w, 'q, Q>
where
    Q: QueryTuple<'w>,
{
    type Item = (EntityId, Q::Output);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.driver.next()?;

        let components = unsafe { Q::make_output(self.sparse_sets, item.dense_indices) };

        let entity = self.entities.get(item.sparse_index);

        Some((entity, components))
    }
}
