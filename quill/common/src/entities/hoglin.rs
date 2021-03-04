use bytemuck::{Pod, Zeroable};
/// Marker component for hoglin entities.
///
/// # Example
/// A system that queries for all hoglins:
/// ```no_run
/// use quill::{Game, Position, entities::Hoglin};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Hoglin)>() {
///         println!("Found a hoglin with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Hoglin;

pod_component_impl!(Hoglin);
