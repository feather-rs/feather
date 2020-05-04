//! Defines types for registering systems and event handlers.

use crate::event::{EventId, OpaqueEvent};
use crate::states::GameState;

/// Specifies a description of a system.
#[repr(C)]
pub struct SystemSpec {
    /// A pointer to the function to run when the system is invoked.
    pub f: fn(*mut GameState),
}

/// Specifies a description of an event handler.
#[repr(C)]
pub struct HandlerSpec {
    /// The function to invoke when the handler runs.
    ///
    /// The function may assume the `OpaqueEvent` pointer
    /// points to the event type associated with `event`.
    pub f: fn(*mut GameState, *mut OpaqueEvent),
    /// The ID of the event type which is to be handled by `f`.
    pub event: EventId,
}
