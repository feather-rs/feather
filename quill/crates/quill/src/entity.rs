use std::marker::PhantomData;

/// Unique internal ID of an entity.
///
/// Can be passed to [`Game::entity`] to get an [`Entity`]
/// handle.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(pub(crate) quill_common::EntityId);

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

    /// Sends the given message to this entity.
    ///
    /// The message sends as a "system" message.
    /// See [the wiki](https://wiki.vg/Chat) for more details.
    pub fn send_message(&self, message: impl AsRef<str>) {
        let message = message.as_ref();
        unsafe { quill_sys::entity_send_message(self.id.0, message.as_ptr(), message.len() as u32) }
    }
}
