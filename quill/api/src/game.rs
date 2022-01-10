use std::marker::PhantomData;

use libcraft_core::{EntityKind, Position};
use libcraft_particles::Particle;

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
    #[allow(clippy::new_without_default)]
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

    /// Creates an empty [`EntityBuilder`](crate::EntityBuilder)
    /// to add entities to the ecs.
    ///
    /// The builder isn initialised without any components.
    pub fn create_empty_entity_builder(&self) -> EntityBuilder {
        let id = unsafe { quill_sys::entity_builder_new_empty() };

        EntityBuilder::new(id)
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
    pub fn create_entity_builder(&self, position: Position, entity: EntityKind) -> EntityBuilder {
        let entity_kind = bincode::serialize(&entity).expect("failed to serialize EntityKind");
        let position: &[u8] = bytemuck::cast_slice(std::slice::from_ref(&position));
        let id = unsafe {
            quill_sys::entity_builder_new(
                position.as_ptr().into(),
                entity_kind.as_ptr().into(),
                entity_kind.len() as u32,
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
    /// # let mut game: quill::Game = todo!();
    /// for (entity, (position, uuid)) in game.query::<(&Position, &Uuid)>() {
    ///    println!("Found an entity with position {:?} and UUID {}", position, uuid);
    /// }
    /// ```
    pub fn query<Q: Query>(&mut self) -> QueryIter<Q> {
        QueryIter::new()
    }

    /// Spawn a particle effect at the position
    ///
    /// # Example
    /// Spawn a flame particle at 0, 0, 0:
    /// ```no_run
    /// # let game: quill::Game = unreachable!();
    /// use quill::{Position, Particle, ParticleKind};
    ///
    /// let position = Position {x: 0.0, y: 0.0, z: 0.0, pitch: 0.0, yaw: 0.0};
    /// let particle = Particle {
    ///     kind: ParticleKind::Flame,
    ///     offset_x: 0.0,
    ///     offset_y: 0.0,
    ///     offset_z: 0.0,
    ///     count: 1,
    /// };
    ///
    /// game.spawn_particle(position, particle);
    /// ```
    pub fn spawn_particle(&self, position: Position, particle: Particle) {
        let mut entity_builder = self.create_empty_entity_builder();

        entity_builder.add(position);
        entity_builder.add(particle);
        entity_builder.finish();
    }

    /// Sends a custom packet to an entity.
    pub fn send_plugin_message(entity: EntityId, channel: &str, data: &[u8]) {
        let channel_ptr = channel.as_ptr().into();
        let data_ptr = data.as_ptr().into();
        unsafe {
            quill_sys::plugin_message_send(
                entity.0,
                channel_ptr,
                channel.len() as u32,
                data_ptr,
                data.len() as u32,
            )
        }
    }
}
