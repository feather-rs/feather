use bytemuck::{Pod, Zeroable};
/// Marker component for giant entities.
///
/// # Example
/// A system that queries for all giants:
/// ```no_run
/// use quill::{Game, Position, entities::Giant};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Giant)>() {
///         println!("Found a giant with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Giant;

pod_component_impl!(Giant);
