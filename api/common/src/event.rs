//! Types associated with events.

/// Unique ID of an event type.
/// These are allocated opaquely
/// by the host. No guarantees are made.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct EventId(pub u64);

/// Opaque type representing some avoid.
///
/// Pointers to this type may be casted
/// to a concrete type, given the user
/// knows the actual type of the event.
#[repr(C)]
pub struct OpaqueEvent {
    _private: [u8; 0],
}
