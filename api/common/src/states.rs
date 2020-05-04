//! To implement the API, the host needs
//! to access its state in host calls.
//! To do this, it passes pointers to opaque
//! types which store some data it needs
//! to execute API requests.
//!
//! There are multiple state types used
//! at different times in a plugin's lifecycle.

/// The server's state at startup, which the plugin
/// can use for access to startup functionality such
/// as the system registry.
#[repr(C)]
pub struct StartupState {
    _private: [u8; 0],
}

/// The server's state during the execution of a synchronous
/// task on some region.
#[repr(C)]
pub struct GameState {
    _private: [u8; 0],
}

/// The server's state potentially outside of the context of a synchronous
/// region-local task.
///
/// Note that an `AsyncState` can be obtained through a `GameState`.
#[repr(C)]
pub struct AsyncState {
    _private: [u8; 0],
}
