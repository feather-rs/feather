use std::marker::PhantomData;

use quill_common::Component;

use crate::{Entity, EntityId};

/// Builder for an entity.
///
/// Created via [`Game::create_entity_builder`](crate::Game::create_entity_builder).
///
/// Add components to the entity with [`EntityBuilder::add`].
/// Finish building the entity with [`EntityBuilder::finish`].
#[derive(Debug)]
pub struct EntityBuilder {
    id: u32,
    _not_send_sync: PhantomData<*mut ()>,
}

impl EntityBuilder {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            _not_send_sync: PhantomData,
        }
    }

    /// Adds a component to the entity.
    ///
    /// If the builder already has this component,
    /// it is overriden.
    pub fn add<T: Component>(&mut self, component: T) -> &mut Self {
        let host_component = T::host_component();
        let bytes = component.to_cow_bytes();
        unsafe {
            quill_sys::entity_builder_add_component(
                self.id,
                host_component,
                bytes.as_ptr().into(),
                bytes.len() as u32,
            );
        }
        self
    }

    /// Adds a component to the entity and returns
    /// `self` for method chaining.
    ///
    /// If the builder already has this component,
    /// it is override.
    pub fn with<T: Component>(mut self, component: T) -> Self {
        self.add(component);
        self
    }

    /// Finishes building the entity and spawns it.
    ///
    /// Returns the built entity.
    pub fn finish(self) -> Entity {
        let id = unsafe { quill_sys::entity_builder_finish(self.id) };
        Entity::new(EntityId(id))
    }
}
