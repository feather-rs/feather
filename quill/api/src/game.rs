use std::marker::PhantomData;

use libcraft_blocks::BlockState;
use libcraft_core::{BlockPosition, ChunkPosition, Position, CHUNK_HEIGHT};
use libcraft_particles::Particle;
use quill_common::entity_init::EntityInit;
use quill_common::Component;

use crate::{
    query::{Query, QueryIter},
    EntityBuilder,
};
use crate::{Entity, EntityId};

/// Error returned when getting or setting a block fails.
#[derive(Debug, thiserror::Error)]
pub enum BlockAccessError {
    #[error("the block's Y coordinate is outside the range [0, 256)")]
    YOutOfBounds,
    #[error("the block's chunk is not loaded")]
    ChunkNotLoaded,
}

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
    pub fn create_entity_builder(&self, position: Position, entity: EntityInit) -> EntityBuilder {
        let entity_init = bincode::serialize(&entity).expect("failed to serialize EntityInit");
        let position: &[u8] = bytemuck::cast_slice(std::slice::from_ref(&position));
        let id = unsafe {
            quill_sys::entity_builder_new(
                position.as_ptr().into(),
                entity_init.as_ptr().into(),
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

    /// Gets the block at `pos`.
    ///
    /// This function returns an error if the block's
    /// chunk is not loaded. Unlike in Bukkit, calling this method
    /// will not cause chunks to be loaded.
    ///
    /// Mutating the returned [`BlockState`](libcraft_blocks::BlockState)
    /// will _not_ cause the block to be modified in the world. In other
    /// words, the `BlockState` is a copy, not a reference. To update
    /// the block, call [`Game::set_block`].
    pub fn block(&self, pos: BlockPosition) -> Result<BlockState, BlockAccessError> {
        check_y_bound(pos)?;

        let result = unsafe { quill_sys::block_get(pos.x, pos.y, pos.z) };

        result
            .get()
            .ok_or(BlockAccessError::ChunkNotLoaded)
            .map(|block_id| BlockState::from_id(block_id).expect("host gave invalid block ID"))
    }

    /// Sets the block at `pos`.
    ///
    /// This function returns an error if the block's
    /// chunk is not loaded. Unlike in Bukkit, calling this method
    /// will not cause chunks to be loaded.
    pub fn set_block(&self, pos: BlockPosition, block: BlockState) -> Result<(), BlockAccessError> {
        check_y_bound(pos)?;

        let was_successful = unsafe { quill_sys::block_set(pos.x, pos.y, pos.z, block.id()) };

        if was_successful {
            Ok(())
        } else {
            Err(BlockAccessError::ChunkNotLoaded)
        }
    }

    /// Efficiently overwrites all blocks in the given chunk section (16x16x16 blocks).
    ///
    /// All blocks in the chunk section are replaced with `block`.
    ///
    /// This function returns an error if the block's
    /// chunk is not loaded. Unlike in Bukkit, calling this method
    /// will not cause chunks to be loaded.
    pub fn fill_chunk_section(
        &self,
        chunk: ChunkPosition,
        section_y: u32,
        block: BlockState,
    ) -> Result<(), BlockAccessError> {
        check_section_y(section_y)?;

        let block_id = block.id();
        let was_successful =
            unsafe { quill_sys::block_fill_chunk_section(chunk.x, section_y, chunk.z, block_id) };

        if was_successful {
            Ok(())
        } else {
            Err(BlockAccessError::ChunkNotLoaded)
        }
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

    /// Inserts an event to the world.
    pub fn insert_event<T: Component>(&self, event: T) {
        let host_component = T::host_component();
        let bytes = event.to_cow_bytes();

        unsafe {
            quill_sys::add_event(host_component, bytes.as_ptr().into(), bytes.len() as u32);
        }
    }
}

fn check_y_bound(pos: BlockPosition) -> Result<(), BlockAccessError> {
    if pos.y < 0 || pos.y >= CHUNK_HEIGHT as i32 {
        Err(BlockAccessError::YOutOfBounds)
    } else {
        Ok(())
    }
}

fn check_section_y(section_y: u32) -> Result<(), BlockAccessError> {
    if section_y >= 16 {
        Err(BlockAccessError::YOutOfBounds)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_y_bound_in_bounds() {
        assert!(check_y_bound(BlockPosition::new(0, 0, 0)).is_ok());
        assert!(check_y_bound(BlockPosition::new(0, 255, 0)).is_ok());
    }

    #[test]
    fn check_y_bound_out_of_bounds() {
        assert!(matches!(
            check_y_bound(BlockPosition::new(0, -1, 0)),
            Err(BlockAccessError::YOutOfBounds)
        ));
        assert!(matches!(
            check_y_bound(BlockPosition::new(0, 256, 0)),
            Err(BlockAccessError::YOutOfBounds)
        ));
    }

    #[test]
    fn check_section_y_in_bounds() {
        assert!(check_section_y(0).is_ok());
        assert!(check_section_y(15).is_ok());
    }

    #[test]
    fn check_section_y_out_of_bounds() {
        assert!(matches!(
            check_section_y(16),
            Err(BlockAccessError::YOutOfBounds)
        ));
        assert!(matches!(
            check_section_y(u32::MAX),
            Err(BlockAccessError::YOutOfBounds)
        ));
    }
}
