use bytemuck::{Pod, Zeroable};
/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all mooshrooms:
/// ```no_run
/// use quill::{Game, Position, entities::Mooshroom};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Mooshroom)>() {
///         println!("Found a mooshroom with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Mooshroom;

pod_component_impl!(Mooshroom);
