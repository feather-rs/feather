use std::marker::PhantomData;

use quill_common::Component;

/// Unique internal ID of an entity.
///
/// Can be passed to [`Game::entity`] to get an [`Entity`]
/// handle.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(pub(crate) quill_common::EntityId);

/// Error returned by [`Entity::get`] when
/// the entity is missing a component.
#[derive(Debug, thiserror::Error)]
#[error("entity does not have component of type {0}")]
pub struct MissingComponent(&'static str);

/// A handle to an entity.
///
/// Allows access to the entity's components, like
/// position and UUID.
///
/// Use [`Game::entity`] to get an `Entity` instance.
///
/// An `Entity` be sent or shared between threads. However,
/// an [`EntityId`] can.
#[derive(Debug)]
#[repr(C)]
pub struct Entity {
    id: EntityId,
    _not_send_sync: PhantomData<*mut ()>,
}

impl Entity {
    pub(crate) fn new(id: EntityId) -> Self {
        Self {
            id,
            _not_send_sync: PhantomData,
        }
    }

    /// Gets a component of this entity. Returns
    /// `Err(MissingComponent)` if the entity does not have this component.
    ///
    /// # Examples
    /// ```no_run
    /// use quill::{Position, Entity};
    /// # let entity: Entity = unreachable!();
    /// let position = entity.get::<Position>().expect("entity has no position component");
    /// ```
    pub fn get<T: Component>(&self) -> Result<T, MissingComponent> {
        let host_component = T::host_component();
        unsafe {
            let bytes = quill_sys::entity_get_component(self.id.0, host_component)
                .ok_or_else(|| MissingComponent(std::any::type_name::<T>()))?;
            let bytes = std::slice::from_raw_parts(bytes.ptr() as *const u8, bytes.len() as usize);
            Ok(T::from_bytes_unchecked(bytes).0)
        }
    }

    /// Sets or replaces a component of this entity.
    ///
    /// If the entity already has this component,
    /// the component is overwritten.
    pub fn set<T: Component>(&self, component: T) {
        let host_component = T::host_component();
        let bytes = component.to_cow_bytes();

        unsafe {
            quill_sys::entity_set_component(
                self.id.0,
                host_component,
                bytes.as_ptr() as u32,
                bytes.len() as u32,
            );
        }
    }

    /// Sends the given message to this entity.
    ///
    /// The message sends as a "system" message.
    /// See [the wiki](https://wiki.vg/Chat) for more details.
    pub fn send_message(&self, message: impl AsRef<str>) {
        let message = message.as_ref();
        unsafe { quill_sys::entity_send_message(self.id.0, message.as_ptr(), message.len() as u32) }
    }

    /// Gets the unique ID of this entity.
    pub fn id(&self) -> EntityId {
        self.id
    }
}
