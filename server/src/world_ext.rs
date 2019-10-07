use specs::{Component, Entity, World};

/// Extension trait on `World` with extra functions for convenience.
pub trait WorldExt {
    /// Retrieves a component for an entity.
    ///
    /// # Panics
    /// Panics if the component does not exist for this entity,
    /// or if the entity is dead.
    fn get<C: Component>(&self, entity: Entity) -> &C;
}

impl WorldExt for World {
    fn get<C: Component>(&self, entity: Entity) -> &C {
        use specs::WorldExt;
        self.read_component().get(entity).unwrap()
    }
}
