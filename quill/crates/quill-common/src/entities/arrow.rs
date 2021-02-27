use bytemuck::{Pod, Zeroable};
/// Marker component for arrow entities.
///
/// # Example
/// A system that queries for all arrows:
/// ```no_run
/// use quill::{Game, Position, entities::Arrow};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Arrow)>() {
///         println!("Found a arrow with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Arrow;

pod_component_impl!(Arrow);
