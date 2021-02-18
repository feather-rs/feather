use std::marker::PhantomData;

use libcraft_core::Position;
use quill_common::entity_init::EntityInit;

use crate::{
    query::{Query, QueryIter},
    EntityBuilder,
};
use crate::{Entity, EntityId};

/// Error returned from [`Game::entity`] if the entity
/// did not exist.
#[derive(Debug, thiserror::Error)]
#[error("entity no longer exists - they either died or were unloaded")]
pub struct EntityRemoved;

/// Provides access to the server's game state for a single world.
///
/// Includes entities, blocks, chunks, etc. All interaction with
/// the game happens through this struct.
///
/// A `Game` is passed to systems when they run.
#[derive(Debug)]
pub struct Game {
    _not_send_sync: PhantomData<*mut ()>,
}

impl Game {
    /// For Quill internal use only. Do not call.
    #[doc(hidden)]
    pub fn new() -> Self {
        Self {
            _not_send_sync: PhantomData,
        }
    }

    /// Gets an [`Entity`] from its [`EntityId`].
    ///
    /// Returns `None` if the entity no longer exists. This
    /// could be the case if:
    /// * The entity has been unloaded (and possibly saved to disk)
    /// * The entity has died
    pub fn entity(&self, id: EntityId) -> Result<Entity, EntityRemoved> {
        unsafe {
            if !quill_sys::entity_exists(id.0) {
                return Err(EntityRemoved);
            }
        }
        Ok(Entity::new(id))
    }

    /// Creates an [`EntityBuilder`](crate::EntityBuilder)
    /// to spawn an entity at the given position.
    ///
    /// The builder is initialized with the default components
    /// for the given `EntityInit`. The default components
    /// include (at least):
    /// * Position`
    /// * `Uuid`
    /// * `EntityType`
    /// * `Velocity` (set to zero)
    /// * the marker component for this entity
    #[must_use = "call `finish` on an EntityBuilder to spawn the entity"]
    pub fn create_entity_builder(&self, position: Position, entity: EntityInit) -> EntityBuilder {
        let entity_init = bincode::serialize(&entity).expect("failed to serialize EntityInit");
        let position = bytemuck::cast_slice(std::slice::from_ref(&position));
        let id = unsafe {
            quill_sys::entity_builder_new(
                position.as_ptr(),
                entity_init.as_ptr(),
                entity_init.len() as u32,
            )
        };
        EntityBuilder::new(id)
    }

    /// Returns an iterator over all entities
    /// with the given components.
    ///
    /// # Example
    /// Iterate over all entities with positions and UUIDs:
    /// ```no_run
    /// use quill::{Position, Uuid};
    /// # let game: quill::Game = todo!();
    /// for (entity, (position, uuid)) in game.query::<(&Position, &Uuid)>() {
    ///    println!("Found an entity with position {:?} and UUID {}", position, uuid);
    /// }
    /// ```
    pub fn query<Q: Query>(&self) -> QueryIter<Q> {
        QueryIter::new()
    }
}
