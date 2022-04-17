use std::{any::type_name, borrow::Cow, iter, marker::PhantomData, mem};

use ahash::AHashMap;
use itertools::Either;

use crate::{
    bundle::ComponentBundle,
    bus::{Bus, BusReceiver},
    component::{Component, ComponentMeta},
    entity::{Entity, EntityIds},
    entity_builder::EntityBuilder,
    entity_ref::EntityRef,
    event::EventTracker,
    query::{QueryDriverIter, QueryTuple},
    storage::SparseSetStorage,
    BorrowError, QueryDriver, Ref, RefMut,
};

pub use self::components::Components;

mod components;

#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("entity does not have a component of type {0}")]
    MissingComponent(&'static str),
    #[error(transparent)]
    MissingEntity(#[from] EntityDead),
    #[error(transparent)]
    BorrowConflict(#[from] BorrowError),
}

#[derive(Debug, thiserror::Error)]
#[error("entity is dead or was unloaded")]
pub struct EntityDead;

/// The entity-component data structure.
///
/// An `Entities` stores _components_ for _entities_.
#[derive(Default)]
pub struct Entities {
    components: Components,
    entity_ids: EntityIds,
    event_tracker: EventTracker,
    pub(crate) bus_receiver: BusReceiver,
}

impl Entities {
    /// Creates a new, empty ECS.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a [`Bus`](crate::Bus) to execute actions on the `Entities`.
    pub fn bus(&self) -> Bus {
        self.bus_receiver.bus()
    }

    /// Returns whether an entity still exists.
    pub fn contains(&self, entity: Entity) -> bool {
        self.check_entity(entity).is_ok()
    }

    /// Gets a component for an entity.
    ///
    /// Borrow checking is dynamic. If a mutable reference to the
    /// component is still active, this function will return an error.
    ///
    /// Note that at most 254 `Ref`s can exist for a given component. Attempting
    /// to acquire more will result in anerror.
    ///
    /// Time complexity: O(1)
    pub fn get<T: Component>(&self, entity: Entity) -> Result<Ref<T>, ComponentError> {
        self.check_entity(entity)?;
        self.components.get(entity.index())
    }

    /// Mutably gets a component for an entity.
    ///
    /// Borrow checking is dynamic. If any references to the
    /// component are still alive, this function will return an error.
    ///
    /// Time complexity: O(1)
    pub fn get_mut<T: Component>(&self, entity: Entity) -> Result<RefMut<T>, ComponentError> {
        self.check_entity(entity)?;
        self.components.get_mut(entity.index())
    }

    /// Determines whether the given entity has the given component.
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        self.get::<T>(entity).is_ok()
    }

    /// Gets an `EntityRef` to the given entity.
    pub fn entity(&self, entity_id: Entity) -> Result<EntityRef, EntityDead> {
        self.check_entity(entity_id)?;
        Ok(EntityRef::new(entity_id, self))
    }

    /// Inserts a component for an entity.
    ///
    /// If the entity already has this component, then it
    /// is overriden.
    ///
    /// Time complexity: O(1)
    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) -> Result<(), EntityDead> {
        self.check_entity(entity)?;
        self.components.insert(entity.index(), component);
        self.get_mut::<T>(entity)
            .expect("component not inserted")
            .on_inserted(self, entity);
        Ok(())
    }

    /// Removes a component from an entity.
    ///
    /// Returns `Err` if the entity does not exist
    /// or if it did not have the component.
    ///
    /// Time complexity: O(1)
    pub fn remove<T: Component>(&mut self, entity: Entity) -> Result<(), ComponentError> {
        self.check_entity(entity)?;
        self.components.remove::<T>(entity.index())
    }

    /// Creates a new entity with no components.
    ///
    /// Time complexity: O(1)
    pub fn spawn_empty(&mut self) -> Entity {
        self.entity_ids.allocate()
    }

    /// Creates a new entity and adds all components
    /// from `builder` to the entity.
    ///
    /// `builder` is reset and can be reused after this call.
    ///
    /// Time complexity: O(n) with respect to the number of components in `builder`.
    pub fn spawn_builder(&mut self, builder: &mut EntityBuilder) -> Entity {
        let entity = self.spawn_empty();

        for (component_meta, component) in builder.drain() {
            unsafe {
                let ptr = self.components.insert_raw(
                    entity.index(),
                    component_meta.clone(),
                    component.as_ptr(),
                );
                (component_meta.on_inserted_fn)(ptr, self, entity);
            }
        }

        builder.reset();

        entity
    }

    /// Creates a new entity using a `ComponentBundle`, i.e.,
    /// a tuple of components.
    ///
    /// Time complexity: O(n) with respect to the number of components in `bundle`.
    pub fn spawn_bundle(&mut self, bundle: impl ComponentBundle) -> Entity {
        let entity = self.spawn_empty();

        bundle.add_to_entity(self, entity);

        entity
    }

    /// Despawns an entity. Future access to the entity
    /// will result in `EntityDead`
    ///
    /// Time complexity: O(n) with respect to the total number of components
    /// stored in this ECS.
    pub fn despawn(&mut self, entity: Entity) -> Result<(), EntityDead> {
        self.entity_ids.deallocate(entity).map_err(|_| EntityDead)?;

        // PERF: could we somehow optimize this linear search
        // by only checking storages containing the entity?
        for (_, storage) in self.components.storages_mut() {
            storage.remove(entity.index());
        }

        Ok(())
    }

    /// Defers removing an entity until before the next time this system
    /// runs, allowing it to be observed by systems one last time.
    pub fn defer_despawn(&mut self, entity: Entity) {
        // a bit of a hack - but this will change once
        // hecs allows taking out components of a despawned entity
        self.event_tracker.insert_event(entity);
    }

    /// Queries for all entities that have the given set of components.
    ///
    /// Returns an iterator over tuples of `(entity, components)`.
    pub fn query<'w, 'q, Q: QueryTuple<'w>>(&'w self) -> Query<'w, 'q, Q> {
        let sparse_sets = Q::sparse_sets(&self.components);
        let sparse_set_refs: Vec<_> = sparse_sets.iter().map(|set| set.to_ref()).collect();
        let dense_indices = Q::dense_indices();

        let driver = QueryDriver::new(Cow::Owned(sparse_set_refs), Cow::Owned(dense_indices));

        Query {
            driver,
            sparse_sets,
            entities: &self.entity_ids,
            _marker: PhantomData,
        }
    }

    /// Iterates over all alive entities in this world.
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entity_ids.iter()
    }

    /// Creates an event not related to any entity. Use
    /// `insert_entity_event` for events regarding specific
    /// entities (`PlayerJoinEvent`, `EntityDamageEvent`, etc...)
    pub fn insert_event<T: Component>(&mut self, event: T) {
        let entity = self.spawn_bundle((event,));
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
    ) -> Result<(), EntityDead> {
        self.insert(entity, event)?;
        self.event_tracker.insert_entity_event::<T>(entity);
        Ok(())
    }

    /// Sets the index of the currently executing system,
    /// used for event tracking.
    pub fn set_current_system_index(&mut self, index: usize) {
        self.event_tracker.set_current_system_index(index);
    }

    /// Should be called before each system runs.
    pub fn remove_old_events(&mut self) {
        let mut tracker = mem::take(&mut self.event_tracker);
        tracker.remove_old_events(self);
        self.event_tracker = tracker;
    }

    fn check_entity(&self, entity: Entity) -> Result<(), EntityDead> {
        self.entity_ids
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
    entities: &'w EntityIds,
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
    entities: &'w EntityIds,
    _marker: PhantomData<Q>,
}

impl<'w, 'q, Q> Iterator for QueryIter<'w, 'q, Q>
where
    Q: QueryTuple<'w>,
{
    type Item = (Entity, Q::Output);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.driver.next()?;

        let components = unsafe { Q::make_output(self.sparse_sets, item.dense_indices) };

        let entity = self.entities.get(item.sparse_index);

        Some((entity, components))
    }
}
