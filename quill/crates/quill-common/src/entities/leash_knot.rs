use bytemuck::{Pod, Zeroable};
/// Marker component for leash knot entities.
///
/// # Example
/// A system that queries for all leash knots:
/// ```no_run
/// use quill::{Game, Position, entities::LeashKnot};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &LeashKnot)>() {
///         println!("Found a leash knot with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct LeashKnot;

pod_component_impl!(LeashKnot);
