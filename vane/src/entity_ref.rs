use crate::{Component, ComponentError, EntityId, Ref, RefMut, World};

/// Convenient wrapper over an `EntityId` that
/// gives access to components.
pub struct EntityRef<'a> {
    entity: EntityId,
    world: &'a World,
}

impl<'a> EntityRef<'a> {
    pub(crate) fn new(entity: EntityId, world: &'a World) -> Self {
        Self { entity, world }
    }

    pub fn get<T: Component>(&self) -> Result<Ref<T>, ComponentError> {
        self.world.get(self.entity)
    }

    pub fn get_mut<T: Component>(&self) -> Result<RefMut<T>, ComponentError> {
        self.world.get_mut(self.entity)
    }
}
