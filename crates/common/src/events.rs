use crate::view::View;

/// Triggered when a player joins the `Game`.
#[derive(Debug)]
pub struct PlayerJoinEvent;

/// Event triggered when a player changes their `View`,
/// meaning they crossed into a new chunk.
#[derive(Debug)]
pub struct ViewUpdateEvent {
    pub old_view: View,
    pub new_view: View,
}
