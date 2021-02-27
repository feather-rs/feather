use bytemuck::{Pod, Zeroable};
/// Marker component for cat entities.
///
/// # Example
/// A system that queries for all cats:
/// ```no_run
/// use quill::{Game, Position, entities::Cat};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Cat)>() {
///         println!("Found a cat with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Cat;

pod_component_impl!(Cat);
