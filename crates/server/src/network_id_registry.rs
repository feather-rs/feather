use std::sync::atomic::{AtomicI32, Ordering};

/// An entity's ID used by the protocol
/// in `entity_id` fields.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkId(pub i32);

impl NetworkId {
    /// Creates a new, unique network ID.
    pub(crate) fn new() -> Self {
        static NEXT: AtomicI32 = AtomicI32::new(0);
        // In theory, this can overflow if the server
        // creates 4 billion entities. The hope is that
        // old entities will have died out at that point.
        Self(NEXT.fetch_add(1, Ordering::SeqCst))
    }
}
