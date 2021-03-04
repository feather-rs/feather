use bytemuck::{Pod, Zeroable};
/// Marker component for turtle entities.
///
/// # Example
/// A system that queries for all turtles:
/// ```no_run
/// use quill::{Game, Position, entities::Turtle};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Turtle)>() {
///         println!("Found a turtle with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Turtle;

pod_component_impl!(Turtle);
