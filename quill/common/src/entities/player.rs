use bytemuck::{Pod, Zeroable};
/// Marker component for player entities.
///
/// # Example
/// A system that queries for all players:
/// ```no_run
/// use quill::{Game, Position, entities::Player};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Player)>() {
///         println!("Found a player with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Player;

pod_component_impl!(Player);
