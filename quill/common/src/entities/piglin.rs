use bytemuck::{Pod, Zeroable};
/// Marker component for piglin entities.
///
/// # Example
/// A system that queries for all piglins:
/// ```no_run
/// use quill::{Game, Position, entities::Piglin};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Piglin)>() {
///         println!("Found a piglin with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Piglin;

pod_component_impl!(Piglin);
