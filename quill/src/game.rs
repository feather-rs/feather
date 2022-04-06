use vane::{Entities, Entity, EntityBuilder, Resources};

/// A plugin's primary interface to interacting with the game state.
///
/// A `Game` contains all worlds, blocks, entities, and resources
/// in the server. It also provides convenience methods.
///
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
    ///
    /// The provided `builder` can be reused to spawn further entities.
    /// (Note that all components stored in the builder are flushed.)
    fn spawn_entity(&mut self, builder: &mut EntityBuilder) -> Entity;

    /// Queues an entity to be removed on the next tick.
    ///
    /// You should prefer this method over removing entities
    /// directly on the ECS, as it allows other plugins and the server
    /// code to react to the `EntityRemoveEvent`.
    fn queue_remove_entity(&mut self, entity: Entity);
}
