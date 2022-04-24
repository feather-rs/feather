//! A plugin's primary interface to interacting with the server state.

use std::cell::{Ref, RefMut};

use tokio::runtime;
use vane::{Entities, Entity, EntityBuilder, Resources};

use crate::{
    saveload::{worldgen::WorldGeneratorFactory, WorldSourceFactory},
    threadpool::ThreadPool,
    world::WorldDescriptor,
    World, WorldId,
};

/// A plugin's primary interface to interacting with the server state.
///
/// A `Game` contains all worlds, blocks, entities, and resources
/// in the server.
///
/// # Asynchronous work
/// Often a plugin needs to run some task asynchronously, on a separate
/// thread, for performance reasons. The server contains two facilities
/// to assist in this:
/// * A Tokio runtime for running asynchronous IO work.
/// * A thread pool for compute tasks.
///
/// Do not spawn heavy computation tasks on the Tokio runtime, or IO
/// work on the thread pool. You'll block other tasks from running.
///
/// # Dynamic dispatch
/// Typically you'll pass around an `&dyn mut Game` - i.e., a dynamically-
/// dispatched reference to some implementation of this trait. The actual
/// struct that implements this trait lives in Feather itself and is not
/// exposed to plugins.
pub trait Game: 'static {
    /// Gets a reference to the ECS that contains all entities.
    fn ecs(&self) -> &Entities;

    /// Gets a mutable reference to the ECS that contains all entities.
    fn ecs_mut(&mut self) -> &mut Entities;

    /// Gets the `Resources` stored in the server.
    fn resources(&self) -> &Resources;

    /// Mutably gets the `Resources` stored in the server.
    fn resources_mut(&mut self) -> &mut Resources;

    /// Creates a new `World` using the given world descriptor.
    ///
    /// Note that the world will not persist in the `Game` after
    /// a server restart. You need to add back the world each time
    /// the server starts, using the same ID.
    fn create_world(&mut self, desc: WorldDescriptor);

    /// Registers a `WorldSourceFactory` that can be referenced in the server config file.
    fn register_world_source_factory(&mut self, name: &str, factory: Box<dyn WorldSourceFactory>);

    /// Registers a `WorldGeneratorFactory` that can be referenced in the server config file.
    fn register_world_generator_factory(
        &mut self,
        name: &str,
        factory: Box<dyn WorldGeneratorFactory>,
    );

    /// Gets a `WorldSourceFactory` by its name.
    fn world_source_factory(
        &self,
        name: &str,
    ) -> Result<&dyn WorldSourceFactory, WorldSourceFactoryNotFound>;

    /// Gets a `WorldGeneratorFactory` by its name.
    fn world_generator_factory(
        &self,
        name: &str,
    ) -> Result<&dyn WorldGeneratorFactory, WorldGeneratorFactoryNotFound>;

    /// Removes the world with the given ID.
    ///
    /// The world will no longer be accessible, but its chunks
    /// are kept in memory until they have all been saved,
    /// even if the server starts to shut down.
    fn remove_world(&mut self, id: WorldId) -> Result<(), WorldNotFound>;

    /// Gets a reference to the world with the given ID.
    fn world(&self, id: WorldId) -> Result<Ref<dyn World>, WorldNotFound>;

    /// Gets a mutable reference to the world with the given ID.
    fn world_mut(&self, id: WorldId) -> Result<RefMut<dyn World>, WorldNotFound>;

    /// Sets the default world.
    ///
    /// # Panics
    /// Panics if `id` does not reference a valid world.
    fn set_default_world(&mut self, id: WorldId);

    /// Gets the ID of the main/default world.
    fn default_world_id(&self) -> WorldId;

    /// Gets a reference to the default world.
    fn default_world(&self) -> Ref<dyn World> {
        self.world(self.default_world_id())
            .expect("default world does not exist?")
    }

    /// Gets a mutable reference to the default world.
    fn default_world_mut(&self) -> RefMut<dyn World> {
        self.world_mut(self.default_world_id())
            .expect("default world does not exist?")
    }

    /// Spawns a new entity.
    ///
    /// This method will correctly trigger events related
    /// to the entity spawn. Avoid spawning entities
    /// directly on the ECS, as those events will not trigger
    /// and thus the entities won't show on clients.
    fn spawn_entity(&mut self, builder: EntityBuilder) -> Entity;

    /// Queues an entity to be removed on the next tick.
    ///
    /// You should prefer this method over removing entities
    /// directly on the ECS, as it allows other plugins and the server
    /// code to react to the `EntityRemoveEvent`.
    fn queue_remove_entity(&mut self, entity: Entity);

    /// Gets a handle to the multithreaded Tokio runtime shared by the server and all plugins.
    fn tokio_runtime(&self) -> runtime::Handle;

    /// Gets a handle to the thread pool used for asynchronous
    /// compute work.
    fn compute_pool(&self) -> &ThreadPool;
}

#[derive(Debug, thiserror::Error)]
#[error("world with ID {0} was not found")]
pub struct WorldNotFound(pub WorldId);

#[derive(Debug, thiserror::Error)]
#[error("world source factory '{0}' was not found")]
pub struct WorldSourceFactoryNotFound(pub String);

#[derive(Debug, thiserror::Error)]
#[error("world generator factory '{0}' was not found")]
pub struct WorldGeneratorFactoryNotFound(pub String);
