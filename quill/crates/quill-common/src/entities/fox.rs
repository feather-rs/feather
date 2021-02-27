use bytemuck::{Pod, Zeroable};
/// Marker component for fox entities.
///
/// # Example
/// A system that queries for all foxs:
/// ```no_run
/// use quill::{Game, Position, entities::Fox};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Fox)>() {
///         println!("Found a fox with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Fox;

pod_component_impl!(Fox);
