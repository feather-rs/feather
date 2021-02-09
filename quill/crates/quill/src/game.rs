use std::marker::PhantomData;

use crate::query::{Query, QueryIter};
use crate::{Entity, EntityId};

/// Provides access to the server's game state for a single world. Includes
/// entities, blocks, chunks, etc. All interaction with
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
    pub fn entity(&self, id: EntityId) -> Option<Entity> {
        unsafe {
            if !quill_sys::entity_exists(id.0) {
                return None;
            }
        }
        Some(Entity::new(id))
    }

    pub fn query<Q: Query>(&self) -> QueryIter<Q> {
        QueryIter::new()
    }
}
