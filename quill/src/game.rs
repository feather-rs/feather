use tokio::runtime;
use vane::{Entities, Entity, EntityBuilder, Resources};

use crate::threadpool::ThreadPool;

/// A plugin's primary interface to interacting with the server state.
///
/// A `Game` contains all worlds, blocks, entities, and resources
/// in the server.
///
/// # Asynchronous work
/// Often a plugin needs to run some task asynchronously, on a separate
/// thread, for performance reasons. The server contains two objects
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
